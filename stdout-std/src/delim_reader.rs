use std::io::{self, BufReader, Read};

use tokio::sync::mpsc;

pub fn delim_reader(r: Box<dyn Read + Send>, delim: u8) -> mpsc::Receiver<String> {
    let (tx, rx) = mpsc::channel(100);

    tokio::spawn(async move {
        let mut buf_reader = BufReader::new(r);
        let mut line = Vec::new();

        loop {
            let mut byte = [0u8];
            match buf_reader.read_exact(&mut byte) {
                Ok(_) => {
                    line.push(byte[0]);

                    if byte[0] == delim {
                        line.pop();
                        // Send the collected line
                        if let Err(err) = tx.send(String::from_utf8_lossy(&line).to_string()).await
                        {
                            eprintln!("=> ERROR sending data: {err}");
                            break;
                        }
                        line.clear();
                    }
                }
                Err(err) => {
                    if err.kind() == io::ErrorKind::UnexpectedEof {
                        println!("=> EOF reached");
                    } else {
                        eprintln!("=> ERROR reading: {err}");
                    }
                    break;
                } // Error or EOF
            }
        }

        // Close the channel when done reading
        drop(tx);
    });

    rx
}

#[cfg(test)]
mod test {
    use std::fs::File;

    use super::*;

    #[tokio::test]
    async fn test_delim_reader() {
        let file = File::open("example.txt").unwrap();
        let reader = Box::new(file);

        let delim = b'\n'; // Newline delimiter
        let mut reader_channel = delim_reader(reader, delim);

        // Read the lines from the channel
        while let Some(line) = reader_channel.recv().await {
            println!("Received line: {}", line);
        }
    }
}
