#![allow(unused)]
use crate::core::nn::{dataset::DataSet, nn::NN};

use super::{nodes::Nodes, session::Session};

#[derive(PartialEq, Debug, Clone)]
pub enum State {
    Empty,
    Loading,
    Ready,
    Running,
}

#[derive(Clone)]
pub struct Context {
    /// session data contains model, dataset, options
    pub session: Option<Session>,
    /// state of nn,
    pub state: State,
    /// view
    pub nodes: Option<Nodes>,
}

impl Default for Context {
    fn default() -> Self {
        Self {
            session: None,
            state: State::Empty,
            nodes: None,
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
