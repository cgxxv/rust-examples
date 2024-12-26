use pnet::datalink;

fn main() {
    let interfaces = datalink::interfaces();
    for iface in interfaces {
        for ip in iface.ips {
            println!("Interface: {}, IP: {}", iface.name, ip.ip());
        }
    }
}

