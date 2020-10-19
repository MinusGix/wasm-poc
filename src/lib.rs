use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn do_thing() -> i32 {
    let value: u128 = 0b10;
    let value: u128 = value.rotate_right(1);
    42
}
