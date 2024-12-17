use super::grid::Move;
use super::Environment;
use rust_ml_toolkit::rand;

pub struct Agent;

impl Agent {
    pub fn select_action() -> Move {
        rand::random::<Move>()
    }
}
