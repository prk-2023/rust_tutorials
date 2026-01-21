//This build script:
// 1. Compiles `mathlib.c` into libmathlib.o
// 2. Generated Rust bindings from `mathlib.h `
//
use std::env;
use std::path::PathBuf;

fn main() {
    // Compile the C code
    cc::Build::new().file("c_src/mathlib.c").compile("mathlib");

    // Tell cargo to invalidate the built crate whenever the header changes
    println!("cargo:rerun-if-changed=c_src/mathlib.h");

    // Generate bindings
    let bindings = bindgen::Builder::default()
        .header("c_src/mathlib.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    // Write bindings to $OUT_DIR/bindings.rs
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
