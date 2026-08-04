# Windows native probe

One binary, run on a Windows machine with Windows Hello configured. It answers questions W1 and W2 of the study.

```
cargo run
```

Windows is not symmetric with macOS. Credential Manager has no user-verification attribute, and Windows Hello exposes two interfaces that are easy to confuse. Only one is a real gate against an attacker who holds the files offline.

W1 exercises `UserConsentVerifier`. It prompts and returns a result to the calling process. The binary records that it works and states plainly what it is: a check our own code performs, which an attacker never runs. Against an offline attacker it is worth nothing. It is measured only so nobody later mistakes it for protection.

W2 exercises `KeyCredentialManager`, which is decisive. It holds a key pair in hardware gated by the user, but it signs rather than encrypts, so deriving a key means deriving from a signature. The binary signs one fixed challenge twice in a single run and prints both signatures as lowercase hex. The property everything rests on is determinism: reboot the machine, run the binary again, and compare the hex against the earlier run. Byte-identical output across separate invocations and across a reboot means the signature can seed a key exactly as the browser extension does. Different output means there is no native gate on Windows, and it joins the platforms that cannot be protected. This mirrors the stability question Q5, and a negative here ends the approach on this platform.

W3, browser rows on the same machine (Windows Hello as a platform authenticator in Chrome and Edge, plus a synced provider if offered), is covered by the shared browser probe pages, not by this binary.

Note on building: the `windows` crate targets Windows only, so this compiles there. The `IAsyncOperation` results are awaited with the blocking `.get()` so the probe stays a plain synchronous binary.
