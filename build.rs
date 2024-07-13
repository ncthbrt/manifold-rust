use std::{env, path::PathBuf};

use cmake::Config;

fn main() {
    let dst = cmake::Config::new("manifold")
        .define("MANIFOLD_TEST", "OFF")
        .define("BUILD_SHARED_LIBS", "OFF")
        .target("x86_64-unknown_linux-gnu")
        .cflag("-target wasm32")
        .cxxflag("-target wasm32")
        .cxxflag("-stdlib=libc++")
        .cflag("-stdlib=libc")
        .build();

    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-lib=dylib=manifoldc");

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
