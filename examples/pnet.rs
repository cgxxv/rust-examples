use ipnetwork::IpNetwork::V4;
use pnet::datalink;

fn main() {
    for iface in datalink::interfaces() {
        for ip in iface.ips {
            if let V4(ipv4) = ip {
                println!("{:#?}", ipv4);
            };
        }
    }
}
