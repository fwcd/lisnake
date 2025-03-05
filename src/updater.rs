use std::sync::Arc;

use anyhow::Result;
use futures::lock::Mutex;
use lighthouse_client::{Lighthouse, TokioWebSocket};
use tokio::time;
use tracing::debug;

use crate::{constants::UPDATE_INTERVAL, model::State};

pub async fn run(lh: Lighthouse<TokioWebSocket>, shared_state: Arc<Mutex<State>>) -> Result<()> {
    loop {
        // Update the snake and render it
        let frame = {
            let mut state = shared_state.lock().await;
            state.step();
            state.render()
        };

        // Send the rendered snake to the lighthouse
        lh.put_model(frame).await?;
        debug!("Sent frame");

        // Wait for a short period of time
        time::sleep(UPDATE_INTERVAL).await;
    }
}
