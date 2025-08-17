use anyhow::{Context, anyhow};
use tracing::{error, instrument};
use tracing_error::ErrorLayer;
use tracing_subscriber::{Registry, fmt, layer::SubscriberExt};

#[instrument(err)]
async fn fail() -> anyhow::Result<()> {
    Err(anyhow!("outer error").context("with extra context"))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = Registry::default()
        .with(fmt::layer())
        .with(ErrorLayer::default());

    tracing::subscriber::set_global_default(subscriber).unwrap();

    fail().await
}
