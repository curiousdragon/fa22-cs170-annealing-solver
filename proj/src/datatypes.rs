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
    // v : pi(v)
    pub partitions: Vec<i32>,
    // total number of partitions represented by
    // (max pi(v)) + 1
    // since partitions are zero-indexed
}

#[derive(Clone, Debug)]
pub struct Loss {
    // components of the loss
    pub weight_loss: f64,
    pub num_partition_loss: f64,
    pub partition_size_loss: f64,

    // total loss = weight_loss + num_partition_loss + partition_size_loss
    pub loss: f64,
}
