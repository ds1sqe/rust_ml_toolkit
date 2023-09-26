use crate::matrix::matrix::Matrix;
use crate::matrix::matrix::__Matrix;

struct NN {
    weights: Vec<Matrix<f64>>,
    biases: Vec<Matrix<f64>>,
    apps: Vec<Matrix<f64>>,
}

// trait __NN {}

impl NN {
    fn new(layers: &[usize]) -> Self {
        let depth = layers.len();

        let mut weights = Vec::with_capacity(depth - 1);
        let mut biases = Vec::with_capacity(depth - 1);
        let mut apps = Vec::with_capacity(depth);

        for (level, size) in layers.iter().enumerate() {
            if level == 0 {
                apps[0] = Matrix::new(1, *size);
            } else {
                weights[level] = Matrix::new(apps[level - 1].len_col(), *size);
                biases[level - 1] = Matrix::new(1, *size);
                apps[level - 1] = Matrix::new(1, *size);
            }
        }

        return NN {
            weights,
            biases,
            apps,
        };
    }

    fn len(&self) -> usize {
        self.weights.len()
    }

    fn process(&mut self) {
        for idx in 0..self.len() {
            let apps = self.apps[idx].clone();
            self.apps[idx+1].dot(&apps,&self.weights[idx]);
            self.apps[idx+1].sum(&self.biases[idx]);
            self.apps[idx+1].sigmoid()
        }
    }

    fn set(&mut self,input:&[f64]) {
        assert!(input.len()==self.apps[0].len_col());
        for (idx, r) in self.apps[0].row_mut(0).iter_mut().enumerate() {
            *r = input[idx];
        }
    }

    fn output(&self) -> &[f64] {
        self.apps[self.len()].row(0)
    }

    fn __diff(&self, expect: &[f64]) {
        assert!(self.output().len()==expect.len());
        let mut diff = 0.0;
        for (idx,output) in self.output().iter().enumerate() {
            diff += (output-expect[idx]).powi(2);
        }
    }

}
