use std::collections::{HashSet, VecDeque};

use lighthouse_client::protocol::{Delta, Frame, Pos, LIGHTHOUSE_RECT, LIGHTHOUSE_SIZE};

use crate::constants::SNAKE_COLOR;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Snake {
    fields: VecDeque<Pos<i32>>,
    dir: Delta<i32>,
}

impl Snake {
    pub fn from_initial_length(length: usize) -> Self {
        let mut pos: Pos<i32> = LIGHTHOUSE_RECT.sample_random().unwrap();
        let dir = Delta::random_cardinal();

        let mut fields = VecDeque::new();
        for _ in 0..length {
            fields.push_back(pos);
            pos = LIGHTHOUSE_RECT.wrap(pos - dir);
        }

        Self { fields, dir }
    }

    pub fn head(&self) -> Pos<i32> { *self.fields.front().unwrap() }

    pub fn back(&self) -> Pos<i32> { *self.fields.back().unwrap() }

    pub fn grow(&mut self) {
        self.fields.push_back(LIGHTHOUSE_RECT.wrap(self.back() - self.dir));
    }

    pub fn step(&mut self) {
        let head = self.head();
        self.fields.pop_back();
        self.fields.push_front(LIGHTHOUSE_RECT.wrap(head + self.dir));
    }

    pub fn intersects_itself(&self) -> bool {
        self.fields.iter().collect::<HashSet<_>>().len() < self.fields.len()
    }

    pub fn rotate_head(&mut self, dir: Delta<i32>) {
        self.dir = dir;
    }

    pub fn render_to(&self, frame: &mut Frame) {
        for field in &self.fields {
            frame[*field] = SNAKE_COLOR;
        }
    }

    pub fn len(&self) -> usize {
        self.fields.len()
    }

    pub fn random_fruit_pos(&self) -> Option<Pos<i32>> {
        let fields = self.fields.iter().collect::<HashSet<_>>();
        if fields.len() >= LIGHTHOUSE_SIZE {
            None
        } else {
            loop {
                let pos = LIGHTHOUSE_RECT.sample_random().unwrap();
                if !fields.contains(&pos) {
                    break Some(pos);
                }
            }
        }
    }
}
