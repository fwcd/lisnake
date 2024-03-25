use lighthouse_client::protocol::{Frame, Pos};
use tracing::info;

use crate::constants::{FRUIT_COLOR, SNAKE_INITIAL_LENGTH};

use super::Snake;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct State {
    pub snake: Snake,
    pub fruit: Pos,
}

impl State {
    pub fn new() -> Self {
        let snake = Snake::from_initial_length(SNAKE_INITIAL_LENGTH);
        let fruit = snake.random_fruit_pos().unwrap();
        Self { snake, fruit }
    }
    
    pub fn reset(&mut self) {
        *self = Self::new();
    }

    pub fn step(&mut self) {
        self.snake.step();

        if self.snake.head() == self.fruit {
            self.snake.grow();
            let length = self.snake.len();
            info! { %length, "Snake grew" };
            if let Some(fruit) = self.snake.random_fruit_pos() {
                self.fruit = fruit;
            } else {
                info!("You win!");
                self.reset();
            }
        } else if self.snake.intersects_itself() {
            info!("Game over!");
            self.reset();
        }
    }

    pub fn render(&self) -> Frame {
        let mut frame = Frame::empty();

        frame[self.fruit] = FRUIT_COLOR;
        self.snake.render_to(&mut frame);

        frame
    }
}
