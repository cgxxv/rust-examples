use std::fs::OpenOptions;
use std::io::{prelude::*, BufWriter};

use bytes::{BufMut, BytesMut};

fn main() {
    let mut buf = BytesMut::with_capacity(10240);
    buf.put(&b"Hello World\n"[..]);
    println!("{:?} {}", buf, buf.capacity());
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("foo.txt")
        .unwrap();
    let mut file = BufWriter::new(file);
    file.write(&buf);
    let mut buf2 = BytesMut::with_capacity(10240);
    buf2.put(&b"hello\n"[..]);
    file.write(&buf2);
    buf.clear();
    println!("{:?} {}", buf, buf.capacity());
}
