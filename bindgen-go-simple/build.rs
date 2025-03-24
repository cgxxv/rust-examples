use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let out_dir = std::env::var("OUT_DIR").unwrap();
    let out_path = format!("{}/main.so", out_dir);

    let output = Command::new("go")
        .arg("build")
        .arg("-o")
        .arg(out_path)
        .arg("-buildmode=c-shared")
        .arg("main.go")
        .output()
        .unwrap();
    println!("Go build output: {:?}", output);
}
