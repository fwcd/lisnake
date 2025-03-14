use std::time::Duration;

use lighthouse_client::protocol::Color;

pub const UPDATE_INTERVAL: Duration = Duration::from_millis(200);
pub const FRUIT_COLOR: Color = Color::RED;
pub const SNAKE_COLORS: [Color; 7] = [Color::GREEN, Color::YELLOW, Color::CYAN, Color::MAGENTA, Color::BLUE, Color::WHITE, Color::GRAY];
pub const SNAKE_INITIAL_LENGTH: usize = 3;
