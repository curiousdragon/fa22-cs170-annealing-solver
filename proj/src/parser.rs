use std::{collections::HashMap, fs};

#[derive(Debug)]
pub struct Graph {
    // u : [(v, w)]
    n: i32,
    e: i32,
    edges: HashMap<i32, Vec<(i32, i32)>>,
}

pub fn run(filename: &str) -> Graph {
    let f = fs::read_to_string(filename).expect("Could not read file");
    let mut it = f.lines();
    let n = it.next().expect("Out of lines").parse().expect("Not int");
    let e = it.next().expect("Out of lines").parse().expect("Not int");
    let mut edges = HashMap::<i32, Vec<(i32, i32)>>::new();
    for line in it {
        let mut split = line.split_whitespace().map(|x| x.parse().expect("Not int"));
        let i = split.next().expect("Not enough elements");
        let j = split.next().expect("Not enough elements");
        let w = split.next().expect("Not enough elements");
        edges.entry(i).or_default().push((j, w));
    }
    Graph { n, e, edges }
}
