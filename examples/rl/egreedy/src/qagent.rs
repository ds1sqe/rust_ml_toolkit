use std::ops::{AddAssign, SubAssign};

use super::grid::Move;
use super::Environment;

use rust_ml_toolkit::core::matrix::{Matrix, MatrixOps};
use rust_ml_toolkit::rand::{self, Rng};

#[derive(Clone, Default)]
pub struct ActionValues([f32; 4]);

impl ActionValues {
    pub fn max(&self) -> Move {
        let x = self
            .0
            .iter()
            .enumerate()
            .max_by(|(_, x), (_, y)| x.total_cmp(y))
            .expect("cannot get max");
        match x.0 {
            0 => Move::Right,
            1 => Move::Left,
            2 => Move::Up,
            3 => Move::Down,
            _ => unreachable!(),
        }
    }
}

pub struct QAgent {
    q_table: Matrix<ActionValues>,
    eps: f32,
    alpha: f32,
}

#[derive(Clone)]
pub struct State {
    pub x: usize,
    pub y: usize,
}

#[derive(Clone)]
pub struct Transition {
    pub state: State,
    pub act: Move,
    pub reward: i64,
}

impl AddAssign for ActionValues {
    fn add_assign(&mut self, rhs: ActionValues) {
        let rhs_iter = rhs.0.into_iter();
        let _ = self.0.iter_mut().zip(rhs_iter).map(|(left, right)| {
            *left += right;
        });
    }
}

impl SubAssign for ActionValues {
    fn sub_assign(&mut self, rhs: ActionValues) {
        let rhs_iter = rhs.0.into_iter();
        let _ = self.0.iter_mut().zip(rhs_iter).map(|(left, right)| {
            *left -= right;
        });
    }
}

impl QAgent {
    pub fn build() -> Self {
        let q_table = Matrix::new(5, 7);
        let eps = 0.8;
        let alpha = 0.01;
        Self {
            q_table,
            eps,
            alpha,
        }
    }

    pub fn select_action(&self, state: (usize, usize)) -> Move {
        let mut rng = rand::thread_rng();
        let do_rand = rng.gen_range(0.0..1.0) < self.eps;
        if do_rand {
            rand::random::<Move>()
        } else {
            self.q_table.at(state.1, state.0).max()
        }
    }

    pub fn update_table(&mut self, history: Vec<Transition>) {
        let mut cum_reward = 0;
        for tr in history.iter() {
            let idx = tr.act.idx();
            let tgt = self.q_table.at_mut(tr.state.y, tr.state.x);
            let record = tgt.clone();
            tgt.0[idx] += self.alpha * (cum_reward as f32 - record.0[idx]);
            cum_reward += tr.reward;
        }
    }

    pub fn anneal_eps(&mut self) {
        self.eps -= 0.01;
        self.eps = self.eps.max(0.1);
    }

    pub fn show_table(&self) {
        for row_idx in 0..self.q_table.len_row() {
            let row = self.q_table.row(row_idx);
            let mut line_buffer = String::new();
            line_buffer.push('|');
            for c in row {
                let max = c.max();
                let x = match max {
                    Move::Right => "➡️",
                    Move::Left => "⬅️",
                    Move::Up => "⬆️",
                    Move::Down => "⬇️",
                };
                line_buffer.push_str(x);
                line_buffer.push('|');
            }
            println!("{}", line_buffer);
        }
    }
}
