use machine_ip;

fn main() {
    let ip = machine_ip::get().unwrap();
    println!("local ip address: {:?}", ip.to_string());
}
