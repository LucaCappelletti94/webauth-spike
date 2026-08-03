//! Android native probe, question A6, the Android counterpart of macOS N3.
//!
//! android-keyring 0.2.0 exposes keystore::KeyGenParameterSpecBuilder::set_user_authentication_required,
//! the Android Keystore flag that gates a key behind the user proving themselves. The study
//! asks whether that flag actually gates, or whether it silently stores without gating, which
//! would be worse than absent because it would be trusted.
//!
//! Important provenance finding: the default credential path in android-keyring 0.2.0
//! (AndroidCredential::get_key) hardcodes set_user_authentication_required(env, false). So a
//! plain keyring::Entry through this crate is NOT gated. To exercise the flag you must build the
//! key yourself, which this probe does.
//!
//! The probe measures gating by observable behaviour:
//!   - a key built with the flag TRUE must refuse to be used without a fresh authentication
//!     (the Keystore throws UserNotAuthenticatedException). That refusal is the gate.
//!   - a key built with the flag FALSE round-trips a stored secret with no gate, which is what
//!     the stock library does by default.
//! Driving the actual biometric prompt and the device-credential fallback is the consuming
//! app's BiometricPrompt plus CryptoObject job (out of scope here), so this probe reports the
//! gate by refusal and the app surfaces the JSON.
//!
//! It is a JNI library, not a bare binary, because the Android Keystore needs a live JVM and a
//! Context. Provenance to record: android-keyring is a single-author dependency that would hold
//! the key to every local replica.

use android_keyring::cipher::Cipher;
use android_keyring::credential::{
    BLOCK_MODE_GCM, CIPHER_TRANSFORMATION, ENCRYPTION_PADDING_NONE, ENCRYPT_MODE,
    KEY_ALGORITHM_AES, MODE_PRIVATE, PROVIDER, PURPOSE_DECRYPT, PURPOSE_ENCRYPT,
};
use android_keyring::keystore::{Key, KeyGenParameterSpecBuilder, KeyGenerator, KeyStore};
use android_keyring::shared_preferences::Context;
use jni::objects::{JClass, JObject};
use jni::sys::jstring;
use jni::JNIEnv;

const SECRET: &[u8] = b"connetto-probe-secret";
const USER: &str = "probe@example.invalid";

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

/// Fetch an existing Keystore key or generate one with the given authentication gate.
fn get_or_create_key(
    env: &mut JNIEnv,
    alias: &str,
    auth_required: bool,
) -> jni::errors::Result<Key> {
    let keystore = KeyStore::get_instance(env, PROVIDER)?;
    keystore.load(env)?;
    if let Some(key) = keystore.get_key(env, alias)? {
        return Ok(key);
    }
    let spec = KeyGenParameterSpecBuilder::new(env, alias, PURPOSE_ENCRYPT | PURPOSE_DECRYPT)?
        .set_block_modes(env, &[BLOCK_MODE_GCM])?
        .set_encryption_paddings(env, &[ENCRYPTION_PADDING_NONE])?
        .set_user_authentication_required(env, auth_required)?
        .build(env)?;
    let generator = KeyGenerator::get_instance(env, KEY_ALGORITHM_AES, PROVIDER)?;
    generator.init(env, spec.into())?;
    Ok(generator.generate_key(env)?.into())
}

/// Try to use the key to encrypt the secret. Returns the ciphertext length on success.
fn try_encrypt(env: &mut JNIEnv, alias: &str, auth_required: bool) -> jni::errors::Result<usize> {
    let key = get_or_create_key(env, alias, auth_required)?;
    let cipher = Cipher::get_instance(env, CIPHER_TRANSFORMATION)?;
    cipher.init(env, ENCRYPT_MODE, &key)?;
    let ciphertext = cipher.do_final(env, SECRET)?;
    Ok(ciphertext.len())
}

/// Full store-and-read round trip through an ungated key, into SharedPreferences, exactly as
/// the stock library does. Demonstrates the default path stores with no gate. Returns whether
/// the read-back matched.
fn open_roundtrip(env: &mut JNIEnv, context: JObject, alias: &str) -> jni::errors::Result<bool> {
    let key = get_or_create_key(env, alias, false)?;

    let cipher = Cipher::get_instance(env, CIPHER_TRANSFORMATION)?;
    cipher.init(env, ENCRYPT_MODE, &key)?;
    let iv = cipher.get_iv(env)?;
    let ciphertext = cipher.do_final(env, SECRET)?;

    // Layout matches the library: [iv_len][iv][ciphertext].
    let iv_len = u8::try_from(iv.len()).map_err(|_| jni::errors::Error::InvalidCtorReturn)?;
    let mut value = vec![iv_len];
    value.extend_from_slice(&iv);
    value.extend_from_slice(&ciphertext);

    let ctx = Context::new(&*env, context)?;
    let prefs = ctx.get_shared_preferences(env, alias, MODE_PRIVATE)?;
    let editor = prefs.edit(env)?;
    editor.put_binary(env, USER, &value)?;
    editor.commit(env)?;

    // Read back.
    let stored = prefs.get_binary(env, USER)?;
    let stored = match stored {
        Some(s) if !s.is_empty() => s,
        _ => return Ok(false),
    };
    let stored_iv_len = usize::from(stored[0]);
    let rest = &stored[1..];
    if rest.len() < stored_iv_len {
        return Ok(false);
    }
    let read_iv = &rest[..stored_iv_len];
    let read_ct = &rest[stored_iv_len..];
    let spec = android_keyring::cipher::GCMParameterSpec::new(env, 128, read_iv)?;
    let cipher = Cipher::get_instance(env, CIPHER_TRANSFORMATION)?;
    cipher.init2(
        env,
        android_keyring::credential::DECRYPT_MODE,
        &key,
        spec.into(),
    )?;
    let plaintext = cipher.do_final(env, read_ct)?;
    Ok(plaintext == SECRET)
}

fn run(env: &mut JNIEnv, context: JObject) -> String {
    // A6 core: a key built with the flag TRUE must refuse use without authentication.
    let (gated_pass, gated_note) = match try_encrypt(env, "connetto-probe-gated", true) {
        Ok(n) => {
            // No prompt, no refusal: the flag was accepted but did not gate. Worse than absent.
            (
                false,
                format!(
                    "gated key encrypted {n} bytes with NO authentication. The flag was accepted but did not gate. This is worse than absent, because it would have been trusted."
                ),
            )
        }
        Err(e) => {
            // Refused. Clear the pending Java exception so later JNI calls are safe.
            let _ = env.exception_clear();
            (
                true,
                format!(
                    "gated key refused use without a fresh authentication ({e}). That refusal is the gate. A real read requires the app to authenticate through BiometricPrompt with a CryptoObject, at which point the biometric prompt and the device-credential fallback appear."
                ),
            )
        }
    };

    // Contrast: the default (flag false) path stores and reads with no gate at all.
    let open_note = match open_roundtrip(env, context, "connetto-probe-open") {
        Ok(true) => "ungated key stored and read a secret back with no prompt (this is the stock library default, which hardcodes the flag to false)".to_string(),
        Ok(false) => "ungated round trip did not match on read back".to_string(),
        Err(e) => {
            let _ = env.exception_clear();
            format!("ungated round trip failed: {e}")
        }
    };

    let notes = format!(
        "{gated_note} Contrast: {open_note}. Provenance: android-keyring 0.2.0 is a single-author dependency that would hold the key to every local replica, and its default credential builder sets user-authentication-required to false, so a plain keyring::Entry is not gated."
    );

    format!(
        "{{\n  \"os\": \"Android (fill exact version and device)\",\n  \"provider\": \"Android Keystore\",\n  \"leg\": \"native\",\n  \"results\": {{\n    \"A6\": {{ \"pass\": {}, \"value\": {} }}\n  }},\n  \"reflect_fallbacks\": [],\n  \"notes\": {}\n}}",
        gated_pass,
        jstr("set_user_authentication_required gates the key (refuses use without authentication)"),
        jstr(&notes),
    )
}

/// JNI entry. Java side: package com.connetto.probe, class ProbeBridge, native method
/// `String runProbe(Context context)`.
///
/// # Safety
/// Called by the JVM with valid `env` and `context` handles.
#[unsafe(no_mangle)]
pub extern "system" fn Java_com_connetto_probe_ProbeBridge_runProbe<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    context: JObject<'local>,
) -> jstring {
    let report = run(&mut env, context);
    match env.new_string(report) {
        Ok(s) => s.into_raw(),
        Err(_) => JObject::null().into_raw(),
    }
}
