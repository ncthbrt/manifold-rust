use std::mem::MaybeUninit;

use manifoldc::{manifold_cylinder, manifold_num_vert, ManifoldManifold};
use wasm_bindgen::prelude::*;

#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(unused)]
mod manifoldc {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

// Next let's define a macro that's like `println!`, only it works for
// `console.log`. Note that `println!` doesn't actually work on the wasm target
// because the standard library currently just eats all output. To get
// `println!`-like behavior in your app you'll likely want a macro like this.

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen(start)]
pub fn main() {
    let mut cylinder: MaybeUninit<ManifoldManifold> = MaybeUninit::zeroed();
    unsafe {
        manifold_cylinder(
            cylinder.as_mut_ptr() as *mut std::os::raw::c_void,
            50.0,
            10.0,
            1.0,
            32,
            0,
        );
    };
    console_log!("Vert Count:  {}", unsafe {
        manifold_num_vert(cylinder.as_mut_ptr())
    })
}
