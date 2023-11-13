#![allow(unused)]
use crate::core::nn::{dataset::DataSet, nn::NN};

use super::session::Session;

enum State {
    Empty,
    Loading,
    Ready,
    Running,
}

struct Context {
    /// session data contains model, dataset, options
    session: Option<Session>,
    /// state of nn,
    state: State,
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
