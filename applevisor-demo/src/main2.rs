use applevisor::*;

fn main() {
    // Creates a new virtual machine. There can be one, and only one, per process. Operations
    // on the virtual machine remains possible as long as this object is valid.
    let _vm = VirtualMachine::new().unwrap();

    // Creates a new virtual CPU. This object abstracts operations that can be performed on
    // CPUs, such as starting and stopping them, changing their registers, etc.
    let vcpu = Vcpu::new().unwrap();

    // Enables debug features for the hypervisor. This is optional, but it might be required
    // for certain features to work, such as breakpoints.
    assert!(vcpu.set_trap_debug_exceptions(true).is_ok());
    assert!(vcpu.set_trap_debug_reg_accesses(true).is_ok());

    // Creates a mapping object that represents a 0x1000-byte physical memory range.
    let mut mem = Mapping::new(0x1000).unwrap();

    // This mapping needs to be mapped to effectively allocate physical memory for the guest.
    // Here we map the region at address 0x4000 and set the permissions to Read-Write-Execute.
    assert_eq!(mem.map(0x4000, MemPerms::RWX), Ok(()));

    // Writes a `mov x0, #0x42` instruction at address 0x4000.
    assert_eq!(mem.write_dword(0x4000, 0xd2800840), Ok(4));
    // Writes a `brk #0` instruction at address 0x4004.
    assert_eq!(mem.write_dword(0x4004, 0xd4200000), Ok(4));

    // Sets PC to 0x4000.
    assert!(vcpu.set_reg(Reg::PC, 0x4000).is_ok());

    // Starts the Vcpu. It will execute our mov and breakpoint instructions before stopping.
    assert!(vcpu.run().is_ok());

    // The *exit information* can be used to used to retrieve different pieces of
    // information about the CPU exit status (e.g. exception type, fault address, etc.).
    let _exit_info = vcpu.get_exit_info();

    // If everything went as expected, the value in X0 is 0x42.
    assert_eq!(vcpu.get_reg(Reg::X0), Ok(0x42));
}
