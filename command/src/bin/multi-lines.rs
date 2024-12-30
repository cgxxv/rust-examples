use std::process::Command;

fn main() {
    let script = r#"
    echo a
    echo b
    echo c
    echo d
    echo e
    "#;

    // 将多行脚本分割成每一行命令
    let commands: Vec<&str> = script
        .lines()
        .map(str::trim) // 去掉每行的多余空白
        .filter(|line| !line.is_empty()) // 忽略空行
        .collect();

    for cmd in commands {
        println!("=> {}", cmd);
        // 使用 std::process::Command 执行每一行命令
        let output = Command::new("sh")
            .arg("-c")
            .arg(cmd)
            .output()
            .expect("Failed to execute command");

        println!("{}", String::from_utf8_lossy(&output.stdout));
    }
}
