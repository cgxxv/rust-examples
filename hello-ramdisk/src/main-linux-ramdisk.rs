#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
use std::{fs::File, io::Write, process::Command};

fn main() {
    let external_exe = include_bytes!(concat!(env!("OUT_DIR"), "/hello.bin"));

    // 将可执行文件写入内存盘
    let temp_path = "/dev/shm/external_tool";
    let mut file = File::create(temp_path).expect("Failed to create file in memory");
    file.write_all(external_exe)
        .expect("Failed to write to file");
    file.sync_all().expect("Failed to sync file");
    drop(file);

    // 赋予文件执行权限
    #[cfg(unix)]
    {
        let mut permissions = std::fs::metadata(temp_path)
            .expect("Failed to get file metadata")
            .permissions();
        permissions.set_mode(0o755); // 设置权限为 rwxr-xr-x
        std::fs::set_permissions(temp_path, permissions).expect("Failed to set file permissions");
    }

    // 执行外部可执行文件
    let output = Command::new(temp_path)
        .output()
        .expect("Failed to execute external tool");

    // 打印外部可执行文件的输出
    println!("External tool output: {:?}", output);

    // 清理临时文件
    std::fs::remove_file(temp_path).expect("Failed to delete temp file");

    println!("Hello, I am from internal!");
}
