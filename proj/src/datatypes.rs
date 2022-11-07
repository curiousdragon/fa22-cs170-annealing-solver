use std::collections::HashMap;

#[derive(Debug)]
pub struct Graph {
    // |V|
    pub n: i32,
    // |E|
    pub e: i32,
    // i : [(j, w), ...]
    pub edges: HashMap<i32, Vec<(i32, i32)>>,
    pub original_file_as_string: String,
}

#[derive(Clone, Debug)]
pub struct Partition {
    // max pi(v)
    // pub k: i32,
    // v : pi(v)
    pub partitions: Vec<i32>,
}

#[derive(Clone, Debug)]
pub struct Loss {
    pub weight_loss: f64,
    pub num_partition_loss: f64,
    pub partition_size_loss: f64,
    pub loss: f64,
}
