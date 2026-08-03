//! Windows native probe, questions W1 and W2.
//!
//! Windows Hello sits behind two interfaces that are easy to confuse, and only one
//! is a real gate against an attacker who holds the files offline.
//!
//! W1: UserConsentVerifier. It prompts and returns a result to the calling process.
//! That is a check our own code performs, which an attacker never runs. Recorded so
//! nobody later mistakes it for protection.
//! W2: KeyCredentialManager. It holds a key pair in hardware gated by the user, but
//! signs rather than encrypts, so deriving a key means deriving from a signature.
//! The decisive question is whether signing the same challenge returns byte-identical
//! output across separate invocations and across a reboot. Deterministic means it can
//! seed a key exactly as the browser extension does. Not deterministic means there is
//! no native gate on Windows. This mirrors the stability question Q5.
//!
//! Run on Windows: `cargo run`. For W2, run once, reboot, run again, and compare the hex.

use std::io::{self, Write};

use windows::core::HSTRING;
use windows::Security::Credentials::UI::{
    UserConsentVerificationResult, UserConsentVerifier, UserConsentVerifierAvailability,
};
use windows::Security::Credentials::{
    KeyCredentialCreationOption, KeyCredentialManager, KeyCredentialStatus,
};
use windows::Security::Cryptography::CryptographicBuffer;

const CRED_NAME: &str = "connetto-probe";
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

/// W1: prompts, returns a result to us. Worth nothing against an offline attacker.
fn run_w1() -> (bool, String) {
    match UserConsentVerifier::CheckAvailabilityAsync().and_then(|op| op.get()) {
        Ok(avail) => {
            if avail != UserConsentVerifierAvailability::Available {
                return (
                    false,
                    format!("verifier not available (availability={})", avail.0),
                );
            }
        }
        Err(e) => return (false, format!("availability check failed: {e}")),
    }
    println!("W1: a Windows Hello consent prompt should appear now.");
    let message = HSTRING::from("connetto probe: confirm your presence");
    match UserConsentVerifier::RequestVerificationAsync(&message).and_then(|op| op.get()) {
        Ok(result) => {
            let verified = result == UserConsentVerificationResult::Verified;
            (
                verified,
                format!(
                    "UserConsentVerifier returned verified={verified}. This is a check our own \
                     process performs, so an offline attacker who holds the files never runs it. \
                     Worth nothing as a gate."
                ),
            )
        }
        Err(e) => (false, format!("verification failed: {e}")),
    }
}

/// W2: sign a fixed challenge twice, hex both. Determinism across runs and reboots is
/// the property everything rests on.
fn run_w2() -> (bool, String) {
    let name = HSTRING::from(CRED_NAME);

    // Create, or open if it already exists from a previous run.
    let retrieval = match KeyCredentialManager::RequestCreateAsync(
        &name,
        KeyCredentialCreationOption::FailIfExists,
    )
    .and_then(|op| op.get())
    {
        Ok(r) => match r.Status() {
            Ok(KeyCredentialStatus::CredentialAlreadyExists) => {
                match KeyCredentialManager::OpenAsync(&name).and_then(|op| op.get()) {
                    Ok(r) => r,
                    Err(e) => return (false, format!("open existing credential failed: {e}")),
                }
            }
            Ok(KeyCredentialStatus::Success) => r,
            Ok(other) => return (false, format!("create returned status {}", other.0)),
            Err(e) => return (false, format!("reading create status failed: {e}")),
        },
        Err(e) => return (false, format!("create request failed: {e}")),
    };

    let credential = match retrieval.Credential() {
        Ok(c) => c,
        Err(e) => return (false, format!("no credential in retrieval result: {e}")),
    };

    let sign_once = |label: &str| -> Option<String> {
        let buffer = CryptographicBuffer::CreateFromByteArray(CHALLENGE).ok()?;
        println!("W2 {label}: a Windows Hello prompt should appear to authorize signing.");
        let op = credential.RequestSignAsync(&buffer).ok()?.get().ok()?;
        match op.Status() {
            Ok(KeyCredentialStatus::Success) => {
                let sig = op.Result().ok()?;
                let hex = CryptographicBuffer::EncodeToHexString(&sig).ok()?;
                Some(hex.to_string().to_lowercase())
            }
            Ok(other) => {
                println!("sign returned status {}", other.0);
                None
            }
            Err(e) => {
                println!("reading sign status failed: {e}");
                None
            }
        }
    };

    let first = sign_once("run 1");
    let second = sign_once("run 2");
    match (first, second) {
        (Some(a), Some(b)) => {
            let same = a == b;
            println!("\nsignature 1: {a}");
            println!("signature 2: {b}");
            println!("byte-identical within this run: {same}");
            println!(
                "NOW: reboot and run this binary again, then compare against these hex values."
            );
            (
                same,
                format!(
                    "within-run identical={same}, sig={a}. Reboot and rerun to confirm across a \
                     reboot. Deterministic means Windows has a native gate, non-deterministic means \
                     it does not."
                ),
            )
        }
        _ => (false, "signing did not complete".to_string()),
    }
}
fn main() {
    println!("== Windows Hello probe (W1, W2) ==\n");

    let (w1_pass, w1_note) = run_w1();
    println!("{w1_note}\n");

    let (w2_pass, w2_note) = run_w2();
    println!();

    let notes = prompt("free-text notes (prompts seen, anything surprising): ");

    println!("\n== report JSON ==");
    println!("{{");
    println!("  \"os\": \"Windows (fill exact version)\",");
    println!("  \"provider\": \"Windows Hello\",");
    println!("  \"leg\": \"native\",");
    println!("  \"results\": {{");
    println!(
        "    \"W1\": {{ \"pass\": {}, \"value\": {} }},",
        w1_pass,
        jstr(&w1_note)
    );
    println!(
        "    \"W2\": {{ \"pass\": {}, \"value\": {} }}",
        w2_pass,
        jstr(&w2_note)
    );
    println!("  }},");
    println!("  \"reflect_fallbacks\": [],");
    println!("  \"notes\": {}", jstr(&notes));
    println!("}}");
}
