use std::collections::HashSet;

use lighthouse_client::protocol::{Frame, Pos, LIGHTHOUSE_COLS, LIGHTHOUSE_ROWS};
use rand::seq::IndexedRandom;
use tracing::info;

use crate::constants::FRUIT_COLOR;

use super::Snake;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct State {
    snakes: Vec<Snake>,
    fruit: Pos<i32>,
}

impl State {
    pub fn new() -> Self {
        let snakes = vec![Snake::new()];
        let fruit = Self::random_fruit_pos(&snakes).unwrap();
        Self { snakes, fruit }
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }

    pub fn respawn(&mut self, i: usize) {
        // TODO: Be smarter about this, i.e. avoid intersecting another snake or the fruit
        self.snakes[i] = Snake::new();
    }

    fn random_fruit_pos(snakes: &[Snake]) -> Option<Pos<i32>> {
        let occupied = snakes.iter().flat_map(|s| s.fields()).collect::<HashSet<_>>();
        let free = (0..LIGHTHOUSE_ROWS)
            .flat_map(|y| (0..LIGHTHOUSE_COLS).map(move |x| Pos::new(x as i32, y as i32)))
            .filter(|pos| !occupied.contains(pos))
            .collect::<Vec<Pos<i32>>>();
        return free.choose(&mut rand::rng()).cloned();
    }

    pub fn step(&mut self) {
        self.step_snakes();
        self.check_self_collisions();
        self.check_collisions();
        self.check_fruits();
    }

    fn step_snakes(&mut self) {
        for snake in &mut self.snakes {
            snake.step();
        }
    }

    fn check_self_collisions(&mut self) {
        if let Some(i) = 'outer: {
            for (i, snake) in self.snakes.iter_mut().enumerate() {
                if snake.intersects_itself() {
                    break 'outer Some(i);
                }
            }
            None
        } {
            info!("Snake {} died!", i + 1);
            self.respawn(i);
        }
    }

    fn check_collisions(&mut self) {
        if let Some(loser) = 'outer: {
            for i in 0..self.snakes.len() {
                for j in (i + 1)..self.snakes.len() {
                    let snake1 = &self.snakes[i];
                    let snake2 = &self.snakes[j];

                    break 'outer if snake1.head() == snake2.head() {
                        // Decide randomly which snake dies
                        Some(if rand::random() { i } else { j })
                    } else if snake1.contains(snake2.head()) {
                        Some(j) // Snake 2 dies
                    } else if snake2.contains(snake1.head()) {
                        Some(i) // Snake 1 dies
                    } else {
                        None
                    };
                }
            }
            None
        } {
            info!("Snake {} was killed!", loser + 1);
            self.respawn(loser);
        }
    }

    fn check_fruits(&mut self) {
        if let Some((i, snake)) = self.snakes.iter_mut().enumerate().find(|(_, s)| s.head() == self.fruit) {
            snake.grow();
            let length = snake.len();
            info! { %length, "Snake {} grew", i + 1 };
            if let Some(fruit) = Self::random_fruit_pos(&self.snakes) {
                self.fruit = fruit;
            } else {
                info!("Snake {} wins!", i + 1);
                self.reset();
            }
        }
    }

    pub fn render(&self) -> Frame {
        let mut frame = Frame::empty();

        frame[self.fruit] = FRUIT_COLOR;
        for snake in &self.snakes {
            snake.render_to(&mut frame);
        }

        frame
    }

    pub fn ensure_snakes(&mut self, count: usize) {
        while self.snakes.len() < count {
            // TODO: Be smarter about this, i.e. avoid intersecting another snake or the fruit
            self.snakes.push(Snake::new());
        }
    }

    pub fn snake_mut(&mut self, i: usize) -> &mut Snake {
        &mut self.snakes[i]
    }
}
