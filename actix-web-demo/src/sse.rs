use actix_web::HttpResponse;
use actix_web::Responder;
use actix_web::get;
use actix_web::http::header::ContentEncoding;
use bytes::Bytes;
use futures_util::Stream;
use std::pin::Pin;
use std::task::Context;
use std::task::Poll;
use std::time::Duration;
use tokio::time::Interval;
use tokio::time::interval;

#[allow(unused)]
pub(super) fn configure_utoipa() -> impl FnOnce(&mut utoipa_actix_web::service_config::ServiceConfig)
{
    |config: &mut utoipa_actix_web::service_config::ServiceConfig| {
        config.service(utoipa_actix_web::scope("").service(handle_ping));
    }
}

pub struct Sse {
    keep_alive: Interval,
}

impl Stream for Sse {
    type Item = Result<Bytes, actix_web::Error>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match self.keep_alive.poll_tick(cx) {
            Poll::Ready(_) => {
                return Poll::Ready(Some(Ok(Bytes::from_static(b": ping\n\n"))));
            }
            Poll::Pending => {}
        }

        Poll::Pending
    }
}

#[utoipa::path(
    responses(
        (status = 200, description = "handle ping", body = String)
    ),
)]
#[get("/ping")]
async fn handle_ping() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/event-stream")
        .insert_header(("Cache-Control", "no-cache"))
        .insert_header(ContentEncoding::Identity) // important for eventsource, when using global compress
        .streaming(Sse {
            keep_alive: interval(Duration::from_secs(1)),
        })
}
