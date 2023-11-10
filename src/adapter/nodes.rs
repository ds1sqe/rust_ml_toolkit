use crate::core::{matrix::matrix::__Matrix, nn::nn::NN};

#[derive(Debug)]
pub enum Nodetype {
    Input,
    Middle,
    Output,
}

#[derive(Debug)]
pub struct Nodes {
    pub layers: Vec<usize>,
    pub nodes: Vec<Vec<Node>>,
    pub connections: Vec<Vec<Vec<Connection>>>,
}

#[derive(Debug)]
pub struct Node {
    pub nodetype: Nodetype,
    pub level: usize,
    pub bias: f64,
    pub value: f64,
}
#[derive(Debug)]
pub struct Connection {
    pub dst: usize,
    pub dst_level: usize,
    pub src: usize,
    pub src_level: usize,
    pub weight: f64,
}

impl Nodes {
    pub fn from(src: &NN) -> Nodes {
        let layers = src.layers.clone();
        let mut nodes = Vec::new();
        let mut connections = Vec::new();
        for (level, layer) in layers.iter().enumerate() {
            if level == 0 {
                let mut input_nodes = Vec::new();
                for col in 0..*layer {
                    input_nodes.push(Node {
                        nodetype: Nodetype::Input,
                        level,
                        bias: 0.0,
                        value: src.apps[0].at(0, col),
                    })
                }
                nodes.push(input_nodes);
            } else {
                let mut mid_nodes = Vec::new();
                let mut mid_connections = Vec::new();

                for node_idx in 0..*layer {
                    let mut node_connections = Vec::new();
                    mid_nodes.push({
                        Node {
                            nodetype: if level < layers.len() - 1 {
                                Nodetype::Middle
                            } else {
                                Nodetype::Output
                            },
                            level,
                            bias: src.biases[level - 1].at(0, node_idx),
                            value: src.apps[level].at(0, node_idx),
                        }
                    });

                    for (src_idx, w) in src.weights[level - 1].col(node_idx).iter().enumerate() {
                        node_connections.push(Connection {
                            dst: node_idx,
                            dst_level: level,
                            src: src_idx,
                            src_level: level - 1,
                            weight: **w,
                        })
                    }
                    mid_connections.push(node_connections);
                }
                nodes.push(mid_nodes);
                connections.push(mid_connections);
            }
        }
        Nodes {
            layers,
            nodes,
            connections,
        }
    }
}

#[test]
fn test_nodes_from() {
    let layers = [2, 4, 4, 1];
    let mut orgin = NN::new(&layers);

    orgin.rand();

    println!("{:#?}", orgin);

    let nodes = Nodes::from(&orgin);

    println!("{:#?}", nodes);
}
