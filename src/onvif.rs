use base64::{Engine, prelude::BASE64_STANDARD};
use color_eyre::eyre::{Result, eyre};
use http_body_util::BodyExt;
use hyper::{Method, Request, client::conn::http1};
use hyper_util::rt::TokioIo;
use rand::rngs::ThreadRng;
use std::net::SocketAddr;
use tokio::net::{TcpStream, UdpSocket};
use tracing::{Instrument, Level, event, instrument, span};
use uuid::Uuid;

mod namespaces;
#[allow(unused_imports)]
use namespaces::tt::*;

const ONVIF_DISCOVER_TEMPLATE: &'static str = include_str!("../include/Discovery.xml");
const ONVIF_DATETIME_TEMPLATE: &'static str = include_str!("../include/SystemDatetime.xml");

pub struct OnvifClient {
    pub target: SocketAddr,
    rng_source: ThreadRng,
}
impl std::fmt::Debug for OnvifClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("Onvif_Client({})", self.target.to_string()))?;
        Ok(())
    }
}

impl OnvifClient {
    pub async fn new(target: SocketAddr) -> Result<Self> {
        let rv = Self {
            target: target.clone(),
            rng_source: rand::rng(),
        };
        Ok(rv)
    }

    #[allow(unused)]
    pub async fn discover() -> Result<()> {
        let client = UdpSocket::bind("0.0.0.0:0").await?;
        if !client.broadcast()? {
            client.set_broadcast(true)?;
            event!(Level::DEBUG, "Enabled broadcasting");
        }
        event!(Level::DEBUG, "Sending discovery packet");
        client
            .send_to(
                ONVIF_DISCOVER_TEMPLATE
                    .replace("$UUID$", &Uuid::new_v4().to_string())
                    .as_bytes(),
                "239.255.255.250:3702",
            )
            .await?;
        event!(Level::DEBUG, "Waiting for response");
        let mut rbuf: Vec<u8> = Vec::new();
        let (_amt, rec_from) = client.recv_buf_from(&mut rbuf).await?;
        event!(
            Level::TRACE,
            "{rec_from} sent {:#?}",
            BASE64_STANDARD.encode(rbuf)
        );
        todo!();
    }
    #[instrument]
    pub async fn get_device_datetime(&mut self) -> Result<()> {
        let host = self.target.ip().to_canonical().to_string();
        let port = self.target.port().to_string();
        let url = format!("http://{host}:{port}/onvif/device_service").parse::<hyper::Uri>()?;
        event!(Level::DEBUG, "Using host={host} port={port}");
        event!(Level::DEBUG, "Connecting {host}");
        let stream = TcpStream::connect(self.target).await?;
        let io = TokioIo::new(stream);
        event!(Level::DEBUG, "Trying handshake");
        let (mut sender, connection) = http1::handshake(io).await?;
        tokio::spawn(
            async move {
                if let Err(e) = connection.await {
                    event!(Level::ERROR, "Connection failed: {e}");
                }
            }
            .instrument(span!(Level::WARN, "")),
        );
        event!(Level::DEBUG, "Building request");
        let req = Request::builder()
            .method(Method::POST)
            .uri(&url)
            .header(
                hyper::header::HOST,
                url.authority()
                    .ok_or(eyre!("No authority found"))?
                    .clone()
                    .as_str(),
            )
            .body(ONVIF_DATETIME_TEMPLATE.to_string())?;
        event!(Level::DEBUG, "Firing request");
        let mut res = sender.send_request(req).await?;
        let mut body: Vec<u8> = Vec::new();
        event!(Level::DEBUG, "Collecting body");
        while let Some(next_frame) = res.frame().await {
            let frame = next_frame?;
            if let Some(chunk) = frame.data_ref() {
                for byte in chunk {
                    body.push(*byte);
                }
            }
        }
        event!(Level::TRACE, "Response: {:#?}", String::from_utf8(body));

        Ok(())
    }
}
