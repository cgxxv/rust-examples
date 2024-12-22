use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=main.c");

    // Configure and compile the C code using cc crate
    cc::Build::new().file("main.c").compile("main");

    let bindings = bindgen::Builder::default()
        .header("main.c")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .blocklist_function("main")
        .generate()
        .expect("Unable to generate bindings");

    let out_dir = env::var("OUT_DIR").unwrap();
    println!("OUT_DIR: {}", out_dir);
    let out_path = PathBuf::from(out_dir).join("bindings.rs");
    bindings
        .write_to_file(out_path)
        .expect("Couldn't write bindings!");
}
