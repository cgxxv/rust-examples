use std::str;

use duct::cmd;
use encoding_rs::GBK;

fn main() {
    if cfg!(target_os = "windows") {
        // PowerShell command
        let output = cmd!(
            "powershell",
            "-Command",
            "Get-WindowsOptionalFeature -Online -FeatureName Microsoft-Hyper-V-All"
        )
        .stdout_capture()
        .unchecked()
        .run()
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
        let output = cmd!("lsb_release", "-a").read().unwrap();
        println!("Linux Output:\n{}", output);
    }
}
