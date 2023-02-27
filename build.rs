extern crate bindgen;

use std::env;
use std::path::PathBuf;

use vcpkg::Library;

const HARFBUZZ_PACKAGE_NAME: &'static str = "harfbuzz";

fn main() {
    println!("cargo:rerun-if-changed=wrapper.h");

    let harfbuzz_package = vcpkg::Config::new()
        .emit_includes(true)
        .find_package(HARFBUZZ_PACKAGE_NAME);

    match harfbuzz_package {
        Ok(lib) => generate_bindings(&lib),
        Err(e) => panic!("{}", e)
    };
}

fn generate_bindings(lib: &Library) {
    let mut bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks)); // Tell cargo to invalidate the built crate whenever any of the included header files changed.

    for include_path in &lib.include_paths {
        bindings = bindings
            .clang_arg("-I")
            .clang_arg(include_path.display().to_string());
    }

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
