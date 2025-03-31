use std::{
    pin::Pin,
    task::{Context, Poll},
};
use tokio::io::{self, AsyncWrite};

// 修改为使用特征对象，明确要求 Send + Sync
#[pin_project::pin_project]
pub struct MultiWriter(pub Vec<Pin<Box<dyn AsyncWrite + Send + Sync>>>);

impl AsyncWrite for MultiWriter {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize, io::Error>> {
        let this = self.project();
        let mut total = 0;
        for w in this.0.iter_mut() {
            match w.as_mut().poll_write(cx, buf) {
                Poll::Ready(Ok(n)) => total += n,
                Poll::Ready(Err(e)) => return Poll::Ready(Err(e)),
                Poll::Pending => return Poll::Pending,
            }
        }
        Poll::Ready(Ok(total))
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), io::Error>> {
        let this = self.project();
        let mut result = Ok(());
        for w in this.0.iter_mut() {
            match w.as_mut().poll_flush(cx) {
                Poll::Ready(Ok(())) => {}
                Poll::Ready(Err(e)) => result = Err(e),
                Poll::Pending => return Poll::Pending,
            }
        }
        Poll::Ready(result)
    }

    fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), io::Error>> {
        let this = self.project();
        let mut result = Ok(());
        for w in this.0.iter_mut() {
            match w.as_mut().poll_shutdown(cx) {
                Poll::Ready(Ok(())) => {}
                Poll::Ready(Err(e)) => result = Err(e),
                Poll::Pending => return Poll::Pending,
            }
        }
        Poll::Ready(result)
    }
}

#[pin_project::pin_project]
pub struct MultiWriterV2<W: AsyncWrite + Send + Unpin>(pub Vec<W>);

impl<W: AsyncWrite + Send + Unpin> AsyncWrite for MultiWriterV2<W> {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize, io::Error>> {
        let this = self.project();
        let mut total = 0;
        for w in this.0.iter_mut() {
            match Pin::new(w).poll_write(cx, buf) {
                Poll::Ready(Ok(n)) => total += n,
                Poll::Ready(Err(e)) => return Poll::Ready(Err(e)),
                Poll::Pending => return Poll::Pending,
            }
        }
        Poll::Ready(Ok(total))
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), io::Error>> {
        let this = self.project();
        let mut result = Ok(());
        for w in this.0.iter_mut() {
            match Pin::new(w).poll_flush(cx) {
                Poll::Ready(Ok(())) => {}
                Poll::Ready(Err(e)) => result = Err(e),
                Poll::Pending => return Poll::Pending,
            }
        }
        Poll::Ready(result)
    }

    fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), io::Error>> {
        let this = self.project();
        let mut result = Ok(());
        for w in this.0.iter_mut() {
            match Pin::new(w).poll_shutdown(cx) {
                Poll::Ready(Ok(())) => {}
                Poll::Ready(Err(e)) => result = Err(e),
                Poll::Pending => return Poll::Pending,
            }
        }
        Poll::Ready(result)
    }
}
