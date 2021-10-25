use env_logger::Env;
use log::{error, info};

use vaccel::client::{Vaccel, VaccelConfig};

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();

    info!("Creating client");
    let client = Vaccel::new(VaccelConfig::Local)
        .await
        .expect("Could not create client");

    match client.tf_session_load(1).await {
        Ok(()) => info!("Loaded model"),
        Err(e) => error!("{}", e),
    }
}
