extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {

    cc::Build::new()
    .include("c")
    .file("c/lib/Config/DEV_Config.c")
    .file("c/lib/e-Paper/EPD_7in5_V2.c")
    .include("c")
    .include("c/lib/Config")
    .include("c/lib/e-Paper")
    .compile("e-paper");


    // Tell cargo to look for shared libraries in the specified directory
    println!("cargo:rustc-link-search=c");
    println!("cargo:rustc-link-search=c/lib");

    // Tell cargo to tell rustc to link the system bzip2
    // shared library.
    println!("cargo:rustc-link-lib=bz2");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rerun-if-changed=c/lib/Config/DEV_config.c");
    println!("cargo:rerun-if-changed=c/lib/Config/DEV_config.h");
    println!("cargo:rerun-if-changed=c/lib/e-Paper/EPD_7in5_V2.c");
    println!("cargo:rerun-if-changed=c/lib/e-Paper/EPD_7in5_V2.h");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        .clang_arg("-Ic/lib")
        .clang_arg("-Ic/lib/Config")
        .clang_arg("-Ic/lib/e-Paper")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
