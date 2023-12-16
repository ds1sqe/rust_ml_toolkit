use serde::Deserialize;
use serde::Serialize;

use crate::core::matrix::matrix::Matrix;
use crate::core::matrix::matrix::__Matrix;
use crate::core::nn::cost::CostInfo;

#[derive(Debug, Serialize, Deserialize)]
pub struct NN {
    pub layers: Vec<usize>,
    pub weights: Vec<Matrix<f64>>,
    pub biases: Vec<Matrix<f64>>,
    pub apps: Vec<Matrix<f64>>,
}

impl NN {
    pub fn new(layers: &[usize]) -> Self {
        let depth = layers.len();

        let mut weights = Vec::with_capacity(depth - 1);
        let mut biases = Vec::with_capacity(depth - 1);
        let mut apps = Vec::with_capacity(depth);

        for (level, size) in layers.iter().enumerate() {
            if level == 0 {
                apps.push(Matrix::new(1, *size));
            } else {
                weights.push(Matrix::new(apps[level - 1].len_col(), *size));
                biases.push(Matrix::new(1, *size));
                apps.push(Matrix::new(1, *size));
            }
        }

        return NN {
            layers: layers.to_vec(),
            weights,
            biases,
            apps,
        };
    }

    pub fn len(&self) -> usize {
        self.weights.len()
    }

    pub fn process(&mut self) {
        for idx in 0..self.len() {
            let apps = self.apps[idx].clone();
            self.apps[idx + 1].dot(&apps, &self.weights[idx]);
            self.apps[idx + 1].sum(&self.biases[idx]);
            self.apps[idx + 1].sigmoid()
        }
    }

    pub fn set(&mut self, input: &[f64]) {
        assert!(input.len() == self.apps[0].len_col());
        for (idx, r) in self.apps[0].row_mut(0).iter_mut().enumerate() {
            *r = input[idx];
        }
    }

    pub fn output(&self) -> &[f64] {
        self.apps[self.len()].row(0)
    }

    pub fn output_mut(&mut self) -> &mut [f64] {
        let len = self.len();
        self.apps[len].row_mut(0)
    }

    fn __cost(&self, expect: &[f64]) -> f64 {
        assert!(self.output().len() == expect.len());
        let mut diff = 0.0;
        for (idx, output) in self.output().iter().enumerate() {
            diff += (output - expect[idx]).powi(2);
        }
        diff
    }

    pub fn cost(&mut self, inputs: &Vec<Vec<f64>>, expects: &Vec<Vec<f64>>) -> f64 {
        assert!(inputs.len() == expects.len());
        let n = inputs.len() as f64;
        let mut diff = 0.0;
        for round in 0..inputs.len() {
            self.set(inputs[round].as_slice());
            self.process();
            diff += self.__cost(expects[round].as_slice())
        }
        diff / n
    }

    pub fn cost_info(&mut self, inputs: &Vec<Vec<f64>>, expects: &Vec<Vec<f64>>) -> CostInfo {
        assert!(inputs.len() == expects.len());
        let mut cost_info = CostInfo::new();
        for round in 0..inputs.len() {
            self.set(inputs[round].as_slice());
            self.process();
            cost_info.push(self.__cost(expects[round].as_slice()))
        }
        cost_info
    }

    pub fn backprop(&mut self, inputs: &Vec<Vec<f64>>, expects: &Vec<Vec<f64>>) -> Self {
        let n = inputs.len();
        let mut delta = Self::new(
            self.apps
                .iter()
                .fold(Vec::new(), |mut layers, apps| {
                    layers.push(apps.len_col());
                    layers
                })
                .as_slice(),
        );

        for (round, input) in inputs.iter().enumerate() {
            self.set(input.as_slice());
            self.process();

            for level in 0..delta.len() {
                delta.apps[level].fill(0.0);
            }

            for oidx in 0..self.output().len() {
                delta.output_mut()[oidx] = self.output()[oidx] - expects[round][oidx];
            }

            for level in (1..=self.len()).rev() {
                for aidx in 0..self.apps[level].len_col() {
                    let a = self.apps[level].at(0, aidx);
                    let da = delta.apps[level].at(0, aidx);
                    *delta.biases[level - 1].at_mut(0, aidx) += 2.0 * da * a * (1.0 - a);

                    for paidx in 0..self.apps[level - 1].len_col() {
                        let pa = self.apps[level - 1].at(0, paidx);
                        let w = self.weights[level - 1].at(paidx, aidx);
                        *delta.weights[level - 1].at_mut(paidx, aidx) +=
                            2.0 * da * a * (1.0 - a) * pa;
                        *delta.apps[level - 1].at_mut(0, paidx) += 2.0 * da * a * (1.0 - a) * w;
                    }
                }
            }
        }

        for level in 0..delta.len() {
            let n = n as f64;
            delta.weights[level].div(&n);
            delta.biases[level].div(&n);
        }

        delta
    }

    pub fn finite_diff(
        &mut self,
        inputs: &Vec<Vec<f64>>,
        expects: &Vec<Vec<f64>>,
        epsilon: &f64,
    ) -> Self {
        let cost_original = self.cost(inputs, expects);

        let mut delta = Self::new(
            self.apps
                .iter()
                .fold(Vec::new(), |mut layers, apps| {
                    layers.push(apps.len_col());
                    layers
                })
                .as_slice(),
        );

        for level in 0..self.len() {
            for row in 0..self.weights[level].len_row() {
                for col in 0..self.weights[level].len_col() {
                    let saved = self.weights[level].at(row, col);
                    *self.weights[level].at_mut(row, col) += *epsilon;
                    let cost_renewed = self.cost(inputs, expects);
                    *delta.weights[level].at_mut(row, col) =
                        (cost_renewed - cost_original) / epsilon;
                    *self.weights[level].at_mut(row, col) = saved;
                }
            }

            for col in 0..self.biases[level].len_col() {
                let saved = self.biases[level].at(0, col);
                *self.biases[level].at_mut(0, col) += *epsilon;
                let cost_renewed = self.cost(inputs, expects);
                *delta.biases[level].at_mut(0, col) = (cost_renewed - cost_original) / epsilon;
                *self.biases[level].at_mut(0, col) = saved;
            }
        }

        delta
    }

    pub fn mul(&mut self, rate: &f64) {
        for level in 0..self.len() {
            self.weights[level].mul(rate);
            self.biases[level].mul(rate);
        }
    }

    pub fn rand(&mut self) {
        for level in 0..self.len() {
            self.weights[level].rand();
            self.biases[level].rand();
        }
    }

    pub fn learn(&mut self, delta: &Self) {
        for level in 0..self.len() {
            self.weights[level].sub(&delta.weights[level]);
            self.biases[level].sub(&delta.biases[level]);
        }
    }
}

impl Clone for NN {
    fn clone(&self) -> Self {
        Self {
            layers: self.layers.clone(),
            weights: self.weights.clone(),
            biases: self.biases.clone(),
            apps: self.apps.clone(),
        }
    }
}
