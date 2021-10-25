use std::path::PathBuf;

use tarpc::serde_transport;
use tarpc::server::{BaseChannel, Channel};
use tarpc::transport::channel;
use tarpc::{client, context};

use tokio::net::UnixStream;
use tokio_serde::formats::Json;
use tokio_util::codec::{Framed, LengthDelimitedCodec};
use tokio_vsock::VsockStream;

use crate::server::{Server, VaccelAPI, VaccelAPIClient};
use crate::session::Session;
use crate::Result;

pub enum VaccelConfig {
    /// In-memory handling of vAccel requests
    Local,
    /// Request handling over a vsock socket
    Vsock(u32, u32),
    /// Request handling over a UNIX socket
    Unix(PathBuf),
}

#[derive(Debug, Clone)]
pub struct Vaccel {
    inner: VaccelAPIClient,
}

impl Vaccel {
    pub async fn new(config: VaccelConfig) -> Result<Self> {
        match config {
            VaccelConfig::Local => {
                let (client_transport, server_transport) = channel::unbounded();
                let server = BaseChannel::with_defaults(server_transport);
                tokio::spawn(server.execute(Server::new()?.serve()));

                Ok(Self {
                    inner: VaccelAPIClient::new(client::Config::default(), client_transport)
                        .spawn(),
                })
            }
            VaccelConfig::Vsock(cid, port) => {
                let stream = VsockStream::connect(cid, port).await?;
                let transport = serde_transport::new(
                    Framed::new(stream, LengthDelimitedCodec::new()),
                    Json::default(),
                );

                Ok(Self {
                    inner: VaccelAPIClient::new(client::Config::default(), transport).spawn(),
                })
            }
            VaccelConfig::Unix(path) => {
                let stream = UnixStream::connect(path).await?;
                let transport = serde_transport::new(
                    Framed::new(stream, LengthDelimitedCodec::new()),
                    Json::default(),
                );

                Ok(Self {
                    inner: VaccelAPIClient::new(client::Config::default(), transport).spawn(),
                })
            }
        }
    }

    pub async fn new_session(&self) -> Result<Session> {
        let id = self.inner.new_session(context::current()).await??;
        Ok(Session::new().with_id(id))
    }

    pub async fn destroy_session(&self, session: &Session) -> Result<()> {
        self.inner
            .destroy_session(context::current(), session.id())
            .await?
    }

    pub async fn tf_session_load(&self, model_id: u64) -> Result<()> {
        self.inner
            .tf_session_load(context::current(), model_id)
            .await?
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn basic_client_session() {
        let client = Vaccel::new(VaccelConfig::Local)
            .await
            .expect("Could not create Server");

        let session = client
            .new_session()
            .await
            .expect("Could not create session");

        assert_eq!(session.id(), 1);
    }
}
