use std::collections::HashMap;

#[derive(Debug)]
pub struct Graph {
    // |V|
    pub n: i32,
    // |E|
    pub e: i32,
    // i : [(j, w), ...]
    pub edges: HashMap<i32, Vec<(i32, i32)>>,
}

#[derive(Clone, Debug)]
pub struct Partition {
    // max pi(v)
    pub k: i32,
    // v : pi(v)
    pub partitions: Vec<i32>,
}
