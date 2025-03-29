use anyhow::Result;
use std::env;
use std::fs;
use std::io::Write;
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
use std::process::Command;

fn main() -> Result<()> {
    let bin_name = "hello.bin";

    // 从编译时嵌入的二进制数据加载
    let embeded_bin = include_bytes!(concat!(env!("OUT_DIR"), "/{bin_name}"));

    let ramdisk_path = if cfg!(target_os = "windows") {
        // Windows: 挂载到 R:\
        "R:\\"
    } else {
        // Linux/macOS: 挂载到 /dev/shm/
        "/dev/shm/"
    };

    let target_path = format!("{ramdisk_path}/{bin_name}");
    let mut file = fs::File::create(&target_path).expect("Failed to create file in memory");
    file.write_all(embeded_bin)
        .expect("Failed to write to file");
    file.sync_all().expect("Failed to sync file");
    drop(file);

    // 设置可执行权限 (Unix-like 系统)
    #[cfg(unix)]
    {
        let mut perms = fs::metadata(&target_path)?.permissions();
        perms.set_mode(0o755); // rwxr-xr-x
        fs::set_permissions(&target_path, perms)?;
    }

    // // 执行二进制
    // let status = Command::new(&target_path)
    //     .status()
    //     .context("Failed to execute binary")?;

    // if !status.success() {
    //     anyhow::bail!("Binary execution failed");
    // }

    // 执行外部可执行文件
    let output = Command::new(&target_path)
        .output()
        .expect("Failed to execute external tool");

    // 打印外部可执行文件的输出
    println!("External tool output: {:?}", output);

    // 清理 (可选)
    fs::remove_file(&target_path).ok();

    Ok(())
}
