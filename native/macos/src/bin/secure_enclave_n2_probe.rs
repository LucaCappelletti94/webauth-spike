//! macOS Secure Enclave probe, question N2, reachable from a CLI.
//!
//! N2 asks whether a biometry-any gate survives a change to the enrolled fingerprint set,
//! unlike biometry-current-set which invalidates on exactly that change. Decides whether the
//! native side needs a recovery story.
//!
//! A persistent secret would be the natural way to test this, but on macOS a persistent
//! Secure Enclave key must live in the data protection keychain (Apple and the security-framework
//! docs both state this), which needs the restricted keychain-access-groups entitlement and so a
//! provisioned .app. A bare CLI cannot persist an SE key. So this tests the same property without
//! persistence: it holds two ephemeral SE key handles across a real fingerprint-set change in one
//! run, a biometry-any key and a biometry-current-set key, and signs with each before and after.
//! biometry-any should still sign after the change. biometry-current-set is the contrast.
//!
//! Each signature triggers Touch ID, so authenticate when prompted. Run from a Terminal on the
//! Mac after an ordinary ad-hoc signature:
//!   cargo build --bin secure_enclave_n2_probe
//!   codesign --force --sign - target/debug/secure_enclave_n2_probe
//!   ./target/debug/secure_enclave_n2_probe

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

fn make_key(flags: AccessControlOptions) -> Result<SecKey, String> {
    let access_control =
        SecAccessControl::create_with_flags(flags.bits()).map_err(|e| e.to_string())?;
    let mut opts = GenerateKeyOptions::default();
    opts.set_key_type(KeyType::ec());
    opts.set_size_in_bits(256);
    opts.set_token(Token::SecureEnclave);
    opts.set_access_control(access_control);
    // No location: ephemeral, so no keychain access group and no AMFI kill.
    SecKey::new(&opts).map_err(|e| e.to_string())
}

fn sign(key: &SecKey) -> Result<usize, String> {
    key.create_signature(Algorithm::ECDSASignatureMessageX962SHA256, CHALLENGE)
        .map(|s| s.len())
        .map_err(|e| e.to_string())
}

fn main() {
    println!("== macOS Secure Enclave probe (N2) ==\n");

    let any_flags = AccessControlOptions::PRIVATE_KEY_USAGE
        | AccessControlOptions::BIOMETRY_ANY
        | AccessControlOptions::DEVICE_PASSCODE
        | AccessControlOptions::OR;
    let current_flags =
        AccessControlOptions::PRIVATE_KEY_USAGE | AccessControlOptions::BIOMETRY_CURRENT_SET;

    let any_key = match make_key(any_flags) {
        Ok(k) => k,
        Err(e) => {
            println!("biometry-any key generation failed: {e}");
            return;
        }
    };
    let current_key = match make_key(current_flags) {
        Ok(k) => k,
        Err(e) => {
            println!("biometry-current-set key generation failed: {e}");
            return;
        }
    };
    println!("generated a biometry-any key and a biometry-current-set key.");

    println!("\n-- baseline: sign with each before any change (authenticate at each prompt) --");
    let any_before = sign(&any_key);
    println!("biometry-any baseline sign: {any_before:?}");
    let current_before = sign(&current_key);
    println!("biometry-current-set baseline sign: {current_before:?}");

    println!("\n-- change the enrolled fingerprint set --");
    println!("in System Settings, add or remove a fingerprint in Touch ID and Password, then return here.");
    prompt("press Enter once the fingerprint set has changed...");

    println!("\n-- after change: sign with each again --");
    let any_after = sign(&any_key);
    println!("biometry-any sign after change: {any_after:?}");
    let current_after = sign(&current_key);
    println!("biometry-current-set sign after change: {current_after:?}");

    // N2 pass: biometry-any still signs after the fingerprint set changed.
    let n2_pass = any_before.is_ok() && any_after.is_ok();
    let current_invalidated = current_before.is_ok() && current_after.is_err();

    let notes = prompt("\nfree-text notes (prompts seen, did current-set stop working): ");

    println!("\n== report JSON ==");
    println!("{{");
    println!("  \"os\": \"macOS (fill exact version)\",");
    println!("  \"provider\": \"macOS Secure Enclave, security-framework\",");
    println!("  \"leg\": \"native\",");
    println!("  \"results\": {{");
    println!(
        "    \"N2\": {{ \"pass\": {}, \"value\": {} }}",
        n2_pass,
        jstr(&format!(
            "biometry-any survives fingerprint change: before={}, after={}. current-set contrast invalidated={} (before={}, after={})",
            any_before.is_ok(),
            any_after.is_ok(),
            current_invalidated,
            current_before.is_ok(),
            current_after.is_ok(),
        ))
    );
    println!("  }},");
    println!("  \"reflect_fallbacks\": [],");
    println!(
        "  \"notes\": {}",
        jstr(&format!(
            "Secure Enclave, ephemeral keys held across the change in one run (persistent SE keys need the data protection keychain and a provisioned app). {notes}"
        ))
    );
    println!("}}");
}
