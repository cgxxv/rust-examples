use applevisor::hypervisor::*;
use applevisor::vm::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化 Hypervisor
    let hypervisor = Hypervisor::new()?;
    println!("Hypervisor initialized.");

    // 创建虚拟机
    let mut vm = VirtualMachine::new(&hypervisor)?;
    println!("Virtual Machine created.");

    // 配置内存
    let memory_size = 4 * 1024 * 1024 * 1024; // 4 GB
    vm.set_memory(memory_size)?;
    println!("Memory size set to {} bytes.", memory_size);

    // 加载磁盘映像
    let disk_path = "/path/to/disk.img";
    let disk = vm.add_block_device(disk_path, false)?;
    println!("Disk added: {}", disk_path);

    // 配置启动加载器
    vm.set_boot_loader("/path/to/bootloader")?;
    println!("Boot loader configured.");

    // 启动虚拟机
    vm.start()?;
    println!("Virtual Machine started.");

    // 等待虚拟机结束
    vm.wait_for_exit()?;
    println!("Virtual Machine exited.");

    Ok(())
}
