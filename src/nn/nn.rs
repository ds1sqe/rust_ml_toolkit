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
}
