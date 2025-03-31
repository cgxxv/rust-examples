use std::{fs::File, io::Write};

struct MultiWriter<W: Write>(Vec<W>);

impl<W: Write> Write for MultiWriter<W> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let mut last_error = None;
        let mut total_written = 0;
        let mut bytes_written = 0;
        for (index, writer) in self.0.iter_mut().enumerate() {
            match writer.write(buf) {
                Ok(n) => {
                    bytes_written = n;
                    total_written += n;
                }
                Err(e) => {
                    println!("=> write {index} failed {e}");
                    last_error = Some(e);
                }
            }
        }

        println!("=> total written: {total_written}");
        last_error.map_or(Ok(bytes_written), Err)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        for writer in &mut self.0 {
            writer.flush()?;
        }
        Ok(())
    }
}

fn main() {
    let file1 = File::create("out1.txt").unwrap();
    let file2 = File::create("out2.txt").unwrap();
    let mut writer = MultiWriter(vec![file1, file2]);
    writer.write_all(b"hello").unwrap();
}
