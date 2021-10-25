use env_logger::Env;
use log::info;

use vaccel::client::{Vaccel, VaccelConfig};

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();

    info!("Connecting to vsock://2:2048");
    let client = Vaccel::new(VaccelConfig::Vsock(2, 2048))
        .await
        .expect("Could not create client");

    let session = client
        .new_session()
        .await
        .expect("Could not create session");
    assert_eq!(session.id(), 1);
    info!("New session: {}", session.id());

    let session = client
        .new_session()
        .await
        .expect("Could not create session");
    assert_eq!(session.id(), 2);
    info!("New session: {}", session.id());
}
