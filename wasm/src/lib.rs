use wasm_bindgen::prelude::*;

use lfsr::Lfsr;

#[wasm_bindgen]
pub fn unroll_lfsr(data_size: usize, state_size: usize, polynomial: u64, include_initial_state: bool) -> String {
    let lfsr = Lfsr::new(data_size, state_size, polynomial, include_initial_state);
    lfsr.to_string()
}
