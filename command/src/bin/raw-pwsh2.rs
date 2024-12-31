use std::process::Command;

fn main() {
    let script = r#"
    echo "Hello"
    Write-Host 'PowerShell script is running'
    Start-Sleep -Seconds 5
    Write-Host 'PowerShell script finished'
    "#;
    let mut cmd = Command::new("pwsh.exe");

    cmd.arg("-c").arg(script);
    println!("=> cmd: {:#?}", cmd);

    let result = cmd.output();

    match result {
        Ok(output) => {
            println!("=> {:#?}", output);
            if output.status.success() {
                println!("{}", String::from_utf8_lossy(&output.stdout));
            }
        }
        Err(e) => {
            eprintln!("=> Error: {}", e);
        }
    }
}
