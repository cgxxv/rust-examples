use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut buffer = File::create("foo.txt")?;

    buffer.write_all(b"some bytes")?;
    buffer.write_all(b"other bytes")?;
    Ok(())
}
