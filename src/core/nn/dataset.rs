use serde::{Deserialize, Serialize};

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

impl<T> Stringfiable for DataSet<T> {
    type Struct = Self;

    fn stringfy(src: &Self::Struct) -> Option<String> {
        todo!()
    }
}

impl<T> Buildable for DataSet<T> {
    type Struct = Self;

    fn build(str: String) -> Option<Self::Struct> {
        todo!()
    }
}

impl<T> Readable for DataSet<T> {
    type Struct = Self;

    fn read(path: &std::path::Path) -> Option<Self::Struct> {
        todo!()
    }
}

impl<T> Savable for DataSet<T> {
    type Struct = Self;

    fn save(data: &Self::Struct, path: &std::path::Path) -> Option<bool> {
        todo!()
    }
}
