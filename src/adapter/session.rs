#![allow(unused)]
use crate::core::nn::{dataset::DataSet, nn::NN};

enum TrainingMethod {
    FiniteDiff { rate: f64, eps: f64 },
    BackProp,
}
enum PostX {
    Sigmoid,
}

struct SessionOption {
    train_method: TrainingMethod,
    post_x: PostX,
}

pub struct Session {
    /// neural network
    model: NN,
    /// training data,
    dataset: DataSet<f64>,
    /// option of session
    option: SessionOption,
}

impl Session {
    /// train model with current setup and dataset
    fn train(&mut self) {
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
    pub fn train_ntimes(&mut self, n: usize) {
        for _ in 0..n {
            self.train();
        }
    }
}
