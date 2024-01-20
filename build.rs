extern crate bindgen;
extern crate cmake;

use std::env;
use std::path::PathBuf;

fn main() {
    // Tell cargo to build submodule GLFW project.
    let dst = cmake::Config::new("glfw")
        .define("GLFW_BUILD_DOCS", "OFF")
        .define("GLFW_BUILD_EXAMPLES", "OFF")
        .define("GLFW_BUILD_TESTS", "OFF")
        .define(
            "GLFW_USE_WAYLAND",
            if cfg!(feature = "wayland") {
                "ON"
            } else {
                "OFF"
            },
        )
        .build();

    // Tell cargo to look for native libraries in the output directory of CMake.
    println!(
        "cargo:rustc-link-search=native={}",
        dst.join("lib").display()
    );

    // Tell cargo to tell rustc to link our custom libraries.
    // Cargo will automatically know it must look for a `lib<name>.a` file.
    println!("cargo:rustc-link-lib=static=glfw3");

    // This is the path to the C header file with the exports.
    let headers_path = PathBuf::from("glfw/include/GLFW/glfw3.h");
    let headers_path_str = headers_path.to_str().expect("Path is not a valid string");

    // Tell cargo to invalidate the built crate whenever the header changes.
    println!("cargo:rerun-if-changed={}", headers_path_str);

    // The bindgen::Builder is the main entry point to bindgen,
    // and lets you build up options for the resulting bindings.
    let bindings = bindgen::Builder::default()
        // Extra content to avoid including OpenGL headers.
        .header_contents("glfw_include_none", "#define GLFW_INCLUDE_NONE")
        // Block system headers (e.g. stdint.h, stddef.h).
        .blocklist_file("/usr/include.*")
        .blocklist_file("/usr/lib/llvm.*?/lib/clang/.*?/include.*")
        // Let all #define statement convert to i32 value, as expected by glfw.
        .default_macro_constant_type(bindgen::MacroTypeVariation::Signed)
        // The input header we would like to generate bindings for.
        .header(headers_path_str)
        // Tell cargo to invalidate the built crate whenever
        // any of the included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("ffi.rs");
    bindings
        .write_to_file(out_path)
        .expect("Couldn't write bindings!");
}
