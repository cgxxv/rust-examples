use std::{process::Command, str};

use encoding_rs::GBK;

fn main() {
    // let argv = &["python", "-c", "print('Hello world!')"];

    // println!("Executing: {}", shell_words::join(argv));

    // std::process::Command::new(&argv[0])
    //     .args(&argv[1..])
    //     .spawn()
    //     .expect("failed to start subprocess")
    //     .wait()
    //     .expect("failed to wait for subprocess");

    //请用shell_words执行Get-WindowsOptionalFeature -Online -FeatureName Microsoft-Hyper-V-All，并打印输出结果
    // let argv = &[
    //     "powershell",
    //     "-Command",
    //     "Get-WindowsOptionalFeature -Online -FeatureName Microsoft-Hyper-V-All",
    // ];
    // println!("Executing: {}", shell_words::join(argv));

    // std::process::Command::new(&argv[0])
    //     .args(&argv[1..])
    //     .spawn()
    //     .expect("failed to start subprocess")
    //     .wait()
    //     .expect("failed to wait for subprocess");

    // let output = Command::new(argv[0])
    //     .args(&argv[1..])
    //     .output()
    //     .unwrap()
    //     .stdout;

    // // Try to decode as UTF-8 first, if it fails, try GBK
    // let output_str = match str::from_utf8(&output) {
    //     Ok(v) => v.to_string(),
    //     Err(_) => {
    //         let (decoded, _, _) = GBK.decode(&output);
    //         decoded.into_owned()
    //     }
    // };

    // println!("Output: {}", output_str);
    // println!();

    let args = vec![r#"echo "Test""#, "exit 0"];

    let command = Command::new("pwsh").arg("-c").args(&args).output().unwrap();

    println!("=> {:#?}", command);

    let output = command.stdout;
    // Try to decode as UTF-8 first, if it fails, try GBK
    let output_str = match str::from_utf8(&output) {
        Ok(v) => v.to_string(),
        Err(_) => {
            let (decoded, _, _) = GBK.decode(&output);
            decoded.into_owned()
        }
    };

    println!("Output: {}", output_str);
    println!();

    let command = Command::new("pwsh")
        .arg("-c")
        .arg(
            r#"
            echo "Setting up environment"
            $VMName = "WindowsServer2022"
            echo $VMName
            exit 0
            "#,
        )
        .output()
        .unwrap();

    println!("=> {:#?}", command);

    let output = command.stdout;
    // Try to decode as UTF-8 first, if it fails, try GBK
    let output_str = match str::from_utf8(&output) {
        Ok(v) => v.to_string(),
        Err(_) => {
            let (decoded, _, _) = GBK.decode(&output);
            decoded.into_owned()
        }
    };

    println!("Output: {}", output_str);
}
