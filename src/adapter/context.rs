#![allow(unused)]
use crate::core::nn::{dataset::DataSet, nn::NN};

use super::session::Session;

#[derive(PartialEq, Debug)]
pub enum State {
    Empty,
    Loading,
    Ready,
    Running,
}

pub struct Context {
    /// session data contains model, dataset, options
    pub session: Option<Session>,
    /// state of nn,
    pub state: State,
}

impl Default for Context {
    fn default() -> Self {
        Self {
            session: None,
            state: State::Empty,
        }
    }
}

impl Context {
    /// load model
    fn open() {}
    /// save model
    fn save() {}
    /// load training data;
    fn load_dataset() {}
    /// start training
    fn start(&self) {}
    /// stop ""
    fn stop() {}
    fn terminate() {}
}
