use wasm_bindgen::prelude::*;

use lfsr::Lfsr;

#[wasm_bindgen]
pub fn unroll_lfsr(data_size: usize, state_size: usize, polynomial: u64) -> String {
    let lfsr = Lfsr::new(data_size, state_size, polynomial);
    lfsr.to_string()
}
