use std::{collections::HashMap, sync::Arc};

use anyhow::Result;
use futures::{lock::Mutex, prelude::*, Stream};
use lighthouse_client::protocol::{EventSource, InputEvent, ServerMessage};
use tracing::{debug, info};

use crate::model::State;

pub async fn run(mut stream: impl Stream<Item = lighthouse_client::Result<ServerMessage<InputEvent>>> + Unpin, shared_state: Arc<Mutex<State>>) -> Result<()> {
    let mut mapped_players: HashMap<EventSource, usize> = HashMap::new();

    while let Some(msg) = stream.next().await {
        let input_event = msg?.payload;
        let source = input_event.source();

        // Map the player if needed to a snake
        if !mapped_players.contains_key(&source) {
            let i = mapped_players.len();
            info!("Mapped new player {} to snake {}", &source, i + 1);
            mapped_players.insert(source.clone(), i);

            let mut state = shared_state.lock().await;
            state.ensure_snakes(mapped_players.len());
        }

        let i = mapped_players[&source];

        // Update the snake's direction
        if let Some(dir) = input_event.direction() {
            debug!("Rotating snake head to {:?}", dir);
            let mut state = shared_state.lock().await;
            state.snake_mut(i).rotate_head(dir.into());
        }
    }

    Ok(())
}
