extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    // Tell cargo to tell rustc to link the system bzip2
    // shared library.
    println!("cargo:rustc-link-lib=bz2");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let mut builder = bindgen::Builder::default();

    // #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
    // {
    //     builder = builder
    //         // 添加 clang 参数，告诉 clang 去哪里找头文件
    //         .clang_arg("-I/opt/homebrew/Cellar/bzip2/1.0.8/include")
    //         // 显式指定标准 C 头文件目录，通常为 Xcode 工具链中的路径
    //         .clang_arg("-I/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk/usr/include");
    // }

    // #[cfg(all(target_os = "macos", target_arch = "x86_64"))]
    // {
    //     builder = builder
    //         // 添加 clang 参数，告诉 clang 去哪里找头文件
    //         .clang_arg("-I/user/homebrew/Cellar/bzip2/1.0.8/include")
    //         // 显式指定标准 C 头文件目录，通常为 Xcode 工具链中的路径
    //         .clang_arg("-I/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk/usr/include");
    // }

    #[cfg(target_os = "macos")]
    {
        builder = builder
            // 显式指定标准 C 头文件目录，通常为 Xcode 工具链中的路径
            .clang_arg("-I/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk/usr/include");
        #[cfg(target_arch = "aarch64")]
        {
            builder = builder
                // 添加 clang 参数，告诉 clang 去哪里找头文件
                .clang_arg("-I/opt/homebrew/Cellar/bzip2/1.0.8/include");
        }
        #[cfg(target_arch = "x86_64")]
        {
            builder = builder
                // 添加 clang 参数，告诉 clang 去哪里找头文件
                .clang_arg("-I/usr/homebrew/Cellar/bzip2/1.0.8/include");
        }
    }

    let bindings = builder
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
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
