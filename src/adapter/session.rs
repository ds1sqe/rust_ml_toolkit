#![allow(unused)]
use crate::core::nn::{dataset::DataSet, nn::NN};

#[derive(Clone)]
pub enum TrainingMethod {
    FiniteDiff { rate: f64, eps: f64 },
    BackProp,
}
#[derive(Clone)]
pub enum PostX {
    Sigmoid,
}

#[derive(Clone)]
pub struct SessionOption {
    pub train_method: TrainingMethod,
    pub post_x: PostX,
}

#[derive(Clone)]
pub struct Session {
    /// neural network
    pub model: NN,
    /// training data,
    pub dataset: DataSet<f64>,
    /// option of session
    pub option: SessionOption,
}

impl Session {
    /// train model with current setup and dataset
    pub fn train(&mut self) {
        let inputs = self.dataset.inputs.clone();
        let expects = self.dataset.outputs.clone();

        let delta = match self.option.train_method {
            TrainingMethod::FiniteDiff { rate, eps } => {
                let mut delta = self.model.finite_diff(&inputs, &expects, &eps);
                delta.mul(&rate);
                delta
            }
            TrainingMethod::BackProp => self.model.backprop(&inputs, &expects),
        };

        self.model.learn(&delta)
    }
    /// train model n times
    pub fn train_ntimes(&mut self, n: usize) {
        for _ in 0..n {
            self.train();
        }
    }
}
