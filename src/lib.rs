use wasm_bindgen::prelude::*;

#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[wasm_bindgen]
pub fn hello() {
    // let manifold_box = manifold_box(mem, 0, 0, 1, 1, -1, -1)
}
