use local_ip;

fn main() {
    let ip = local_ip::get().unwrap();
    println!("local ip address: {:?}", ip.to_string());
}
