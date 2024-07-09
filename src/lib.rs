use std::mem::MaybeUninit;

use manifold::ManifoldManifold;
use wasm_bindgen::prelude::*;

#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
mod manifold {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

#[wasm_bindgen]
pub fn hello() {
    let mut cube: MaybeUninit<ManifoldManifold> = MaybeUninit::uninit();
    let cube_ref = unsafe {
        manifold::manifold_cube(
            cube.as_mut_ptr() as *mut std::os::raw::c_void,
            1.0,
            1.0,
            1.0,
            0,
        );
    };
}
