use std::sync::Arc;

use anyhow::Result;
use futures::{lock::Mutex, prelude::*, Stream};
use lighthouse_client::protocol::{InputEvent, ServerMessage};
use tracing::debug;

use crate::model::State;

pub async fn run(mut stream: impl Stream<Item = lighthouse_client::Result<ServerMessage<InputEvent>>> + Unpin, shared_state: Arc<Mutex<State>>) -> Result<()> {
    while let Some(msg) = stream.next().await {
        let input_event = msg?.payload;
        // Update the snake's direction
        if let Some(dir) = input_event.direction() {
            debug!("Rotating snake head to {:?}", dir);
            let mut state = shared_state.lock().await;
            state.snake.rotate_head(dir.into());
        }
    }

    Ok(())
}
