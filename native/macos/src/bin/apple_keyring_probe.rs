//! macOS existing-library probe, question N3.
//!
//! Store a secret through apple-native-keyring-store 1.0.1 with
//! AccessPolicy::RequireUserPresence, then read it back and observe the prompt,
//! the passcode fallback, and survival of a fingerprint-set change.
//!
//! N1 against N3 is the whole upstream question. N1 (security_framework_probe) asks
//! for biometry-any combined with device passcode. This library maps RequireUserPresence
//! to the userPresence access-control flag with WhenUnlocked protection, which is a
//! different construction. If it prompts differently, or invalidates on a fingerprint
//! change, or offers no passcode fallback, that gap is what an upstream proposal asks for.
//!
//! Like the security-framework probe, this uses the data protection keychain and so needs a
//! code-signed binary carrying a keychain access group. Build, ad-hoc sign, run the signed
//! binary directly. See the README. The access group must match entitlements.plist.

use std::io::{self, Write};

use apple_native_keyring_store::protected::{AccessPolicy, Cred};

const SERVICE: &str = "connetto-probe";
const ACCOUNT: &str = "probe@example.invalid";
const SECRET: &[u8] = b"connetto-probe-secret";
// Team-prefixed keychain access group. Must match entitlements.plist and the signing team.
const ACCESS_GROUP: &str = "7W8527FJJE.com.connetto.probe";

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

fn print_signing_help() {
    println!();
    println!(
        "apple-native-keyring-store forces the data protection keychain, which needs a keychain"
    );
    println!("access group. That is a restricted entitlement, so an ad-hoc signature is killed at launch.");
    println!(
        "It requires signing with a real Apple Development identity whose team prefixes the group,"
    );
    println!("for example TEAMID.connetto.probe in both the code and entitlements.plist:");
    println!("  security find-identity -v -p codesigning   # find your identity and team id");
    println!("  cargo build --bin apple_keyring_probe");
    println!("  codesign --force --sign \"Apple Development: you (TEAMID)\" --entitlements entitlements.plist target/debug/apple_keyring_probe");
    println!("  ./target/debug/apple_keyring_probe");
}

fn main() {
    println!("== macOS apple-native-keyring-store probe (N3) ==\n");

    let entry = match Cred::build(
        SERVICE,
        ACCOUNT,
        AccessPolicy::RequireUserPresence,
        Some(ACCESS_GROUP.to_string()),
        false,
    ) {
        Ok(e) => e,
        Err(e) => {
            println!("could not build protected entry: {e}");
            return;
        }
    };

    // Clean slate.
    let _ = entry.delete_credential();

    let mut prompt_seen = false;
    let mut passcode_ok = false;
    let mut survived = false;
    let mut read_ok = false;

    match entry.set_secret(SECRET) {
        Ok(()) => {
            println!("stored a RequireUserPresence item.");
            println!("about to read it back. WATCH FOR: a biometric prompt, and whether a passcode fallback is offered.");
            prompt("press Enter to read the item...");
            match entry.get_secret() {
                Ok(bytes) => {
                    read_ok = bytes == SECRET;
                    println!("read returned {} bytes, matches: {read_ok}", bytes.len());
                    prompt_seen = ask_bool("did a biometric prompt appear");
                    passcode_ok = ask_bool(
                        "was a passcode fallback offered (answer n if none, or if you did not try it)",
                    );
                }
                Err(e) => println!("read failed: {e}"),
            }

            // Same fingerprint-set change as N2, through the library this time.
            println!("\n-- fingerprint-set change (as in N2) --");
            println!("change the enrolled fingerprint set in System Settings, then return here.");
            prompt("press Enter once the fingerprint set has changed...");
            match entry.get_secret() {
                Ok(bytes) => {
                    let ok = bytes == SECRET;
                    println!(
                        "read after fingerprint change returned {} bytes, matches: {ok}",
                        bytes.len()
                    );
                    survived =
                        ok && ask_bool("did the item still open after the fingerprint change");
                }
                Err(e) => println!("read after fingerprint change failed: {e} (did NOT survive)"),
            }
        }
        Err(e) => {
            println!("store failed: {e}");
            print_signing_help();
            println!(
                "\nskipping the read and the fingerprint-change step because nothing was stored."
            );
        }
    }

    let notes = prompt(
        "\nfree-text notes (does RequireUserPresence behave like N1 biometry-any+passcode, or differ): ",
    );

    let pass = read_ok && prompt_seen;
    println!("\n== report JSON ==");
    println!("{{");
    println!("  \"os\": \"macOS (fill exact version)\",");
    println!("  \"provider\": \"apple-native-keyring-store (protected)\",");
    println!("  \"leg\": \"native\",");
    println!("  \"results\": {{");
    println!(
        "    \"N3\": {{ \"pass\": {}, \"value\": {} }}",
        pass,
        jstr(&format!(
            "RequireUserPresence: prompt={}, passcode fallback={}, survives fingerprint change={}",
            prompt_seen, passcode_ok, survived
        ))
    );
    println!("  }},");
    println!("  \"reflect_fallbacks\": [],");
    println!("  \"notes\": {}", jstr(&notes));
    println!("}}");
}
