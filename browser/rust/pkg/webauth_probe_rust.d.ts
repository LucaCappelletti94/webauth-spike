/* tslint:disable */
/* eslint-disable */

/**
 * Q4, Q6, Q8, Q9, Q10. One assertion scoped to a stored identifier, optionally
 * with a second input. Returns the outputs as hex plus the user-verification flag.
 */
export function assert(rp_id: string, id: Uint8Array, first: string, second: string | null | undefined, challenge: Uint8Array): Promise<any>;

/**
 * Q1: is the extension offered? getClientCapabilities has no typed binding in
 * web-sys 0.3.103, so the whole call goes through Reflect. That is a genuine
 * finding for this leg, recorded in the returned fallbacks array.
 */
export function capabilities(): Promise<any>;

/**
 * Q2 (enabled) and Q3 (output at creation). Returns rawId hex, enabled, and any
 * output present at creation time.
 */
export function create_credential(rp_id: string, first: string, challenge: Uint8Array, resident_required: boolean): Promise<any>;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly assert: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number) => any;
    readonly capabilities: () => any;
    readonly create_credential: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => any;
    readonly wasm_bindgen_c962ca86b39f545___convert__closures_____invoke___wasm_bindgen_c962ca86b39f545___JsValue__core_1cfd5997b9e55077___result__Result_____wasm_bindgen_c962ca86b39f545___JsError___true_: (a: number, b: number, c: any) => [number, number];
    readonly wasm_bindgen_c962ca86b39f545___convert__closures_____invoke___js_sys_183ffb55c64ab634___Function_fn_wasm_bindgen_c962ca86b39f545___JsValue_____wasm_bindgen_c962ca86b39f545___sys__Undefined___js_sys_183ffb55c64ab634___Function_fn_wasm_bindgen_c962ca86b39f545___JsValue_____wasm_bindgen_c962ca86b39f545___sys__Undefined_______true_: (a: number, b: number, c: any, d: any) => void;
    readonly __wbindgen_exn_store: (a: number) => void;
    readonly __externref_table_alloc: () => number;
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __wbindgen_destroy_closure: (a: number, b: number) => void;
    readonly __wbindgen_malloc: (a: number, b: number) => number;
    readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
    readonly __externref_table_dealloc: (a: number) => void;
    readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;

/**
 * Instantiates the given `module`, which can either be bytes or
 * a precompiled `WebAssembly.Module`.
 *
 * @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
 *
 * @returns {InitOutput}
 */
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
 * If `module_or_path` is {RequestInfo} or {URL}, makes a request and
 * for everything else, calls `WebAssembly.instantiate` directly.
 *
 * @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
 *
 * @returns {Promise<InitOutput>}
 */
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
