use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};

use color_eyre::eyre::Context;
use tracing::{Level, event};
use tracing_subscriber::util::SubscriberInitExt;

use crate::onvif::Onvif_Client;
mod onvif;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .without_time()
        .finish()
        .try_init()?;
    event!(Level::INFO, "Creating ONVIF client");
    let mut client = Onvif_Client::new(
        "10.66.3.2:80"
            .parse()
            .wrap_err("while parsing socket address")?,
    )
    .await.wrap_err("while creating onvif client")?;
    event!(Level::INFO, "Sending probe request");
    match client.probe().await {
        Ok(data) => {
            event!(Level::INFO, "Got response: {}", String::from_utf8(data)?)
        },
        Err(e) => {
            event!(Level::WARN, "Connection failed: {e:?}");
        }
    }
    client.probe_authed("spectator", "1331business!").await?;
    Ok(())
}
