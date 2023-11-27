#![allow(unused)]
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::Path;

use crate::core::nn::{dataset::DataSet, nn::NN};
use serde::Deserialize;
use serde::Serialize;

use super::data::Buildable;
use super::data::Readable;
use super::data::Savable;
use super::data::Stringfiable;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TrainingMethod {
    FiniteDiff { rate: f64, eps: f64 },
    BackProp,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PostX {
    Sigmoid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionOption {
    pub train_method: TrainingMethod,
    pub post_x: PostX,
    pub cycle: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    /// neural network
    pub model: NN,
    /// training data,
    pub dataset: Option<DataSet<f64>>,
    /// option of session
    pub option: SessionOption,
}

impl Session {
    /// train model with current setup and dataset
    pub fn train_single(&mut self) {
        match self.dataset.clone() {
            None => {
                println!("Session>>train_single: Dataset is None")
            }
            Some(ds) => {
                let inputs = &ds.inputs;
                let expects = &ds.outputs;

                let delta = match self.option.train_method {
                    TrainingMethod::FiniteDiff { rate, eps } => {
                        let mut delta =
                            self.model.finite_diff(&inputs, &expects, &eps);
                        delta.mul(&rate);
                        delta
                    }
                    TrainingMethod::BackProp => {
                        self.model.backprop(&inputs, &expects)
                    }
                };

                self.model.learn(&delta)
            }
        }
    }
    /// train model n times
    pub fn train_ntimes(&mut self, n: usize) {
        for _ in 0..n {
            self.train_single();
        }
    }
    /// train model self.cycle times
    pub fn train(&mut self) {
        for _ in 0..self.option.cycle {
            self.train_single();
        }
    }

    pub fn cost(&mut self) -> f64 {
        match self.dataset.clone() {
            None => {
                // HACK: change return type (f64->Option f64)
                println!("Session>>train_single: Dataset is None");
                return 0.0;
            }
            Some(ds) => {
                let inputs = &ds.inputs;
                let expects = &ds.outputs;
                return self.model.cost(inputs, expects);
            }
        }
    }
}

impl Stringfiable for Session {
    type Struct = Session;
    fn stringfy(src: &Self::Struct) -> Option<String> {
        let output = serde_json::to_string_pretty(&src);
        if output.is_ok() {
            return Some(output.unwrap());
        }
        None
    }
}

impl Buildable for Session {
    type Struct = Session;
    fn build(str: String) -> Option<Self::Struct> {
        let cloned = str.clone();
        let ss = serde_json::from_str(&cloned);
        if ss.is_ok() {
            return ss.unwrap();
        }
        None
    }
}

impl Savable for Session {
    type Struct = Session;
    fn save(data: &Self::Struct, path: &Path) -> Option<bool> {
        let mut file = match File::create(path) {
            Err(e) => panic!("could not create at {}: {}", path.display(), e),
            Ok(file) => file,
        };
        let str = Session::stringfy(data);
        if str.is_none() {
            return None;
        }
        let flag = file.write_all(str.unwrap().as_bytes()).is_ok();
        if flag {
            return Some(true);
        }
        return None;
    }
}

impl Readable for Session {
    type Struct = Session;

    fn read(path: &Path) -> Option<Self::Struct> {
        let mut file = match File::open(path) {
            Err(e) => panic!("could not open {}: {}", path.display(), e),
            Ok(file) => file,
        };
        let mut buf = String::new();
        let flag = file.read_to_string(&mut buf);
        if flag.is_err() {
            return None;
        }

        let ss = Session::build(buf.to_string());
        if ss.is_none() {
            return None;
        }
        ss
    }
}
