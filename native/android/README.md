# Android native probe

Answers question A6, the Android counterpart of the macOS N3 question: does the Android Keystore flag that gates a key behind the user proving themselves actually gate, or does it silently store without gating.

The probe is a Rust library (`rust/`) invoked from a minimal Android app (`app/`). It has to be a library rather than a bare binary because the Android Keystore needs a live Java virtual machine and a `Context`.

## What A6 measures

`android-keyring` 0.2.0 exposes `keystore::KeyGenParameterSpecBuilder::set_user_authentication_required`, the Keystore flag in question. The probe builds one key with the flag set to true and one with it set to false, then observes behaviour. A key built with the flag true must refuse to be used without a fresh authentication (the Keystore throws `UserNotAuthenticatedException`), and that refusal is the gate. A key built with the flag false round-trips a stored secret into `SharedPreferences` and back with no gate at all.

Driving the actual biometric prompt and its device-credential fallback is the consuming app's `BiometricPrompt` plus `CryptoObject` work, which is out of scope for this measurement. The probe reports the gate by refusal, and a real read past that refusal is what an app would wire up next.

Two findings are baked in. First, the default credential path in `android-keyring` 0.2.0 (`AndroidCredential::get_key`) hardcodes the flag to false, so a plain `keyring::Entry` through this crate is not gated. Getting a gate means building the key yourself, which this probe does. Second, provenance: `android-keyring` is a single-author dependency that would hold the key to every local replica, which is worth weighing before depending on it.

If the gated key refuses use as expected, Android needs no upstream work and is ahead of Apple, where the equivalent flag is missing from `keyring` entirely. If the gated key encrypts with no authentication, the flag is exposed but ineffective, which is worse than absent because it would have been trusted.

## Building

Build the Rust library for the device ABIs and drop each `.so` into the app. The simplest path is `cargo-ndk`:

```
cargo install cargo-ndk
cd rust
cargo ndk -t arm64-v8a -t x86_64 -o ../app/app/src/main/jniLibs build --release
```

That writes `libwebauth_probe_android.so` under `app/app/src/main/jniLibs/<abi>/`, where the app's `build.gradle.kts` already looks. The NDK linker setup (the `ANDROID_NDK_HOME` environment variable) is the operator's responsibility. Without `cargo-ndk`, build with `cargo build --release --target aarch64-linux-android` after configuring the NDK linker for that target, then copy the `.so` into `app/app/src/main/jniLibs/arm64-v8a/` by hand.

Then build and install the app with Gradle (Android SDK required), open it on the device, and press the button. The report JSON appears on screen.

## Browser rows

The Android browser rows A1 through A5 are covered by the shared browser probe pages, not by this app. Reach them from the device over `adb reverse tcp:8000 tcp:8000` and load `http://localhost:8000/browser/`. Do not use the emulator host alias: WebAuthn needs a trustworthy origin, and a raw address is refused in a way that looks like missing features rather than a refusal.
