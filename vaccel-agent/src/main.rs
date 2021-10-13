use std::error::Error;
use std::path::PathBuf;
use std::sync::mpsc;
use std::thread;

use structopt::StructOpt;

use tokio::net::UnixListener;
use tokio_serde::formats::Json;
use tokio_util::codec::{Framed, LengthDelimitedCodec};

use tarpc::serde_transport;
use tarpc::server::{BaseChannel, Channel};

use vaccel::server::{Server, VaccelAPI};

use log::{debug, error};

extern crate signal_hook;

mod cli;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let cli = cli::AgentCli::from_args();

    debug!("Opening API socket at {}", cli.uri);
    let unix_socket_path = PathBuf::from(cli.uri);
    let listener = UnixListener::bind(unix_socket_path)?;
    loop {
        match listener.accept().await {
            Ok((stream, addr)) => {
                debug!("New client at {:?}", addr);
                let transport = serde_transport::new(
                    Framed::new(stream, LengthDelimitedCodec::new()),
                    Json::default(),
                );

                let server = BaseChannel::with_defaults(transport);
                tokio::spawn(server.execute(Server::new()?.serve()));
            }
            Err(e) => {
                error!("Error while connecting to client: {}", e);
                break;
            }
        }
    }

    Ok(())
}
