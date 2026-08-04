# webauth-spike

A probe that measures whether a real authenticator returns pseudo-random function output for a WebAuthn credential, and whether an operating system can hold a secret released only after the user proves themselves. These facts are documented nowhere and can only be measured, which is why this runs before anything is built. The load-bearing question is stability: if the same credential and input do not return identical bytes across runs, the approach is dead, so that is checked first.

There are four things to run. Two are browser pages under `browser/`: one written in plain JavaScript, which is the control and establishes what the platform does, and one written in Rust and compiled to wasm, which establishes whether the design can drive the same flow through the `web-sys` bindings. Run the JavaScript leg first, because a single failing Rust attempt cannot tell a platform limitation from a binding one. The other two are native Rust probes under `native/`, one per desktop and mobile platform, that test the operating system's own user-gated key stores.

## Serving the browser pages

WebAuthn requires a trustworthy origin. Localhost qualifies, a `file://` URL and a raw IP address do not, and both fail in the misleading way of reporting features as absent rather than refused. Serve from the repository root:

```
./serve.sh          # or: python3 -m http.server 8000
```

Then open `http://localhost:8000/browser/` for the JavaScript leg and `http://localhost:8000/browser/rust/` for the Rust leg. Each page has a button per question, appends a result row per press, and emits the whole run as report JSON with its last button. Credentials are bound to the origin host, so ones created on localhost work only there.

For a phone, forward a port rather than using the machine's address. On Android, `adb reverse tcp:8000 tcp:8000` then load `http://localhost:8000/browser/` on the device. On the Android emulator do the same and do not use the emulator host alias. iPhones and iPads cannot reach the machine's localhost, so they need a real https host or a tunnel that presents one.

The Rust leg is already built into `browser/rust/pkg/`. To rebuild it after changing `browser/rust/src/lib.rs`, run `wasm-pack build --target web` from `browser/rust/` (the crate carries a cargo config that turns on the unstable-apis flag every part of the extension needs).

## The native probes

`native/macos/`, `native/windows/`, and `native/android/` each carry their own README with build and run instructions and an explanation of what their questions decide. They run on their own platform against a live prompt, so they cannot be exercised from Linux. The macOS and Windows probes are plain interactive binaries. The Android probe is a Rust library invoked from the minimal app in `native/android/app/`, because the Android Keystore needs a Java virtual machine and a `Context`.

## Recording results

`report/` holds the report format, a template covering the whole test matrix, and `check.py`, which validates a filled report mechanically. See `report/README.md`. The one field that matters most is which provider actually answered: on Chrome a chooser appears, and a row that does not say which option responded is uninterpretable.
