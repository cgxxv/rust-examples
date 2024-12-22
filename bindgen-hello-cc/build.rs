use std::env;
use std::path::PathBuf;

fn main() {
    let libdir_path = PathBuf::from("hello")
        .canonicalize()
        .expect("cannot canonicalize path");

    let headers_path = libdir_path.join("hello.h");
    let headers_path_str = headers_path.to_str().expect("Path is not a valid string");

    let out_dir = env::var("OUT_DIR").unwrap();

    // Configure and compile the C code using cc crate
    cc::Build::new()
        .file(libdir_path.join("hello.c"))
        .compile("hello");

    // Tell cargo to look for shared libraries in the specified directory
    println!("cargo:rustc-link-search={}", libdir_path.to_str().unwrap());

    // Tell cargo to tell rustc to link our `hello` library
    println!("cargo:rustc-link-lib=static=hello");

    let bindings = bindgen::Builder::default()
        .header(headers_path_str)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(out_dir).join("bindings.rs");
    bindings
        .write_to_file(out_path)
        .expect("Couldn't write bindings!");
}
