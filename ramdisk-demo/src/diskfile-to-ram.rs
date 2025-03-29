use std::io::{self, Write};
use std::path::Path;
use std::process::Command;

#[cfg(target_os = "linux")]
fn create_ramdisk_linux(size_mb: u32, mount_point: &str) -> io::Result<()> {
    // 检查挂载点是否存在
    if !Path::new(mount_point).exists() {
        std::fs::create_dir_all(mount_point)?;
    }

    // 挂载 tmpfs
    let size_kb = size_mb * 1024;
    let output = Command::new("mount")
        .args(&[
            "-t",
            "tmpfs",
            "-o",
            &format!("size={}M", size_mb),
            "tmpfs",
            mount_point,
        ])
        .output()?;

    if !output.status.success() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!(
                "Failed to mount tmpfs: {}",
                String::from_utf8_lossy(&output.stderr)
            ),
        ));
    }

    println!("Ramdisk mounted at {} with size {}MB", mount_point, size_mb);
    Ok(())
}

#[cfg(target_os = "macos")]
fn create_ramdisk_macos(size_mb: u32, mount_point: &str) -> io::Result<()> {
    // 计算 sectors (1 sector = 512 bytes)
    let sectors = size_mb * 2048; // 2048 = (1024 * 1024) / 512

    // 创建 Ramdisk 设备
    let output = Command::new("hdiutil")
        .args(&["attach", "-nomount", &format!("ram://{}", sectors)])
        .output()?;

    if !output.status.success() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!(
                "Failed to create Ramdisk: {}",
                String::from_utf8_lossy(&output.stderr)
            ),
        ));
    }

    // 获取设备名 (如 /dev/disk4)
    let device = String::from_utf8(output.stdout)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?
        .trim()
        .to_string();

    // 格式化 Ramdisk 为 HFS+
    Command::new("diskutil")
        .args(&["eraseDisk", "HFS+", "RAMDisk", &device])
        .output()?;

    println!("Ramdisk mounted at {} with size {}MB", mount_point, size_mb);
    Ok(())
}

#[cfg(target_os = "windows")]
fn create_ramdisk_windows(size_mb: u32, mount_point: &str) -> io::Result<()> {
    // 检查 ImDisk 是否安装
    let output = Command::new("imdisk").arg("-l").output();

    if output.is_err() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "ImDisk not found. Please install ImDisk Toolkit first.",
        ));
    }

    // 创建 Ramdisk (需要管理员权限)
    let output = Command::new("imdisk")
        .args(&[
            "-a",
            "-s",
            &format!("{}M", size_mb),
            "-m",
            mount_point,
            "-p",
            "/fs:ntfs /q /y",
        ])
        .output()?;

    if !output.status.success() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!(
                "Failed to create Ramdisk: {}",
                String::from_utf8_lossy(&output.stderr)
            ),
        ));
    }

    println!("Ramdisk mounted at {} with size {}MB", mount_point, size_mb);
    Ok(())
}

// 将diskfile挂载到内存中
fn main() -> io::Result<()> {
    let size_mb = 1024; // 1GB
    let mount_point = if cfg!(target_os = "windows") {
        "R:"
    } else {
        "/mnt/ramdisk"
    };

    println!("Creating Ramdisk on {}...", std::env::consts::OS);

    match std::env::consts::OS {
        #[cfg(target_os = "linux")]
        "linux" => create_ramdisk_linux(size_mb, mount_point),
        #[cfg(target_os = "macos")]
        "macos" => create_ramdisk_macos(size_mb, mount_point),
        #[cfg(target_os = "windows")]
        "windows" => create_ramdisk_windows(size_mb, mount_point),
        _ => Err(io::Error::new(
            io::ErrorKind::Other,
            "Unsupported operating system",
        )),
    }
}
