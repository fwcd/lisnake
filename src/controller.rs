use std::sync::Arc;

use anyhow::Result;
use futures::{lock::Mutex, prelude::*, Stream};
use lighthouse_client::protocol::{Delta, GamepadButtonEvent, GamepadControlEvent, GamepadEvent, InputEvent, KeyEvent, ServerMessage};
use tracing::debug;

use crate::model::State;

pub async fn run(mut stream: impl Stream<Item = lighthouse_client::Result<ServerMessage<InputEvent>>> + Unpin, shared_state: Arc<Mutex<State>>) -> Result<()> {
    while let Some(msg) = stream.next().await {
        let opt_dir = match msg?.payload {
            InputEvent::Key(KeyEvent { code, down, .. }) if down => match code.as_str() {
                "ArrowLeft" => Some(Delta::LEFT),
                "ArrowUp" => Some(Delta::UP),
                "ArrowRight" => Some(Delta::RIGHT),
                "ArrowDown" => Some(Delta::DOWN),
                _ => None,
            }
            InputEvent::Gamepad(GamepadEvent { control, .. }) => match control {
                GamepadControlEvent::Button(GamepadButtonEvent { index, down, .. }) if down => match index {
                    // Per https://w3c.github.io/gamepad/#remapping
                    12 => Some(Delta::UP),
                    13 => Some(Delta::DOWN),
                    14 => Some(Delta::LEFT),
                    15 => Some(Delta::RIGHT),
                    _ => None,
                },
                _ => None,
            }
            _ => None,
        };

        // Update the snake's direction
        if let Some(dir) = opt_dir {
            debug!("Rotating snake head to {:?}", dir);
            let mut state = shared_state.lock().await;
            state.snake.rotate_head(dir);
        }
    }

    Ok(())
}
