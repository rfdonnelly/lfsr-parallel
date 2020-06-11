/* tslint:disable */
/* eslint-disable */
/**
* @param {number} data_size 
* @param {number} state_size 
* @param {BigInt} polynomial 
* @param {boolean} initial_state 
* @returns {string} 
*/
export function unroll_lfsr(data_size: number, state_size: number, polynomial: BigInt, initial_state: boolean): string;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly unroll_lfsr: (a: number, b: number, c: number, d: number, e: number, f: number) => void;
  readonly __wbindgen_free: (a: number, b: number) => void;
}

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
        