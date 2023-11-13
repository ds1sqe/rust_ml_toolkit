use std::{
    fs::File,
    io::{Read, Write},
    path::Path,
};

use crate::core::nn::nn::NN;

// TODO: rename this file

pub trait Stringfiable {
    type Struct;
    fn stringfy(src: &Self::Struct) -> Option<String>;
}

pub trait Buildable {
    type Struct;
    fn build(str: String) -> Option<Self::Struct>;
}

pub trait Readable {
    type Struct;
    fn read(path: &Path) -> Option<Self::Struct>;
}
pub trait Savable {
    type Struct;
    fn save(data: &Self::Struct, path: &Path) -> Option<bool>;
}

impl Stringfiable for NN {
    type Struct = NN;
    fn stringfy(src: &Self::Struct) -> Option<String> {
        let output = serde_json::to_string_pretty(&src);
        if output.is_ok() {
            return Some(output.unwrap());
        }
        None
    }
}

impl Buildable for NN {
    type Struct = NN;
    fn build(str: String) -> Option<Self::Struct> {
        let cloned = str.clone();
        let nn = serde_json::from_str(&cloned);
        if nn.is_ok() {
            return nn.unwrap();
        }
        None
    }
}

impl Savable for NN {
    type Struct = NN;
    fn save(data: &Self::Struct, path: &Path) -> Option<bool> {
        let mut file = match File::create(path) {
            Err(e) => panic!("could not create at {}: {}", path.display(), e),
            Ok(file) => file,
        };
        let str = NN::stringfy(data);
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

impl Readable for NN {
    type Struct = NN;

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

        let nn = NN::build(buf.to_string());
        if nn.is_none() {
            return None;
        }
        nn
    }
}

#[test]
fn test_data_save_and_read() {
    let layers = [2, 4, 4, 1];
    let mut orgin = NN::new(&layers);

    const PRSIZE: f64 = 1_000_000_000.0;

    let path = Path::new("nn.json");

    orgin.rand();

    println!("Origianl NN:\n{:?}", orgin);

    NN::save(&orgin, path);

    let saved = NN::read(path).unwrap();

    println!("Saved NN:\n{:?}", saved);

    for (idx, l) in orgin.layers.iter().enumerate() {
        assert!(saved.layers[idx] == *l)
    }
    for (idx, matrix) in orgin.weights.iter().enumerate() {
        for (widx, weight) in matrix.el.iter().enumerate() {
            for (wwidx, val) in weight.iter().enumerate() {
                assert!(
                    (f64::trunc(saved.weights[idx].el[widx][wwidx] * PRSIZE)
                        / PRSIZE)
                        == (f64::trunc(*val * PRSIZE) / PRSIZE)
                )
            }
        }
    }
    for (idx, matrix) in orgin.biases.iter().enumerate() {
        for (bidx, biase) in matrix.el.iter().enumerate() {
            for (bbidx, val) in biase.iter().enumerate() {
                assert!(
                    (f64::trunc(saved.biases[idx].el[bidx][bbidx] * PRSIZE)
                        / PRSIZE)
                        == (f64::trunc(*val * PRSIZE) / PRSIZE)
                )
            }
        }
    }

    std::fs::remove_file(path).unwrap();
}
