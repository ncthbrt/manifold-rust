use std::mem::MaybeUninit;

use manifoldc::{manifold_cylinder, manifold_num_vert, ManifoldManifold};

#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(unused)]
mod manifoldc {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

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
    println!("Vert Count:  {}", unsafe {
        manifold_num_vert(cylinder.as_mut_ptr())
    })
}
