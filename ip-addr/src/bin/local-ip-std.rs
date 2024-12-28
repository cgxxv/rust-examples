use std::net::{SocketAddr, UdpSocket};

fn get_local_ip() -> Option<String> {
    // 创建一个UDP套接字
    let socket = UdpSocket::bind("0.0.0.0:0").ok()?;
    // 连接到外部地址
    socket.connect("8.8.8.8:80").ok()?;
    // 获取本地地址
    let local_addr: SocketAddr = socket.local_addr().ok()?;
    Some(local_addr.ip().to_string())
}

fn main() {
    match get_local_ip() {
        Some(ip) => println!("Local IP: {}", ip),
        None => eprintln!("Unable to determine local IP address."),
    }
}

