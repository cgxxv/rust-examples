use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

mod thread_pool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = thread_pool::ThreadPool::new(4);

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        //handle_connection(stream);

        //thread::spawn(|| {
        //    handle_connection(stream);
        //});

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    //println!("{}", String::from_utf8_lossy(&buffer[..]));

    let get = b"GET / HTTP/1.1\r\n";

    //if buffer.starts_with(get) {
    //    let contents = fs::read_to_string("hello.html").unwrap();

    //    let response = format!(
    //        "HTTP/1.1 200 ok\r\nContent-Length: {}\r\n\r\n{}",
    //        contents.len(),
    //        contents
    //    );
    //    stream.write(response.as_bytes()).unwrap();
    //    stream.flush().unwrap();
    //} else {
    //    let status_line = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
    //    let contents = fs::read_to_string("404.html").unwrap();

    //    let response = format!("{}{}", status_line, contents);

    //    stream.write(response.as_bytes()).unwrap();
    //    stream.flush().unwrap();
    //}

    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
