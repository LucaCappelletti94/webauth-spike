//! macOS raw-capability probe, questions N1 and N2.
//!
//! N1: store a keychain item whose access control is biometry-any combined with
//! device passcode, then read it back and observe whether a Touch ID prompt appears
//! and whether the passcode fallback works.
//! N2: after the operator changes the enrolled fingerprint set, read again. biometry-any
//! should still open, unlike biometry-current-set which invalidates on exactly that change.
//!
//! This stores into the ordinary login keychain, where kSecAttrAccessControl still applies
//! the biometric access control list. That path needs no code signing, so plain
//! `cargo run --bin security_framework_probe` works. The data protection keychain would gate
//! the same way but requires a signed binary with a keychain access group, which is a
//! restricted entitlement an ad-hoc signature cannot carry.
//!
//! Interactive: each read triggers a real prompt, so the binary pauses and records what
//! the operator saw.

use std::io::{self, Write};

use security_framework::passwords::{
    delete_generic_password, generic_password, set_generic_password_options, AccessControlOptions,
    PasswordOptions,
};

const SERVICE: &str = "connetto-probe";
const ACCOUNT: &str = "probe@example.invalid";
const SECRET: &[u8] = b"connetto-probe-secret";

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

fn store_protected() -> security_framework::base::Result<()> {
    // biometry-any OR device passcode. The OR flag makes the two alternatives rather
    // than both required, which is the biometry-plus-passcode gate N1 asks for.
    let flags = AccessControlOptions::BIOMETRY_ANY
        | AccessControlOptions::DEVICE_PASSCODE
        | AccessControlOptions::OR;
    let mut opts = PasswordOptions::new_generic_password(SERVICE, ACCOUNT);
    opts.set_access_control_options(flags);
    // Login keychain, not the data protection keychain: kSecAttrAccessControl still applies
    // the biometric access control list here, and this needs no entitlement.
    set_generic_password_options(SECRET, opts)
}

fn read_protected() -> security_framework::base::Result<Vec<u8>> {
    generic_password(PasswordOptions::new_generic_password(SERVICE, ACCOUNT))
}

fn main() {
    println!("== macOS security-framework probe (N1, N2) ==\n");

    // Clean slate so a stale item does not mask the store.
    let _ = delete_generic_password(SERVICE, ACCOUNT);

    let mut n1_pass = false;
    let mut n1_prompt_seen = false;
    let mut n1_passcode_ok = false;
    let mut n2_ran = false;
    let mut n2_pass = false;

    match store_protected() {
        Ok(()) => {
            println!("stored a biometry-any-or-passcode item.");
            println!("about to read it back. WATCH FOR: a Touch ID prompt, and try the passcode fallback link on it.");
            prompt("press Enter to read the item...");
            match read_protected() {
                Ok(bytes) => {
                    let ok = bytes == SECRET;
                    println!(
                        "read returned {} bytes, matches stored secret: {ok}",
                        bytes.len()
                    );
                    n1_prompt_seen = ask_bool("did a Touch ID prompt appear");
                    n1_passcode_ok =
                        ask_bool("did the passcode fallback work (answer n if you did not try it)");
                    n1_pass = ok && n1_prompt_seen;
                }
                Err(e) => println!("read failed: {e}"),
            }

            // N2 only makes sense once an item exists.
            println!("\n-- N2: change the enrolled fingerprint set --");
            println!("in System Settings, add or remove a fingerprint in Touch ID and Password, then return here.");
            prompt("press Enter once the fingerprint set has changed...");
            n2_ran = true;
            match read_protected() {
                Ok(bytes) => {
                    let ok = bytes == SECRET;
                    println!(
                        "read after fingerprint change returned {} bytes, matches: {ok}",
                        bytes.len()
                    );
                    let opened = ask_bool("did biometry-any still open the item (expected yes)");
                    n2_pass = ok && opened;
                }
                Err(e) => {
                    println!("read after fingerprint change failed: {e}");
                    println!("if this failed, biometry-any did NOT survive the change, which is the finding for N2.");
                }
            }
        }
        Err(e) => {
            println!("store failed: {e}");
            println!("skipping N1 read and N2 because nothing was stored.");
        }
    }

    let notes = prompt("\nfree-text notes (prompts seen, anything surprising): ");

    let n2_value = if n2_ran {
        "biometry-any survives fingerprint-set change"
    } else {
        "skipped, store failed"
    };

    println!("\n== report JSON ==");
    println!("{{");
    println!("  \"os\": \"macOS (fill exact version)\",");
    println!("  \"provider\": \"macOS login keychain, security-framework\",");
    println!("  \"leg\": \"native\",");
    println!("  \"results\": {{");
    println!(
        "    \"N1\": {{ \"pass\": {}, \"value\": {} }},",
        n1_pass,
        jstr(&format!(
            "biometry-any+passcode: touch id prompt={}, passcode fallback ok={}",
            n1_prompt_seen, n1_passcode_ok
        ))
    );
    println!(
        "    \"N2\": {{ \"pass\": {}, \"value\": {} }}",
        n2_pass,
        jstr(n2_value)
    );
    println!("  }},");
    println!("  \"reflect_fallbacks\": [],");
    println!("  \"notes\": {}", jstr(&notes));
    println!("}}");
}
