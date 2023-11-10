use std::{fs::File, path::Path};

use crate::core::{matrix::matrix::Matrix, nn::nn::NN};

pub trait Stringfiable {
    type Struct;
    fn stringfy(src: &Self::Struct) -> Option<String>;
}

pub trait Buildable {
    type Struct;
    fn build(str: String) -> Self::Struct;
}

pub trait Readable {
    type Struct;
    fn read(path: &Path) -> Option<Self::Struct>;
}
pub trait Savable {
    type Struct;
    fn save(data: Self::Struct, path: &Path) -> bool;
}

impl Stringfiable for Matrix<f64> {
    type Struct = Matrix<f64>;

    fn stringfy(src: &Self::Struct) -> Option<String> {
        todo!()
    }
}

impl Stringfiable for NN {
    type Struct = NN;
    fn stringfy(src: &Self::Struct) -> Option<String> {
        let mut result = String::new();
        let cloned = src.clone();

        result.push_str("[Layers]\n");
        for (level, layer_info) in cloned.layers.iter().enumerate() {
            result.push_str(&layer_info.to_string());
            if level < cloned.layers.len() - 1 {
                result.push(',')
            }
        }
        result.push_str("\n[Weights]\n");
        for (level, weight_info) in cloned.weights.iter().enumerate() {
            result.push_str(&Matrix::<f64>::stringfy(&weight_info).unwrap());
            if level < cloned.weights.len() - 1 {
                result.push(',')
            }
        }
        result.push_str("\n[Biases]\n");
        for (level, biase_info) in cloned.biases.iter().enumerate() {
            result.push_str(&Matrix::<f64>::stringfy(&biase_info).unwrap());
            if level < cloned.biases.len() - 1 {
                result.push(',')
            }
        }
        result.push_str("\n[Apps]\n");
        for (level, app_info) in cloned.apps.iter().enumerate() {
            result.push_str(&Matrix::<f64>::stringfy(&app_info).unwrap());
            if level < cloned.apps.len() - 1 {
                result.push(',')
            }
        }

        Some(result)
    }
}

impl Buildable for NN {
    type Struct = NN;
    fn build(str: String) -> Self::Struct {
        let cloned = str.clone();
        let idx_layer_start = cloned.find("[Layers]");
        let idx_weights_start = cloned.find("[Weights]");

        let idx_biases_start = cloned.find("[Biases]");
        let idx_apps_start = cloned.find("[Apps]");

        todo!()
    }
}

impl Savable for NN {
    type Struct = NN;
    fn save(data: Self::Struct, path: &Path) -> bool {
        todo!()
    }
}

impl Readable for NN {
    type Struct = NN;

    fn read(path: &Path) -> Option<Self::Struct> {
        let mut data = match File::open(path) {
            Err(e) => panic!("could not open {}: {}", path.display(), e),
            Ok(file) => file,
        };
        todo!()
    }
}
