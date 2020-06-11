use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn unroll_lfsr(
    data_size: usize,
    state_size: usize,
    polynomial: u64,
) -> String {
    let state = lfsr::unroll_lfsr(data_size, state_size, polynomial);
    lfsr::state_to_s(&state)
}
