use std::str;

use encoding_rs::GBK;
use run_script::ScriptOptions;

fn main() {
    let options = ScriptOptions::new();

    if cfg!(target_os = "windows") {
        // PowerShell script
        let script = r#"
        powershell -Command "Get-WindowsOptionalFeature -Online -FeatureName Microsoft-Hyper-V-All"
        "#;

        let (code, output, error) = run_script::run(script, &vec![], &options).unwrap();

        // Try to decode as UTF-8 first, if it fails, try GBK
        let output_str = match str::from_utf8(output.as_bytes()) {
            Ok(v) => v.to_string(),
            Err(e) => {
                println!("Error: {}", e);
                let (decoded, _, _) = GBK.decode(output.as_bytes());
                decoded.into_owned()
            }
        };

        println!(
            "Exit Code: {}\nOutput: {}\nError: {}",
            code, output_str, error
        );
    } else {
        // Linux script
        let script = r#"
        lsb_release -a
        "#;
        let (code, output, error) = run_script::run(script, &vec![], &options).unwrap();
        println!("Exit Code: {}\nOutput: {}\nError: {}", code, output, error);
    }
}
