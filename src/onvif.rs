use std::{net::{IpAddr, SocketAddr}, time::{Duration, SystemTime}};
use base64::{Engine, prelude::BASE64_STANDARD};
use color_eyre::eyre::{eyre, Context, Result};
use rand::{Fill, Rng, rngs::ThreadRng};
use sha1::Digest;
use tokio::{net::UdpSocket, time::timeout};
use tracing::{Level, event, instrument};
use uuid::Uuid;

const ONVIF_DISCOVER_UNAUTHED_TEMPLATE:&'static str=include_str!("../include/Discovery_Unauthenticated.xml");
const ONVIF_DISCOVER_AUTHED_TEMPLATE:&'static str=include_str!("../include/Discovery_Authenticated.xml");

pub struct Onvif_Client {
    inner:UdpSocket,
    pub target:SocketAddr,
    rng_source:ThreadRng
}
impl std::fmt::Debug for Onvif_Client {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("Onvif_Client({})", self.target.to_string()))?;
            Ok(())
    }
}

impl Onvif_Client {
    pub async fn new(target:SocketAddr) -> Result<Self> {
        let rv = Self {
            inner: UdpSocket::bind("0.0.0.0:0").await.wrap_err("While binding UDP socket")?,
            target:target.clone(),
            rng_source:rand::rng()
        };

        Ok(rv)
    }

    #[instrument(skip(user,pass))]
    pub async fn probe_authed<'a>(&mut self, user:&'a str, pass:&'a str) -> Result<Vec<u8>> {
        let mut nonce:[u8;32] = [0u8;32];
        self.rng_source.fill(&mut nonce);
        let nonce_b64 = BASE64_STANDARD.encode(nonce);
        event!(Level::DEBUG, "Generated nonce {nonce_b64}");
        let now:chrono::DateTime<chrono::Utc> = chrono::Utc::now();
        event!(Level::DEBUG, "Using NOW={}", now.format("%FT%TZ"));
        let mut hasher = sha1::Sha1::new();
        hasher.update([
            Vec::from(nonce),
            now.format("%TT%FZ").to_string().into(),
            pass.into()
        ].concat());
         let digest = hasher.finalize();
         event!(Level::DEBUG, "digest hash: {}", BASE64_STANDARD.encode(digest));
         let payload = ONVIF_DISCOVER_AUTHED_TEMPLATE
             .replace("$USERNAME$", user)
             .replace("$PASSHASH$", &BASE64_STANDARD.encode(digest))
             .replace("$NONCE$", &nonce_b64)
             .replace("$NOW$", &now.format("%FT%TZ").to_string())
             .replace("$UUID$", &Uuid::new_v4().to_string());
        Ok(self.send_with_timeout(payload.as_bytes(), 3000).await.wrap_err("While sending authenticated probe")?)
    }
    #[instrument(skip(payload))]
    async fn send_with_timeout(&mut self, payload:&[u8], max_time:u16)->Result<Vec<u8>> {
        event!(Level::TRACE, "Payload: {}", String::from_utf8(payload.to_vec())?);
        let mut buf:Vec<u8> = Vec::default();
        for try_count in 1u8..4u8 {
            event!(Level::DEBUG, "Sending datagram to {}", self.target);
            self.inner.connect(self.target).await.wrap_err("while connecting socket")?;
            self.inner.send(&payload).await.wrap_err("while sending datagram")?;
            event!(Level::DEBUG, "Waiting for response from {}", self.target);
            match timeout(Duration::from_millis(max_time as u64), self.inner.recv_buf(&mut buf)).await {
                Ok(rec) => {
                    rec.wrap_err(format!("While receiving response from {}", self.target))?;
                    event!(Level::DEBUG, "Request succeeded");
                    break;
                },
                Err(e) => {
                    event!(Level::DEBUG, "Attempt {try_count} failed: {e}");
                }
            }
        }
        if buf.len() == 0 {
            return Err(eyre!("No data received from {}", self.target));
        }

        Ok(buf)
    }

    #[instrument]
    pub async fn probe(&mut self)->Result<Vec<u8>> {
        Ok(self.send_with_timeout(
            ONVIF_DISCOVER_UNAUTHED_TEMPLATE.replace("$UUID$", &Uuid::new_v4().to_string()).as_bytes(),
            3000
        ).await.wrap_err("While sending probe")?)
    }
}
