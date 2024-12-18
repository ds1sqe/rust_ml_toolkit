use super::Environment;
use rust_ml_toolkit::rand::{
    distributions::{Distribution, Standard},
    Rng,
};

#[derive(Clone)]
pub struct GridWorld {
    x: usize,
    x_max: usize,
    y: usize,
    y_max: usize,
}

pub enum Move {
    Right,
    Left,
    Up,
    Down,
}

impl Distribution<Move> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Move {
        let index: u8 = rng.gen_range(0..3);
        match index {
            0 => Move::Right,
            1 => Move::Left,
            2 => Move::Up,
            3 => Move::Down,
            _ => unreachable!(),
        }
    }
}

impl GridWorld {
    pub fn new(x_max: usize, y_max: usize) -> Self {
        Self {
            x: 0,
            x_max,
            y: 0,
            y_max,
        }
    }
}

impl Environment for GridWorld {
    type Action = Move;
    type Reward = i64;
    type State = (usize, usize);

    fn step(&mut self, action: Self::Action) -> Self::Reward {
        match action {
            Move::Right => self.move_right(),
            Move::Left => self.move_left(),
            Move::Up => self.move_up(),
            Move::Down => self.move_down(),
        }
        -1
    }

    fn state(&self) -> Self::State {
        (self.x, self.y)
    }

    fn is_done(&self) -> bool {
        self.x == self.x_max && self.y == self.y_max
    }

    fn reset(&mut self) {
        self.x = 0;
        self.y = 0;
    }
}

impl GridWorld {
    fn move_right(&mut self) {
        if self.x_max > self.x {
            self.x += 1;
        }
    }
    fn move_left(&mut self) {
        if 0 < self.x {
            self.x -= 1;
        }
    }
    fn move_up(&mut self) {
        if self.y_max > self.y {
            self.y += 1;
        }
    }
    fn move_down(&mut self) {
        if 0 < self.y {
            self.y -= 1;
        }
    }
}
