use std::io::Write;
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
use std::process::Command;
use tempfile::NamedTempFile;

fn main() {
    let external_exe = include_bytes!(concat!(env!("OUT_DIR"), "/hello.bin"));

    // 将嵌入的可执行文件写入临时文件
    let mut temp_file = NamedTempFile::new().expect("Failed to create temp file");
    temp_file
        .write_all(external_exe)
        .expect("Failed to write to temp file");
    let temp_path = temp_file.into_temp_path();

    // 赋予文件执行权限
    #[cfg(unix)]
    {
        let mut permissions = std::fs::metadata(&temp_path)
            .expect("Failed to get file metadata")
            .permissions();
        permissions.set_mode(0o755); // 设置权限为 rwxr-xr-x
        std::fs::set_permissions(&temp_path, permissions).expect("Failed to set file permissions");
    }

    // 执行外部可执行文件
    let output = Command::new(&temp_path)
        .output()
        .expect("Failed to execute external tool");

    // 打印外部可执行文件的输出
    println!("External tool output: {:?}", output);

    // 清理临时文件
    std::fs::remove_file(temp_path).expect("Failed to delete temp file");

    println!("Hello, I am from internal!");
}
