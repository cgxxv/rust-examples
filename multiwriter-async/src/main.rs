use std::{
    io::Error,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll},
};

use pin_project::pin_project;
use tokio::{
    fs::File,
    io::{AsyncWrite, AsyncWriteExt, BufWriter},
};

#[pin_project]
struct MultiWriter(Vec<Box<dyn AsyncWrite + Unpin + Send>>);

impl AsyncWrite for MultiWriter {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize, Error>> {
        let this = self.project();
        let mut all_ready = true;
        let mut min_written = buf.len();
        let mut last_error = None;

        for writer in this.0.iter_mut() {
            match Pin::new(writer).poll_write(cx, buf) {
                Poll::Ready(Ok(n)) => {
                    if n < min_written {
                        min_written = n;
                    }
                }
                Poll::Ready(Err(e)) => {
                    last_error = Some(e);
                }
                Poll::Pending => {
                    all_ready = false;
                }
            }
        }

        if !all_ready {
            return Poll::Pending;
        }

        if let Some(e) = last_error {
            return Poll::Ready(Err(e));
        }

        if min_written == 0 {
            // No progress made on any writer
            Poll::Ready(Ok(0))
        } else if min_written == buf.len() {
            // Full write on all writers
            Poll::Ready(Ok(min_written))
        } else {
            // Partial write (some writers wrote less than others)
            Poll::Ready(Err(Error::new(
                std::io::ErrorKind::Other,
                "One or more writers wrote fewer bytes than requested",
            )))
        }
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Error>> {
        let this = self.project();
        let mut all_ready = true;
        let mut last_error = None;

        for writer in this.0.iter_mut() {
            match Pin::new(writer).poll_flush(cx) {
                Poll::Ready(Ok(())) => (),
                Poll::Ready(Err(e)) => last_error = Some(e),
                Poll::Pending => all_ready = false,
            }
        }

        if !all_ready {
            Poll::Pending
        } else {
            Poll::Ready(last_error.map_or(Ok(()), Err))
        }
    }

    fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Error>> {
        let this = self.project();
        let mut all_ready = true;
        let mut last_error = None;

        for writer in this.0.iter_mut() {
            match Pin::new(writer).poll_shutdown(cx) {
                Poll::Ready(Ok(())) => (),
                Poll::Ready(Err(e)) => last_error = Some(e),
                Poll::Pending => all_ready = false,
            }
        }

        if !all_ready {
            Poll::Pending
        } else {
            Poll::Ready(last_error.map_or(Ok(()), Err))
        }
    }
}

struct BufferWriter(Arc<Mutex<Vec<u8>>>);

impl tokio::io::AsyncWrite for BufferWriter {
    fn poll_write(
        self: std::pin::Pin<&mut Self>,
        _: &mut std::task::Context<'_>,
        buf: &[u8],
    ) -> std::task::Poll<std::io::Result<usize>> {
        let mut buffer = self.0.lock().unwrap();
        buffer.extend_from_slice(buf);
        std::task::Poll::Ready(Ok(buf.len()))
    }

    fn poll_flush(
        self: std::pin::Pin<&mut Self>,
        _: &mut std::task::Context<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        std::task::Poll::Ready(Ok(()))
    }

    fn poll_shutdown(
        self: std::pin::Pin<&mut Self>,
        _: &mut std::task::Context<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        std::task::Poll::Ready(Ok(()))
    }
}

#[tokio::main]
async fn main() {
    let file1 = File::create("output1.txt").await.unwrap();
    let file2 = File::create("output2.txt").await.unwrap();
    let buffer: Arc<Mutex<Vec<u8>>> = Arc::new(Mutex::new(Vec::new()));

    let file1_writer = Box::new(BufWriter::new(file1));
    let file2_writer = Box::new(BufWriter::new(file2));
    let mem_writer = Box::new(BufWriter::new(BufferWriter(buffer.clone())));

    let mut multi_writer = MultiWriter(vec![file1_writer, file2_writer, mem_writer]);

    multi_writer
        .write_all(b"Hello, MultiWriter!\n")
        .await
        .unwrap();
    multi_writer.flush().await.unwrap();

    // 访问 `mem_writer` 缓冲区
    let buffer = buffer.lock().unwrap();
    println!("=> {}", String::from_utf8_lossy(&buffer));
}
