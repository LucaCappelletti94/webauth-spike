/* @ts-self-types="./webauth_probe_rust.d.ts" */

/**
 * Q4, Q6, Q8, Q9, Q10. One assertion scoped to a stored identifier, optionally
 * with a second input. Returns the outputs as hex plus the user-verification flag.
 * @param {string} rp_id
 * @param {Uint8Array} id
 * @param {string} first
 * @param {string | null | undefined} second
 * @param {Uint8Array} challenge
 * @returns {Promise<any>}
 */
export function assert(rp_id, id, first, second, challenge) {
    const ptr0 = passStringToWasm0(rp_id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    const ptr1 = passArray8ToWasm0(id, wasm.__wbindgen_malloc);
    const len1 = WASM_VECTOR_LEN;
    const ptr2 = passStringToWasm0(first, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len2 = WASM_VECTOR_LEN;
    var ptr3 = isLikeNone(second) ? 0 : passStringToWasm0(second, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len3 = WASM_VECTOR_LEN;
    const ptr4 = passArray8ToWasm0(challenge, wasm.__wbindgen_malloc);
    const len4 = WASM_VECTOR_LEN;
    const ret = wasm.assert(ptr0, len0, ptr1, len1, ptr2, len2, ptr3, len3, ptr4, len4);
    return ret;
}

/**
 * Q1: is the extension offered? getClientCapabilities has no typed binding in
 * web-sys 0.3.103, so the whole call goes through Reflect. That is a genuine
 * finding for this leg, recorded in the returned fallbacks array.
 * @returns {Promise<any>}
 */
export function capabilities() {
    const ret = wasm.capabilities();
    return ret;
}

/**
 * Q2 (enabled) and Q3 (output at creation). Returns rawId hex, enabled, and any
 * output present at creation time.
 * @param {string} rp_id
 * @param {string} first
 * @param {Uint8Array} challenge
 * @param {boolean} resident_required
 * @returns {Promise<any>}
 */
export function create_credential(rp_id, first, challenge, resident_required) {
    const ptr0 = passStringToWasm0(rp_id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    const ptr1 = passStringToWasm0(first, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    const ptr2 = passArray8ToWasm0(challenge, wasm.__wbindgen_malloc);
    const len2 = WASM_VECTOR_LEN;
    const ret = wasm.create_credential(ptr0, len0, ptr1, len1, ptr2, len2, resident_required);
    return ret;
}
function __wbg_get_imports() {
    const import0 = {
        __proto__: null,
        __wbg___wbindgen_is_function_1ff95bcc5517c252: function(arg0) {
            const ret = typeof(arg0) === 'function';
            return ret;
        },
        __wbg___wbindgen_is_undefined_c05833b95a3cf397: function(arg0) {
            const ret = arg0 === undefined;
            return ret;
        },
        __wbg___wbindgen_throw_344f42d3211c4765: function(arg0, arg1) {
            throw new Error(getStringFromWasm0(arg0, arg1));
        },
        __wbg__wbg_cb_unref_fffb441def202758: function(arg0) {
            arg0._wbg_cb_unref();
        },
        __wbg_authenticatorData_5eb277bb689843ed: function(arg0) {
            const ret = arg0.authenticatorData;
            return ret;
        },
        __wbg_call_8a2dd23819f8a60a: function() { return handleError(function (arg0, arg1) {
            const ret = arg0.call(arg1);
            return ret;
        }, arguments); },
        __wbg_call_a6e5c5dce5018821: function() { return handleError(function (arg0, arg1, arg2) {
            const ret = arg0.call(arg1, arg2);
            return ret;
        }, arguments); },
        __wbg_create_0d8693b035703796: function() { return handleError(function (arg0, arg1) {
            const ret = arg0.create(arg1);
            return ret;
        }, arguments); },
        __wbg_credentials_2dd3ea53d7a0c758: function(arg0) {
            const ret = arg0.credentials;
            return ret;
        },
        __wbg_getClientExtensionResults_7c6894dc257f0279: function(arg0) {
            const ret = arg0.getClientExtensionResults();
            return ret;
        },
        __wbg_get_3d98f49bcbef8e58: function() { return handleError(function (arg0, arg1) {
            const ret = arg0.get(arg1);
            return ret;
        }, arguments); },
        __wbg_get_78f252d074a84d0b: function() { return handleError(function (arg0, arg1) {
            const ret = Reflect.get(arg0, arg1);
            return ret;
        }, arguments); },
        __wbg_get_enabled_6cd1ed9f764b23ea: function(arg0) {
            const ret = arg0.enabled;
            return isLikeNone(ret) ? 0xFFFFFF : ret ? 1 : 0;
        },
        __wbg_get_first_c4617cc4ae598d07: function(arg0) {
            const ret = arg0.first;
            return ret;
        },
        __wbg_get_index_e68b01fac18aa799: function(arg0, arg1) {
            const ret = arg0[arg1 >>> 0];
            return ret;
        },
        __wbg_get_prf_7a448d082c609d2e: function(arg0) {
            const ret = arg0.prf;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_get_results_e37674dc00a4d0e3: function(arg0) {
            const ret = arg0.results;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_get_second_0361b26904b21810: function(arg0) {
            const ret = arg0.second;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_instanceof_AuthenticatorAssertionResponse_718f2496ced6012a: function(arg0) {
            let result;
            try {
                result = arg0 instanceof AuthenticatorAssertionResponse;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        },
        __wbg_instanceof_Promise_4cb210c0b8f8c959: function(arg0) {
            let result;
            try {
                result = arg0 instanceof Promise;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        },
        __wbg_instanceof_PublicKeyCredential_9ef5f0c41955a61f: function(arg0) {
            let result;
            try {
                result = arg0 instanceof PublicKeyCredential;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        },
        __wbg_instanceof_Window_05ba1ee4f6781663: function(arg0) {
            let result;
            try {
                result = arg0 instanceof Window;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        },
        __wbg_length_1f0964f4a5e2c6d8: function(arg0) {
            const ret = arg0.length;
            return ret;
        },
        __wbg_navigator_99621db14b3f1099: function(arg0) {
            const ret = arg0.navigator;
            return ret;
        },
        __wbg_new_32b398fb48b6d94a: function() {
            const ret = new Array();
            return ret;
        },
        __wbg_new_cd45aabdf6073e84: function(arg0) {
            const ret = new Uint8Array(arg0);
            return ret;
        },
        __wbg_new_da52cf8fe3429cb2: function() {
            const ret = new Object();
            return ret;
        },
        __wbg_new_from_slice_77cdfb7977362f3c: function(arg0, arg1) {
            const ret = new Uint8Array(getArrayU8FromWasm0(arg0, arg1));
            return ret;
        },
        __wbg_new_typed_1824d93f294193e5: function(arg0, arg1) {
            try {
                var state0 = {a: arg0, b: arg1};
                var cb0 = (arg0, arg1) => {
                    const a = state0.a;
                    state0.a = 0;
                    try {
                        return wasm_bindgen_c962ca86b39f545___convert__closures_____invoke___js_sys_183ffb55c64ab634___Function_fn_wasm_bindgen_c962ca86b39f545___JsValue_____wasm_bindgen_c962ca86b39f545___sys__Undefined___js_sys_183ffb55c64ab634___Function_fn_wasm_bindgen_c962ca86b39f545___JsValue_____wasm_bindgen_c962ca86b39f545___sys__Undefined_______true_(a, state0.b, arg0, arg1);
                    } finally {
                        state0.a = a;
                    }
                };
                const ret = new Promise(cb0);
                return ret;
            } finally {
                state0.a = 0;
            }
        },
        __wbg_prototypesetcall_4770620bbe4688a0: function(arg0, arg1, arg2) {
            Uint8Array.prototype.set.call(getArrayU8FromWasm0(arg0, arg1), arg2);
        },
        __wbg_push_d2ae3af0c1217ae6: function(arg0, arg1) {
            const ret = arg0.push(arg1);
            return ret;
        },
        __wbg_queueMicrotask_0ab5b2d2393e99b9: function(arg0) {
            const ret = arg0.queueMicrotask;
            return ret;
        },
        __wbg_queueMicrotask_6a09b7bc46549209: function(arg0) {
            queueMicrotask(arg0);
        },
        __wbg_rawId_643a998adbf81918: function(arg0) {
            const ret = arg0.rawId;
            return ret;
        },
        __wbg_resolve_2191a4dfe481c25b: function(arg0) {
            const ret = Promise.resolve(arg0);
            return ret;
        },
        __wbg_response_f185c421101f2182: function(arg0) {
            const ret = arg0.response;
            return ret;
        },
        __wbg_set_8535240470bf2500: function() { return handleError(function (arg0, arg1, arg2) {
            const ret = Reflect.set(arg0, arg1, arg2);
            return ret;
        }, arguments); },
        __wbg_set_alg_6637ded9a5267455: function(arg0, arg1) {
            arg0.alg = arg1;
        },
        __wbg_set_allow_credentials_b2c4e62f4068b17c: function(arg0, arg1) {
            arg0.allowCredentials = arg1;
        },
        __wbg_set_authenticator_selection_578a65a78b8f6b25: function(arg0, arg1) {
            arg0.authenticatorSelection = arg1;
        },
        __wbg_set_challenge_u8_array_9bbf621935c1a0f5: function(arg0, arg1) {
            arg0.challenge = arg1;
        },
        __wbg_set_challenge_u8_array_d76071904f429cc3: function(arg0, arg1) {
            arg0.challenge = arg1;
        },
        __wbg_set_display_name_1be7e5a2dd6e79a2: function(arg0, arg1, arg2) {
            arg0.displayName = getStringFromWasm0(arg1, arg2);
        },
        __wbg_set_eval_716d355ecff3bded: function(arg0, arg1) {
            arg0.eval = arg1;
        },
        __wbg_set_extensions_3a10044b1c2ac3d6: function(arg0, arg1) {
            arg0.extensions = arg1;
        },
        __wbg_set_extensions_49e227bc159c9b4a: function(arg0, arg1) {
            arg0.extensions = arg1;
        },
        __wbg_set_first_u8_array_b6454ffd52165217: function(arg0, arg1) {
            arg0.first = arg1;
        },
        __wbg_set_id_0861f287ab944a34: function(arg0, arg1, arg2) {
            arg0.id = getStringFromWasm0(arg1, arg2);
        },
        __wbg_set_id_u8_array_2e3dceec3a094322: function(arg0, arg1) {
            arg0.id = arg1;
        },
        __wbg_set_id_u8_array_bc1303e95dfadffa: function(arg0, arg1) {
            arg0.id = arg1;
        },
        __wbg_set_name_adf694c4d65443c4: function(arg0, arg1, arg2) {
            arg0.name = getStringFromWasm0(arg1, arg2);
        },
        __wbg_set_name_f8d3c39bb10a6630: function(arg0, arg1, arg2) {
            arg0.name = getStringFromWasm0(arg1, arg2);
        },
        __wbg_set_prf_d0aa85b6b77b9189: function(arg0, arg1) {
            arg0.prf = arg1;
        },
        __wbg_set_pub_key_cred_params_66b5a5f5625a499f: function(arg0, arg1) {
            arg0.pubKeyCredParams = arg1;
        },
        __wbg_set_public_key_093274a9ec3ad901: function(arg0, arg1) {
            arg0.publicKey = arg1;
        },
        __wbg_set_public_key_6e370b79fea338ca: function(arg0, arg1) {
            arg0.publicKey = arg1;
        },
        __wbg_set_resident_key_4efc5de609034340: function(arg0, arg1, arg2) {
            arg0.residentKey = getStringFromWasm0(arg1, arg2);
        },
        __wbg_set_rp_d4dcf2c7142268ec: function(arg0, arg1) {
            arg0.rp = arg1;
        },
        __wbg_set_rp_id_23ff400d0cc8bc22: function(arg0, arg1, arg2) {
            arg0.rpId = getStringFromWasm0(arg1, arg2);
        },
        __wbg_set_second_u8_array_8f251718a554722c: function(arg0, arg1) {
            arg0.second = arg1;
        },
        __wbg_set_type_4498ec6e6db7b741: function(arg0, arg1) {
            arg0.type = __wbindgen_enum_PublicKeyCredentialType[arg1];
        },
        __wbg_set_type_d091be7518cbeaf7: function(arg0, arg1) {
            arg0.type = __wbindgen_enum_PublicKeyCredentialType[arg1];
        },
        __wbg_set_user_3f54d35aa8685591: function(arg0, arg1) {
            arg0.user = arg1;
        },
        __wbg_set_user_verification_33a0790327c25efa: function(arg0, arg1) {
            arg0.userVerification = __wbindgen_enum_UserVerificationRequirement[arg1];
        },
        __wbg_set_user_verification_3be3af92e830a8c8: function(arg0, arg1) {
            arg0.userVerification = __wbindgen_enum_UserVerificationRequirement[arg1];
        },
        __wbg_static_accessor_GLOBAL_4ef717fb391d88b7: function() {
            const ret = typeof global === 'undefined' ? null : global;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_static_accessor_GLOBAL_THIS_8d1badc68b5a74f4: function() {
            const ret = typeof globalThis === 'undefined' ? null : globalThis;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_static_accessor_SELF_146583524fe1469b: function() {
            const ret = typeof self === 'undefined' ? null : self;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_static_accessor_WINDOW_f2829a2234d7819e: function() {
            const ret = typeof window === 'undefined' ? null : window;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_then_16d107c451e9905d: function(arg0, arg1, arg2) {
            const ret = arg0.then(arg1, arg2);
            return ret;
        },
        __wbg_then_6ec10ae38b3e92f7: function(arg0, arg1) {
            const ret = arg0.then(arg1);
            return ret;
        },
        __wbindgen_cast_0000000000000001: function(arg0, arg1) {
            // Cast intrinsic for `Closure(Closure { owned: true, function: Function { arguments: [Externref], shim_idx: 28, ret: Result(Unit), inner_ret: Some(Result(Unit)) }, mutable: true }) -> Externref`.
            const ret = makeMutClosure(arg0, arg1, wasm_bindgen_c962ca86b39f545___convert__closures_____invoke___wasm_bindgen_c962ca86b39f545___JsValue__core_1cfd5997b9e55077___result__Result_____wasm_bindgen_c962ca86b39f545___JsError___true_);
            return ret;
        },
        __wbindgen_cast_0000000000000002: function(arg0) {
            // Cast intrinsic for `F64 -> Externref`.
            const ret = arg0;
            return ret;
        },
        __wbindgen_cast_0000000000000003: function(arg0, arg1) {
            // Cast intrinsic for `Ref(String) -> Externref`.
            const ret = getStringFromWasm0(arg0, arg1);
            return ret;
        },
        __wbindgen_init_externref_table: function() {
            const table = wasm.__wbindgen_externrefs;
            const offset = table.grow(4);
            table.set(0, undefined);
            table.set(offset + 0, undefined);
            table.set(offset + 1, null);
            table.set(offset + 2, true);
            table.set(offset + 3, false);
        },
    };
    return {
        __proto__: null,
        "./webauth_probe_rust_bg.js": import0,
    };
}

function wasm_bindgen_c962ca86b39f545___convert__closures_____invoke___wasm_bindgen_c962ca86b39f545___JsValue__core_1cfd5997b9e55077___result__Result_____wasm_bindgen_c962ca86b39f545___JsError___true_(arg0, arg1, arg2) {
    const ret = wasm.wasm_bindgen_c962ca86b39f545___convert__closures_____invoke___wasm_bindgen_c962ca86b39f545___JsValue__core_1cfd5997b9e55077___result__Result_____wasm_bindgen_c962ca86b39f545___JsError___true_(arg0, arg1, arg2);
    if (ret[1]) {
        throw takeFromExternrefTable0(ret[0]);
    }
}

function wasm_bindgen_c962ca86b39f545___convert__closures_____invoke___js_sys_183ffb55c64ab634___Function_fn_wasm_bindgen_c962ca86b39f545___JsValue_____wasm_bindgen_c962ca86b39f545___sys__Undefined___js_sys_183ffb55c64ab634___Function_fn_wasm_bindgen_c962ca86b39f545___JsValue_____wasm_bindgen_c962ca86b39f545___sys__Undefined_______true_(arg0, arg1, arg2, arg3) {
    wasm.wasm_bindgen_c962ca86b39f545___convert__closures_____invoke___js_sys_183ffb55c64ab634___Function_fn_wasm_bindgen_c962ca86b39f545___JsValue_____wasm_bindgen_c962ca86b39f545___sys__Undefined___js_sys_183ffb55c64ab634___Function_fn_wasm_bindgen_c962ca86b39f545___JsValue_____wasm_bindgen_c962ca86b39f545___sys__Undefined_______true_(arg0, arg1, arg2, arg3);
}


const __wbindgen_enum_PublicKeyCredentialType = ["public-key"];


const __wbindgen_enum_UserVerificationRequirement = ["required", "preferred", "discouraged"];

function addToExternrefTable0(obj) {
    const idx = wasm.__externref_table_alloc();
    wasm.__wbindgen_externrefs.set(idx, obj);
    return idx;
}

const CLOSURE_DTORS = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(state => wasm.__wbindgen_destroy_closure(state.a, state.b));

function getArrayU8FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getUint8ArrayMemory0().subarray(ptr / 1, ptr / 1 + len);
}

function getStringFromWasm0(ptr, len) {
    return decodeText(ptr >>> 0, len);
}

let cachedUint8ArrayMemory0 = null;
function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
}

function handleError(f, args) {
    try {
        return f.apply(this, args);
    } catch (e) {
        const idx = addToExternrefTable0(e);
        wasm.__wbindgen_exn_store(idx);
    }
}

function isLikeNone(x) {
    return x === undefined || x === null;
}

function makeMutClosure(arg0, arg1, f) {
    const state = { a: arg0, b: arg1, cnt: 1 };
    const real = (...args) => {

        // First up with a closure we increment the internal reference
        // count. This ensures that the Rust closure environment won't
        // be deallocated while we're invoking it.
        state.cnt++;
        const a = state.a;
        state.a = 0;
        try {
            return f(a, state.b, ...args);
        } finally {
            state.a = a;
            real._wbg_cb_unref();
        }
    };
    real._wbg_cb_unref = () => {
        if (--state.cnt === 0) {
            wasm.__wbindgen_destroy_closure(state.a, state.b);
            state.a = 0;
            CLOSURE_DTORS.unregister(state);
        }
    };
    CLOSURE_DTORS.register(real, state, state);
    return real;
}

function passArray8ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 1, 1) >>> 0;
    getUint8ArrayMemory0().set(arg, ptr / 1);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}

function passStringToWasm0(arg, malloc, realloc) {
    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length, 1) >>> 0;
        getUint8ArrayMemory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len, 1) >>> 0;

    const mem = getUint8ArrayMemory0();

    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }
    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, len = offset + arg.length * 3, 1) >>> 0;
        const view = getUint8ArrayMemory0().subarray(ptr + offset, ptr + len);
        const ret = cachedTextEncoder.encodeInto(arg, view);

        offset += ret.written;
        ptr = realloc(ptr, len, offset, 1) >>> 0;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

function takeFromExternrefTable0(idx) {
    const value = wasm.__wbindgen_externrefs.get(idx);
    wasm.__externref_table_dealloc(idx);
    return value;
}

let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
cachedTextDecoder.decode();
const MAX_SAFARI_DECODE_BYTES = 2146435072;
let numBytesDecoded = 0;
function decodeText(ptr, len) {
    numBytesDecoded += len;
    if (numBytesDecoded >= MAX_SAFARI_DECODE_BYTES) {
        cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
        cachedTextDecoder.decode();
        numBytesDecoded = len;
    }
    return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));
}

const cachedTextEncoder = new TextEncoder();

if (!('encodeInto' in cachedTextEncoder)) {
    cachedTextEncoder.encodeInto = function (arg, view) {
        const buf = cachedTextEncoder.encode(arg);
        view.set(buf);
        return {
            read: arg.length,
            written: buf.length
        };
    };
}

let WASM_VECTOR_LEN = 0;

let wasmModule, wasmInstance, wasm;
function __wbg_finalize_init(instance, module) {
    wasmInstance = instance;
    wasm = instance.exports;
    wasmModule = module;
    cachedUint8ArrayMemory0 = null;
    wasm.__wbindgen_start();
    return wasm;
}

async function __wbg_load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);
            } catch (e) {
                const validResponse = module.ok && expectedResponseType(module.type);

                if (validResponse && module.headers.get('Content-Type') !== 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve Wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else { throw e; }
            }
        }

        const bytes = await module.arrayBuffer();
        return await WebAssembly.instantiate(bytes, imports);
    } else {
        const instance = await WebAssembly.instantiate(module, imports);

        if (instance instanceof WebAssembly.Instance) {
            return { instance, module };
        } else {
            return instance;
        }
    }

    function expectedResponseType(type) {
        switch (type) {
            case 'basic': case 'cors': case 'default': return true;
        }
        return false;
    }
}

function initSync(module) {
    if (wasm !== undefined) return wasm;


    if (module !== undefined) {
        if (Object.getPrototypeOf(module) === Object.prototype) {
            ({module} = module)
        } else {
            console.warn('using deprecated parameters for `initSync()`; pass a single object instead')
        }
    }

    const imports = __wbg_get_imports();
    if (!(module instanceof WebAssembly.Module)) {
        module = new WebAssembly.Module(module);
    }
    const instance = new WebAssembly.Instance(module, imports);
    return __wbg_finalize_init(instance, module);
}

async function __wbg_init(module_or_path) {
    if (wasm !== undefined) return wasm;


    if (module_or_path !== undefined) {
        if (Object.getPrototypeOf(module_or_path) === Object.prototype) {
            ({module_or_path} = module_or_path)
        } else {
            console.warn('using deprecated parameters for the initialization function; pass a single object instead')
        }
    }

    if (module_or_path === undefined) {
        module_or_path = new URL('webauth_probe_rust_bg.wasm', import.meta.url);
    }
    const imports = __wbg_get_imports();

    if (typeof module_or_path === 'string' || (typeof Request === 'function' && module_or_path instanceof Request) || (typeof URL === 'function' && module_or_path instanceof URL)) {
        module_or_path = fetch(module_or_path);
    }

    const { instance, module } = await __wbg_load(await module_or_path, imports);

    return __wbg_finalize_init(instance, module);
}

export { initSync, __wbg_init as default };
