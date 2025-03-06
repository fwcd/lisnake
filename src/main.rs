use anyhow::Result;
use clap::Parser;
use futures::lock::Mutex;
use lighthouse_client::{protocol::Authentication, Lighthouse, LIGHTHOUSE_URL};
use tracing::info;
use tokio::task;
use std::sync::Arc;

use crate::model::State;

mod constants;
mod controller;
mod model;
mod updater;

#[derive(Parser)]
#[command(version)]
struct Args {
    /// The username.
    #[arg(short, long, env = "LIGHTHOUSE_USER")]
    username: String,
    /// The API token.
    #[arg(short, long, env = "LIGHTHOUSE_TOKEN")]
    token: String,
    /// The server URL.
    #[arg(long, env = "LIGHTHOUSE_URL", default_value = LIGHTHOUSE_URL)]
    url: String,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().init();
    _ = dotenvy::dotenv();

    let args = Args::parse();
    let auth = Authentication::new(&args.username, &args.token);
    let state = Arc::new(Mutex::new(State::new()));

    let lh = Lighthouse::connect_with_tokio_to(&args.url, auth).await?;
    info!("Connected to the Lighthouse server");

    let input = lh.stream_input().await?;

    let updater_handle = task::spawn(updater::run(lh, state.clone()));
    let controller_handle = task::spawn(controller::run(input, state));

    updater_handle.await??;
    controller_handle.await??;

    Ok(())
}
