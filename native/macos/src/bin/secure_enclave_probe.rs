//! macOS Secure Enclave probe, the raw-capability half of N1 reachable from a CLI.
//!
//! The keychain-item path (security_framework_probe, apple_keyring_probe) uses the data
//! protection keychain, which forces the restricted keychain-access-groups entitlement. On
//! macOS that entitlement must be authorized by an embedded provisioning profile, so a bare
//! signed CLI is killed at exec by AMFI (observed: rc=137 with a valid signature and the
//! entitlement embedded). Only a provisioned .app can use that path.
//!
//! This probe measures the same underlying capability, an operating system secret released
//! only after the user proves themselves, through the Secure Enclave instead of the keychain.
//! It generates an ephemeral Secure Enclave key gated by biometry-or-passcode and signs a
//! fixed challenge with it. Because the key is not persisted (no location is set), it needs no
//! keychain access group, so a plain signature (ad-hoc or Development) runs without the kill,
//! and the signing operation triggers Touch ID.
//!
//! Scope: this answers N1's gate question (does a prompt appear, is there a passcode fallback).
//! It does not answer N2 (survival of a fingerprint-set change) or N3 (the apple-native-keyring-store
//! comparison), because both need a persistent secret, which on macOS means the data protection
//! keychain and therefore a provisioned .app.
//!
//! Run on macOS from a Terminal (so Touch ID can present) after an ordinary signature:
//!   cargo build --bin secure_enclave_probe
//!   codesign --force --sign - target/debug/secure_enclave_probe
//!   ./target/debug/secure_enclave_probe

use std::io::{self, Write};

use security_framework::access_control::SecAccessControl;
use security_framework::key::{Algorithm, GenerateKeyOptions, KeyType, SecKey, Token};
use security_framework::passwords::AccessControlOptions;

const CHALLENGE: &[u8] = b"connetto-probe-challenge";

fn prompt(msg: &str) -> String {
    print!("{msg}");
    io::stdout().flush().ok();
    let mut line = String::new();
    io::stdin().read_line(&mut line).ok();
    line.trim().to_string()
}

fn ask_bool(msg: &str) -> bool {
    let a = prompt(&format!("{msg} [y/n]: ")).to_lowercase();
    a == "y" || a == "yes"
}

fn jstr(s: &str) -> String {
    let mut out = String::from("\"");
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

fn main() {
    println!("== macOS Secure Enclave probe (N1 gate) ==\n");

    // biometry-any OR device passcode, constraining private key usage. PRIVATE_KEY_USAGE is
    // required for a Secure Enclave signing key. The OR makes biometry and passcode alternatives.
    let flags = AccessControlOptions::PRIVATE_KEY_USAGE
        | AccessControlOptions::BIOMETRY_ANY
        | AccessControlOptions::DEVICE_PASSCODE
        | AccessControlOptions::OR;

    let access_control = match SecAccessControl::create_with_flags(flags.bits()) {
        Ok(ac) => ac,
        Err(e) => {
            println!("could not build access control: {e}");
            return;
        }
    };

    let mut opts = GenerateKeyOptions::default();
    opts.set_key_type(KeyType::ec());
    opts.set_size_in_bits(256);
    opts.set_token(Token::SecureEnclave);
    opts.set_access_control(access_control);
    // No location: the key is ephemeral, so no keychain access group is needed.

    let key = match SecKey::new(&opts) {
        Ok(k) => k,
        Err(e) => {
            println!("Secure Enclave key generation failed: {e}");
            println!("if this is an entitlement or -34018 error, this Mac's policy blocks even ephemeral");
            println!("Secure Enclave keys from this binary, and only a provisioned .app can measure the gate.");
            return;
        }
    };
    println!("generated an ephemeral Secure Enclave key gated by biometry-or-passcode.");
    println!("about to sign a fixed challenge. WATCH FOR: a Touch ID prompt, and try its passcode fallback.");
    prompt("press Enter to sign...");

    let mut prompt_seen = false;
    let mut passcode_ok = false;
    let mut signed = false;
    match key.create_signature(Algorithm::ECDSASignatureMessageX962SHA256, CHALLENGE) {
        Ok(sig) => {
            signed = !sig.is_empty();
            println!("signature produced, {} bytes.", sig.len());
            prompt_seen = ask_bool("did a Touch ID prompt appear");
            passcode_ok =
                ask_bool("did the passcode fallback work (answer n if you did not try it)");
        }
        Err(e) => println!("signing failed: {e}"),
    }

    let notes = prompt("\nfree-text notes (prompts seen, anything surprising): ");

    let pass = signed && prompt_seen;
    println!("\n== report JSON ==");
    println!("{{");
    println!("  \"os\": \"macOS (fill exact version)\",");
    println!("  \"provider\": \"macOS Secure Enclave, security-framework\",");
    println!("  \"leg\": \"native\",");
    println!("  \"results\": {{");
    println!(
        "    \"N1\": {{ \"pass\": {}, \"value\": {} }}",
        pass,
        jstr(&format!(
            "secure enclave key, biometry-or-passcode gate: touch id prompt={}, passcode fallback ok={}",
            prompt_seen, passcode_ok
        ))
    );
    println!("  }},");
    println!("  \"reflect_fallbacks\": [],");
    println!(
        "  \"notes\": {}",
        jstr(&format!(
            "Secure Enclave gate, ephemeral key. Does not cover N2 or N3, which need persistence and a provisioned .app. {notes}"
        ))
    );
    println!("}}");
}
