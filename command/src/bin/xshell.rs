use std::str;

use encoding_rs::GBK;
use xshell::{cmd, Shell};

fn main() {
    let sh = Shell::new().unwrap();

    if cfg!(target_os = "windows") {
        // PowerShell command
        let output = cmd!(sh, "powershell -Command Get-WindowsOptionalFeature -Online -FeatureName Microsoft-Hyper-V-All")
            .output()
            .unwrap()
            .stdout;

        // Try to decode as UTF-8 first, if it fails, try GBK
        let output_str = match str::from_utf8(&output) {
            Ok(v) => v.to_string(),
            Err(_) => {
                let (decoded, _, _) = GBK.decode(&output);
                decoded.into_owned()
            }
        };

        println!("Windows Output:\n{}", output_str);
    } else {
        // Linux shell command
        let output = cmd!(sh, "lsb_release -a")
            .read()
            .expect("Failed to execute Linux shell command");
        println!("Linux Output:\n{}", output);
    }
}
