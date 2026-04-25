/* tslint:disable */
/* eslint-disable */

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly main: (a: number, b: number) => number;
    readonly wasm_bindgen__convert__closures_____invoke__h70d1f424303d92e1: (a: number, b: number, c: any) => [number, number];
    readonly wasm_bindgen__convert__closures_____invoke__hd8b83ee41db76dae: (a: number, b: number, c: any, d: any) => void;
    readonly wasm_bindgen__convert__closures_____invoke__h0f6aabf1b5467bac: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen__convert__closures_____invoke__h02989e84463b0aa1: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen__convert__closures_____invoke__hca719ee392f099cc: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen__convert__closures_____invoke__h0a1aad1a62578dcb: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen__convert__closures_____invoke__h30fc388794ca14b0: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen__convert__closures_____invoke__h39f93277c01e7207: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen__convert__closures_____invoke__h026041428eef6c91: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen__convert__closures_____invoke__h00cf63e44b02acd9: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen__convert__closures_____invoke__h8485f4860200284c: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen__convert__closures_____invoke__h26584037928c4979: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen__convert__closures_____invoke__hedea87ca738f0610: (a: number, b: number) => number;
    readonly wasm_bindgen__convert__closures_____invoke__h584cfcdc69184f1d: (a: number, b: number) => void;
    readonly __wbindgen_malloc: (a: number, b: number) => number;
    readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
    readonly __externref_table_alloc: () => number;
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __wbindgen_exn_store: (a: number) => void;
    readonly __wbindgen_destroy_closure: (a: number, b: number) => void;
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
