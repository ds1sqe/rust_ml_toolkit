#![allow(unused)]
use std::{
    path::Path,
    sync::mpsc::{channel, Receiver, Sender},
};

use crate::core::nn::{dataset::DataSet, nn::NN};

use super::{
    data::{Readable, Savable},
    learner::{spawn_learner, ControlSignal, G2w, W2g},
    nodes::Nodes,
    session::{PostX, Session, SessionOption, TrainingMethod},
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
    /// create model
    fn create_model(
        &mut self,
        layers: &[usize],
        train_method: TrainingMethod,
        post_x: PostX,
        cycle: usize,
    ) -> Self {
        let session = Session {
            model: NN::new(layers),
            dataset: None,
            option: SessionOption {
                train_method,
                post_x,
                cycle,
            },
        };
        Context {
            session: Some(session),
            state: State::Loading,
            nodes: Some(Nodes::from(&session.model)),
            trcv: None,
            costs: Vec::new(),
        }
    }
    /// load session from saved file
    fn load_session(path: &Path) -> Option<Self> {
        let session = Session::read(path);
        if session.is_none() {
            return None;
        };
        let state = if session.unwrap().dataset.is_none() {
            State::Loading
        } else {
            State::Ready
        };

        Some(Context {
            session,
            state,
            nodes: Some(Nodes::from(&session.unwrap().model)),
            trcv: None,
            costs: Vec::new(),
        })
    }
    /// save model
    fn save_session(&self, path: &Path) -> Option<bool> {
        if self.session.is_none() {
            println!("Context>>save_session: Session is None");
            None
        } else {
            Session::save(&self.session.unwrap(), path)
        }
    }
    /// attach data set to self.session
    fn attach_dataset(&mut self, data_set: DataSet<f64>) {
        if self.session.is_none() {
            println!("Context>>attach_dataset: Session is None");
        } else {
            self.session.unwrap().dataset = Some(data_set);
        }
    }
    /// load training data and attach to self.session;
    fn load_dataset(&mut self, path: &Path) {
        if self.session.is_none() {
            println!("Context>>load_dataset: Session is None");
        } else {
            let dataset = DataSet::read(path);
            match dataset {
                None => {
                    println!(
                        "Context>>load_dataset: dataset is None 
                    (faild to load dataset)"
                    );
                }
                Some(dataset) => {
                    self.session.unwrap().dataset = Some(dataset);
                }
            }
        }
    }
    /// save context's session's dataset to given path
    fn save_dataset(&mut self, path: &Path) -> Option<bool> {
        if self.session.is_none() {
            println!("Context>>save_dataset: Session is None");
            return None;
        } else {
            match self.session.unwrap().dataset {
                None => {
                    println!("Context>>save_dataset: dataset is None");
                    return None;
                }
                Some(dataset) => DataSet::save(&dataset, path),
            }
        }
    }
    /// start training (spawn_learner)
    fn start(&mut self) {
        match self.state {
            State::Loading | State::Empty => {
                println!("Context>>start: Not Readied");
            }
            State::Running => {
                println!("Context>>start: Already Running");
            }
            State::Ready => {
                let (snd, rx) = channel();
                let rec = spawn_learner(self.session.clone().unwrap(), rx);
                self.state = State::Running;
                self.trcv = Some(Transceiver { snd, rec })
            }
        }
    }
    /// stop training
    fn stop(&mut self) {
        match self.state {
            State::Running => {
                let res = self.trcv.as_ref().unwrap().snd.send(G2w {
                    sig: ControlSignal::Stop,
                });
                self.state = State::Ready;
            }
            _ => {}
        }
    }
    fn terminate(&mut self) {}
}
