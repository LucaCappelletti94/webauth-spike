//! WebAuthn PRF probe, Rust leg. The same flow as the JavaScript control, driven
//! through web-sys 0.3.103, to establish whether the design can use the platform
//! through the typed bindings. Every place the typed bindings are insufficient and
//! a raw js_sys::Reflect fallback is required is recorded per call and surfaced to
//! the page, because a flow that only works by bypassing the bindings is a
//! different answer from one that works through them.
//!
//! Comparison bookkeeping (baselines for Q5/Q12/Q13, worker topology for Q11) lives
//! in the page glue, since that logic is language-neutral. This module owns the
//! WebAuthn create and assert calls, which are the part the binding layer must carry.

use js_sys::{Array, Function, Object, Promise, Reflect, Uint8Array};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{
    AuthenticationExtensionsClientInputs, AuthenticationExtensionsPrfInputs,
    AuthenticationExtensionsPrfValues, AuthenticatorAssertionResponse,
    AuthenticatorSelectionCriteria, CredentialCreationOptions, CredentialRequestOptions,
    CredentialsContainer, PublicKeyCredential, PublicKeyCredentialCreationOptions,
    PublicKeyCredentialDescriptor, PublicKeyCredentialParameters,
    PublicKeyCredentialRequestOptions, PublicKeyCredentialRpEntity, PublicKeyCredentialType,
    PublicKeyCredentialUserEntity, UserVerificationRequirement,
};

const RP_NAME: &str = "PRF probe";
const USER_NAME: &str = "probe@example.invalid";
const USER_DISPLAY: &str = "Probe User";
// user.id, the 16 bytes 00 01 .. 0f.
const USER_ID: [u8; 16] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];

fn creds() -> Result<CredentialsContainer, JsValue> {
    let win = web_sys::window().ok_or_else(|| JsValue::from_str("no window"))?;
    Ok(win.navigator().credentials())
}

fn hex(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        s.push_str(&format!("{b:02x}"));
    }
    s
}

fn hex_of(obj: &JsValue) -> String {
    hex(&Uint8Array::new(obj).to_vec())
}

fn set(obj: &Object, key: &str, val: &JsValue) {
    let _ = Reflect::set(obj, &JsValue::from_str(key), val);
}

fn pub_key_params() -> Array {
    let arr = Array::new();
    let es256 = PublicKeyCredentialParameters::new(-7, PublicKeyCredentialType::PublicKey);
    let rs256 = PublicKeyCredentialParameters::new(-257, PublicKeyCredentialType::PublicKey);
    arr.push(es256.as_ref());
    arr.push(rs256.as_ref());
    arr
}

fn prf_inputs(first: &str, second: Option<&str>) -> AuthenticationExtensionsClientInputs {
    // The typed builder is present and sufficient here: new_with_u8_array on the
    // values dict, set_second_u8_array for the rotation input, set_eval on the
    // inputs dict, and the gated set_prf on the client inputs.
    let values =
        AuthenticationExtensionsPrfValues::new_with_u8_array(&Uint8Array::from(first.as_bytes()));
    if let Some(second) = second {
        values.set_second_u8_array(&Uint8Array::from(second.as_bytes()));
    }
    let inputs = AuthenticationExtensionsPrfInputs::new();
    inputs.set_eval(&values);
    let client = AuthenticationExtensionsClientInputs::new();
    client.set_prf(&inputs);
    client
}

/// Q1: is the extension offered? getClientCapabilities has no typed binding in
/// web-sys 0.3.103, so the whole call goes through Reflect. That is a genuine
/// finding for this leg, recorded in the returned fallbacks array.
#[wasm_bindgen]
pub async fn capabilities() -> Result<JsValue, JsValue> {
    let out = Object::new();
    let fallbacks = Array::new();

    let win = web_sys::window().ok_or_else(|| JsValue::from_str("no window"))?;
    let pkc = Reflect::get(win.as_ref(), &JsValue::from_str("PublicKeyCredential"))?;
    if pkc.is_undefined() {
        set(&out, "absent", &JsValue::TRUE);
        set(&out, "prf", &JsValue::NULL);
        set(&out, "fallbacks", fallbacks.as_ref());
        return Ok(out.into());
    }
    let getter = Reflect::get(&pkc, &JsValue::from_str("getClientCapabilities"))?;
    if !getter.is_function() {
        set(&out, "absent", &JsValue::TRUE);
        set(&out, "prf", &JsValue::NULL);
        fallbacks.push(&JsValue::from_str(
            "Q1 getClientCapabilities absent in bindings and on this client (expected below Chrome 133)",
        ));
        set(&out, "fallbacks", fallbacks.as_ref());
        return Ok(out.into());
    }
    fallbacks.push(&JsValue::from_str(
        "Q1 getClientCapabilities: no typed binding in web-sys 0.3.103, called via Reflect",
    ));
    let func: Function = getter.dyn_into()?;
    let promise: Promise = func.call0(&pkc)?.dyn_into()?;
    let caps = JsFuture::from(promise).await?;
    let prf = Reflect::get(&caps, &JsValue::from_str("extension:prf"))?;
    set(&out, "absent", &JsValue::FALSE);
    set(&out, "prf", &prf);
    set(&out, "fallbacks", fallbacks.as_ref());
    Ok(out.into())
}

/// Q2 (enabled) and Q3 (output at creation). Returns rawId hex, enabled, and any
/// output present at creation time.
#[wasm_bindgen]
pub async fn create_credential(
    rp_id: &str,
    first: &str,
    challenge: &[u8],
    resident_required: bool,
) -> Result<JsValue, JsValue> {
    let out = Object::new();
    let fallbacks = Array::new();

    let rp = PublicKeyCredentialRpEntity::new(RP_NAME);
    rp.set_id(rp_id);
    let user = PublicKeyCredentialUserEntity::new_with_u8_array(
        USER_NAME,
        USER_DISPLAY,
        &Uint8Array::from(&USER_ID[..]),
    );
    let opts = PublicKeyCredentialCreationOptions::new_with_u8_array(
        &Uint8Array::from(challenge),
        pub_key_params().as_ref(),
        &rp,
        &user,
    );
    let selection = AuthenticatorSelectionCriteria::new();
    // residentKey defaults to discouraged (Q10). On Android, PRF requires a discoverable
    // resident passkey, so the caller can request required.
    selection.set_resident_key(if resident_required { "required" } else { "discouraged" });
    selection.set_user_verification(UserVerificationRequirement::Required);
    opts.set_authenticator_selection(&selection);
    opts.set_extensions(&prf_inputs(first, None));

    let cc = CredentialCreationOptions::new();
    cc.set_public_key(&opts);
    let cred: PublicKeyCredential = JsFuture::from(creds()?.create_with_options(&cc)?)
        .await?
        .dyn_into()?;

    set(
        &out,
        "rawIdHex",
        &JsValue::from_str(&hex_of(cred.raw_id().as_ref())),
    );

    let ext = cred.get_client_extension_results();
    match ext.get_prf() {
        Some(prf) => {
            match prf.get_enabled() {
                Some(v) => set(&out, "enabled", &JsValue::from_bool(v)),
                None => set(&out, "enabled", &JsValue::NULL),
            }
            match prf.get_results() {
                Some(results) => set(
                    &out,
                    "resultsAtCreationHex",
                    &JsValue::from_str(&hex_of(results.get_first().as_ref())),
                ),
                None => set(&out, "resultsAtCreationHex", &JsValue::NULL),
            }
        }
        None => {
            set(&out, "enabled", &JsValue::NULL);
            set(&out, "resultsAtCreationHex", &JsValue::NULL);
        }
    }
    set(&out, "fallbacks", fallbacks.as_ref());
    Ok(out.into())
}

/// Q4, Q6, Q8, Q9, Q10. One assertion scoped to a stored identifier, optionally
/// with a second input. Returns the outputs as hex plus the user-verification flag.
#[wasm_bindgen]
pub async fn assert(
    rp_id: &str,
    id: &[u8],
    first: &str,
    second: Option<String>,
    challenge: &[u8],
) -> Result<JsValue, JsValue> {
    let out = Object::new();
    let fallbacks = Array::new();

    let opts = PublicKeyCredentialRequestOptions::new_with_u8_array(&Uint8Array::from(challenge));
    opts.set_rp_id(rp_id);
    opts.set_user_verification(UserVerificationRequirement::Required);
    let descriptor = PublicKeyCredentialDescriptor::new_with_u8_array(
        &Uint8Array::from(id),
        PublicKeyCredentialType::PublicKey,
    );
    let allow = Array::new();
    allow.push(descriptor.as_ref());
    opts.set_allow_credentials(allow.as_ref());
    opts.set_extensions(&prf_inputs(first, second.as_deref()));

    let rc = CredentialRequestOptions::new();
    rc.set_public_key(&opts);
    let assertion: PublicKeyCredential = JsFuture::from(creds()?.get_with_options(&rc)?)
        .await?
        .dyn_into()?;

    let ext = assertion.get_client_extension_results();
    let prf = ext
        .get_prf()
        .ok_or_else(|| JsValue::from_str("no prf in extension results"))?;
    let results = prf
        .get_results()
        .ok_or_else(|| JsValue::from_str("no prf.results in assertion"))?;

    let first_u8 = Uint8Array::new(results.get_first().as_ref());
    let first_bytes = first_u8.to_vec();
    set(&out, "firstHex", &JsValue::from_str(&hex(&first_bytes)));
    // length() is u32, widened to f64 for JS. Lossless: a PRF output is 32 bytes.
    set(
        &out,
        "firstLen",
        &JsValue::from_f64(f64::from(first_u8.length())),
    );
    match results.get_second() {
        Some(second_obj) => set(
            &out,
            "secondHex",
            &JsValue::from_str(&hex_of(second_obj.as_ref())),
        ),
        None => set(&out, "secondHex", &JsValue::NULL),
    }

    // Q9: user-verification flag, bit 2 (0x04) of byte 32 of authenticatorData.
    let resp: AuthenticatorAssertionResponse = assertion.response().dyn_into()?;
    let auth_data = Uint8Array::new(resp.authenticator_data().as_ref());
    let flags = auth_data.get_index(32); // u8, no cast
    set(&out, "uv", &JsValue::from_bool((flags & 0x04) != 0));
    set(&out, "flagsByte", &JsValue::from_f64(f64::from(flags)));

    set(&out, "fallbacks", fallbacks.as_ref());
    Ok(out.into())
}
