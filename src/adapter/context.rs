#![allow(unused)]
use std::sync::mpsc::{Receiver, Sender};

use crate::core::nn::{dataset::DataSet, nn::NN};

use super::{
    learner::{G2w, W2g},
    nodes::Nodes,
    session::Session,
};

#[derive(PartialEq, Debug, Clone)]
pub enum State {
    Empty,
    Loading,
    Ready,
    Running,
}

pub struct Transceiver {
    pub snd: Sender<G2w>,
    pub rec: Receiver<W2g>,
}

pub struct Context {
    /// session data contains model, dataset, options
    pub session: Option<Session>,
    /// state of nn,
    pub state: State,
    /// view
    pub nodes: Option<Nodes>,
    /// transceiver between learnner thread
    pub trcv: Option<Transceiver>,
    /// costs history
    pub costs: Vec<f64>,
}

impl Clone for Context {
    fn clone(&self) -> Self {
        Self {
            session: self.session.clone(),
            state: self.state.clone(),
            nodes: self.nodes.clone(),
            costs: self.costs.clone(),
            trcv: None,
        }
    }
}

impl Default for Context {
    fn default() -> Self {
        Self {
            state: State::Empty,
            costs: Vec::new(),
            session: None,
            nodes: None,
            trcv: None,
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
