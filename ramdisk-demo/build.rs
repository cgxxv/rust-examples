fn main() {
    // println!("cargo:rustc-link-arg=/MANIFESTUAC:level='requireAdministrator'");

    use std::env;
    use std::fs;
    use std::path::{Path, PathBuf};

    // 2. 获取输出目录
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let bin_dir = Path::new("bin");

    // 3. 遍历 bin 目录下的所有文件
    for entry in fs::read_dir(bin_dir).unwrap() {
        let entry = entry.unwrap();
        let src_path = entry.path();

        if src_path
            .extension()
            .map(|ext| {
                if cfg!(target_os = "windows") {
                    ext.to_string_lossy().to_lowercase() == "exe"
                } else {
                    true
                }
            })
            .unwrap_or(false)
        {
            let bin_name = src_path.file_stem().unwrap().to_string_lossy();
            let bin_name = to_snake_case(&bin_name);

            // 5. 将文件复制到输出目录
            let dest_path = out_dir.join(format!("{bin_name}.bin"));
            fs::copy(&src_path, &dest_path).unwrap();

            // 6. 告诉 Cargo 如果文件发生变化，重新运行 build.rs
            println!("cargo:rerun-if-changed={}", src_path.display());
        }
    }
}

#[allow(unused)]
fn to_snake_case(input: &str) -> String {
    let mut result = String::new();
    let mut prev_was_upper = false;
    let mut prev_was_underscore = false;

    for (i, c) in input.chars().enumerate() {
        if c.is_uppercase() {
            if i > 0 && !prev_was_upper && !prev_was_underscore {
                result.push('_');
            }
            result.push(c.to_ascii_lowercase());
            prev_was_upper = true;
            prev_was_underscore = false;
        } else if c == '-' || c == '.' || c == '_' {
            if !prev_was_underscore {
                result.push('_');
            }
            prev_was_upper = false;
            prev_was_underscore = true;
        } else {
            result.push(c);
            prev_was_upper = false;
            prev_was_underscore = false;
        }
    }

    result
}
