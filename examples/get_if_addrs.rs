use get_if_addrs::get_if_addrs;

fn main() {
    match get_if_addrs() {
        Ok(if_addrs) => {
            for iface in if_addrs {
                println!("Interface: {}, IP: {}", iface.name, iface.addr.ip());
            }
        }
        Err(e) => eprintln!("Failed to get network interfaces: {}", e),
    }
}
