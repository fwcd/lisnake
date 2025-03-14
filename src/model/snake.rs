use std::collections::{HashSet, VecDeque};

use lighthouse_client::protocol::{Color, Delta, Frame, Pos, LIGHTHOUSE_RECT};

use crate::constants::SNAKE_INITIAL_LENGTH;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Snake {
    fields: VecDeque<Pos<i32>>,
    dir: Delta<i32>,
    color: Color,
}

impl Snake {
    pub fn new(color: Color) -> Self {
        Self::with_length(SNAKE_INITIAL_LENGTH, color)
    }

    pub fn with_length(length: usize, color: Color) -> Self {
        let mut pos: Pos<i32> = LIGHTHOUSE_RECT.sample_random().unwrap();
        let dir = Delta::random_cardinal();

        let mut fields = VecDeque::new();
        for _ in 0..length {
            fields.push_back(pos);
            pos = LIGHTHOUSE_RECT.wrap(pos - dir);
        }

        Self { fields, dir, color }
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
        self.field_set().len() < self.fields.len()
    }

    pub fn contains(&self, pos: Pos<i32>) -> bool {
        self.fields.contains(&pos)
    }

    pub fn rotate_head(&mut self, dir: Delta<i32>) {
        self.dir = dir;
    }

    pub fn render_to(&self, frame: &mut Frame) {
        for field in &self.fields {
            frame[*field] = self.color;
        }
    }

    pub fn len(&self) -> usize {
        self.fields.len()
    }

    pub fn fields(&self) -> &VecDeque<Pos<i32>> {
        &self.fields
    }

    pub fn field_set(&self) -> HashSet<Pos<i32>> {
        self.fields.iter().cloned().collect::<HashSet<_>>()
    }

    pub fn color(&self) -> Color {
        self.color
    }
}
