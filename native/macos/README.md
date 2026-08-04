# macOS native probe

Two binaries, run on a Mac with Touch ID. They answer questions N1, N2, and N3 of the study: whether the operating system can hold a secret released only after the user proves themselves, and whether the existing library already delivers that.

```
cargo run --bin security_framework_probe   # N1, N2
cargo run --bin apple_keyring_probe         # N3
```

Each binary is interactive. It stores an item, pauses with a printed instruction of what to watch for, reads the item back (which triggers the prompt), and asks you to record what you saw. At the end it prints one report JSON object per the study report format.

`security_framework_probe` builds the access control by hand with `security_framework::passwords::AccessControlOptions`, combining `BIOMETRY_ANY`, `DEVICE_PASSCODE`, and `OR` so the item opens on either a fingerprint or the device passcode (N1). It then asks you to change the enrolled fingerprint set and reads again (N2). Biometry-any should still open, unlike biometry-current-set which invalidates on exactly that change.

`apple_keyring_probe` does the same round trip through `apple-native-keyring-store` 1.0.1 with `AccessPolicy::RequireUserPresence` (N3). In that crate `RequireUserPresence` maps to the `userPresence` access-control flag with `AccessibleWhenUnlocked` protection, which is a different construction from the biometry-any-plus-passcode gate in N1.

N1 against N3 is the whole upstream question. If `RequireUserPresence` prompts the same way, offers a passcode fallback, and survives a fingerprint change, nothing needs contributing. If it prompts differently, or invalidates on a fingerprint change, or offers no passcode fallback, the gap is precisely the biometry distinction and the combination with a passcode, and that is what a proposal to the library would ask for.

Note on building: these crates target Apple platforms through `core-foundation` and `security-framework`, so they compile only on macOS. The `protected` feature of `apple-native-keyring-store` is enabled in `Cargo.toml`, which is required for the protected data store.
