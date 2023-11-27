use std::{
    fs::File,
    io::{Read, Write},
    path::Path,
};

use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::adapter::data::{Buildable, Readable, Savable, Stringfiable};

/// training dataset
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataSet<T> {
    /// input data on nn
    pub inputs: Vec<Vec<T>>,
    /// expected outputs
    pub outputs: Vec<Vec<T>>,
}

impl<T> DataSet<T> {
    pub fn new(inputs: Vec<Vec<T>>, outputs: Vec<Vec<T>>) -> Self {
        debug_assert!(inputs.len() == outputs.len());
        Self { inputs, outputs }
    }
}

impl<T: Serialize> Stringfiable for DataSet<T> {
    type Struct = DataSet<T>;
    fn stringfy(src: &Self::Struct) -> Option<String> {
        let output = serde_json::to_string_pretty(&src);
        if output.is_ok() {
            return Some(output.unwrap());
        }
        None
    }
}

impl<T: DeserializeOwned> Buildable for DataSet<T> {
    type Struct = DataSet<T>;
    fn build(str: String) -> Option<Self::Struct> {
        let cloned = str.clone();
        let ss = serde_json::from_str(&cloned);
        if ss.is_ok() {
            return ss.unwrap();
        }
        None
    }
}

impl<T: Serialize> Savable for DataSet<T> {
    type Struct = DataSet<T>;
    fn save(data: &Self::Struct, path: &Path) -> Option<bool> {
        let mut file = match File::create(path) {
            Err(e) => panic!("could not create at {}: {}", path.display(), e),
            Ok(file) => file,
        };
        let str = DataSet::stringfy(data);
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

impl<T: DeserializeOwned> Readable for DataSet<T> {
    type Struct = DataSet<T>;

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

        let ss = DataSet::build(buf.to_string());
        if ss.is_none() {
            return None;
        }
        ss
    }
}
