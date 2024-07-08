use std::{env, path::PathBuf};

use cmake::Config;

fn main() {
    let dst = Config::new("manifold")
        .define("MANIFOLD_TEST", "OFF")
        .build();

    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=libmanifold");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings
        .write_to_file(out_dir.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
