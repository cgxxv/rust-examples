// use std::pin::Pin;
// use tokio::io::{self, AsyncWrite};

// // 修改为使用特征对象，明确要求 Send + Sync
// #[pin_project::pin_project]
// pub struct MultiWriter(pub Vec<Pin<Box<dyn AsyncWrite + Send + Sync>>>);

// impl AsyncWrite for MultiWriter {
//     fn poll_write(
//         self: Pin<&mut Self>,
//         cx: &mut std::task::Context<'_>,
//         buf: &[u8],
//     ) -> std::task::Poll<Result<usize, io::Error>> {
//         let this = self.project();
//         let mut total = 0;
//         for w in this.0.iter_mut() {
//             match w.as_mut().poll_write(cx, buf) {
//                 std::task::Poll::Ready(Ok(n)) => total += n,
//                 std::task::Poll::Ready(Err(e)) => return std::task::Poll::Ready(Err(e)),
//                 std::task::Poll::Pending => return std::task::Poll::Pending,
//             }
//         }
//         std::task::Poll::Ready(Ok(total))
//     }

//     fn poll_flush(
//         self: Pin<&mut Self>,
//         cx: &mut std::task::Context<'_>,
//     ) -> std::task::Poll<Result<(), io::Error>> {
//         let this = self.project();
//         let mut result = Ok(());
//         for w in this.0.iter_mut() {
//             match w.as_mut().poll_flush(cx) {
//                 std::task::Poll::Ready(Ok(())) => {}
//                 std::task::Poll::Ready(Err(e)) => result = Err(e),
//                 std::task::Poll::Pending => return std::task::Poll::Pending,
//             }
//         }
//         std::task::Poll::Ready(result)
//     }

//     fn poll_shutdown(
//         self: Pin<&mut Self>,
//         cx: &mut std::task::Context<'_>,
//     ) -> std::task::Poll<Result<(), io::Error>> {
//         let this = self.project();
//         let mut result = Ok(());
//         for w in this.0.iter_mut() {
//             match w.as_mut().poll_shutdown(cx) {
//                 std::task::Poll::Ready(Ok(())) => {}
//                 std::task::Poll::Ready(Err(e)) => result = Err(e),
//                 std::task::Poll::Pending => return std::task::Poll::Pending,
//             }
//         }
//         std::task::Poll::Ready(result)
//     }
// }

use std::pin::Pin;

use tokio::io::{self, AsyncWrite};

#[pin_project::pin_project]
pub struct MultiWriter<W: AsyncWrite + Send + Unpin>(pub Vec<W>);

impl<W: AsyncWrite + Send + Unpin> AsyncWrite for MultiWriter<W> {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &[u8],
    ) -> std::task::Poll<Result<usize, io::Error>> {
        let this = self.project();
        let mut total = 0;
        for w in this.0.iter_mut() {
            match Pin::new(w).poll_write(cx, buf) {
                std::task::Poll::Ready(Ok(n)) => total += n,
                std::task::Poll::Ready(Err(e)) => return std::task::Poll::Ready(Err(e)),
                std::task::Poll::Pending => return std::task::Poll::Pending,
            }
        }
        std::task::Poll::Ready(Ok(total))
    }

    fn poll_flush(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), io::Error>> {
        let this = self.project();
        let mut result = Ok(());
        for w in this.0.iter_mut() {
            match Pin::new(w).poll_flush(cx) {
                std::task::Poll::Ready(Ok(())) => {}
                std::task::Poll::Ready(Err(e)) => result = Err(e),
                std::task::Poll::Pending => return std::task::Poll::Pending,
            }
        }
        std::task::Poll::Ready(result)
    }

    fn poll_shutdown(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), io::Error>> {
        let this = self.project();
        let mut result = Ok(());
        for w in this.0.iter_mut() {
            match Pin::new(w).poll_shutdown(cx) {
                std::task::Poll::Ready(Ok(())) => {}
                std::task::Poll::Ready(Err(e)) => result = Err(e),
                std::task::Poll::Pending => return std::task::Poll::Pending,
            }
        }
        std::task::Poll::Ready(result)
    }
}
