use std::sync::Arc;

use anyhow::Result;
use futures::{lock::Mutex, prelude::*, Stream};
use lighthouse_client::protocol::{Delta, Model, ServerMessage};
use tracing::debug;

use crate::model::State;

pub async fn run(mut stream: impl Stream<Item = lighthouse_client::Result<ServerMessage<Model>>> + Unpin, shared_state: Arc<Mutex<State>>) -> Result<()> {
    while let Some(msg) = stream.next().await {
        if let Model::InputEvent(event) = msg?.payload {
            if event.is_down {
                // Map the key code to a direction vector
                let opt_dir = match event.key {
                    Some(37) => Some(Delta::LEFT),
                    Some(38) => Some(Delta::UP),
                    Some(39) => Some(Delta::RIGHT),
                    Some(40) => Some(Delta::DOWN),
                    _ => None,
                };

                // Update the snake's direction
                if let Some(dir) = opt_dir {
                    debug!("Rotating snake head to {:?}", dir);
                    let mut state = shared_state.lock().await;
                    state.snake.rotate_head(dir);
                }
            }
        }
    }

    Ok(())
}
