"use strict";

// Shared worker for Q11. Two jobs.
//   type "create":        attempt WebAuthn creation here, which must fail because
//                         PublicKeyCredential is [Exposed=Window] and absent in workers.
//   type "deriveWithKey": receive a structured-cloned CryptoKey, derive with it,
//                         and confirm exporting a non-extractable key fails here too.

const toHex = (buf) => [...new Uint8Array(buf)].map(b => b.toString(16).padStart(2, "0")).join("");

self.onmessage = async (ev) => {
  const msg = ev.data;

  if (msg.type === "create") {
    // Report the constraint by measurement rather than by assumption.
    if (typeof PublicKeyCredential === "undefined") {
      postMessage({ created: false, detail: "PublicKeyCredential is undefined in the worker (constraint confirmed)" });
      return;
    }
    if (!self.navigator || !self.navigator.credentials) {
      postMessage({ created: false, detail: "navigator.credentials absent in the worker (constraint confirmed)" });
      return;
    }
    try {
      await self.navigator.credentials.create({ publicKey: {
        challenge: crypto.getRandomValues(new Uint8Array(32)),
        rp: { id: msg.options.rpId, name: msg.options.name },
        user: { id: new Uint8Array(msg.options.userId), name: msg.options.userName, displayName: msg.options.userDisplay },
        pubKeyCredParams: msg.options.params,
        extensions: { prf: { eval: { first: new TextEncoder().encode(msg.options.first) } } },
      }});
      // If this ever succeeds the constraint would be broken, which is itself the finding.
      postMessage({ created: true, detail: "creation unexpectedly succeeded in a worker" });
    } catch (e) {
      postMessage({ created: false, detail: "creation threw in the worker: " + e });
    }
    return;
  }

  if (msg.type === "deriveWithKey") {
    const key = msg.key; // structured-cloned CryptoKey
    const out = { transferred: false, derived: false, workerExportFailed: false, bitsHex: null };
    try {
      out.transferred = (key && typeof key === "object" && key.type === "secret");
      const bits = await crypto.subtle.deriveBits(
        { name: "HKDF", hash: "SHA-256", salt: new Uint8Array(16), info: new TextEncoder().encode("connetto-probe") },
        key, 256);
      out.derived = true;
      out.bitsHex = toHex(bits);
    } catch (e) {
      out.derived = false;
      out.bitsHex = "derive error: " + e;
    }
    try {
      await crypto.subtle.exportKey("raw", key);
      out.workerExportFailed = false; // exportable, which would be a finding
    } catch (e) {
      out.workerExportFailed = true;  // expected: non-extractable
    }
    postMessage(out);
    return;
  }

  postMessage({ error: "unknown message type " + msg.type });
};
