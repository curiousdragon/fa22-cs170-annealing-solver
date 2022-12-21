use itertools::Itertools;
use rand::distributions::{Distribution, Uniform};
use std::fs;

const MAX_WEIGHT: i32 = 1000;
const MAX_EDGES: i32 = 10000;
const MIN_TOTAL_WEIGHT: i32 = 500000;
const MIN_EDGES: i32 = MIN_TOTAL_WEIGHT / MAX_WEIGHT;

const N_SMALL: usize = 100;
const N_MEDIUM: usize = 300;
const N_LARGE: usize = 1000;
const N_SAMPLES: usize = 10;

pub fn run() {
    generate_and_write(N_SMALL, 0);
    generate_and_write(N_MEDIUM, N_SAMPLES);
    generate_and_write(N_LARGE, N_SAMPLES * 2);
}

fn generate_and_write(n: usize, start_index: usize) {
    let mut rng = rand::thread_rng();

    let edge_size = Uniform::from(MIN_EDGES..MAX_EDGES);
    for i in start_index..start_index + N_SAMPLES {
        let e = edge_size.sample(&mut rng);
        let edges = generate_edges(n.try_into().unwrap(), e);
        let out = [n, e as usize]
            .map(|x| format!("{x}"))
            .into_iter()
            .chain(edges.iter().map(|[i, j, w]| format!("{i} {j} {w}")))
            .join("\n")
            + "\n";
        fs::write(format!("inputs/input{i}"), out).expect("Could not write to file");
    }
}

fn generate_edges(n: i32, e: i32) -> Vec<[i32; 3]> {
    let mut rng = rand::thread_rng();

    let nodes = Uniform::from(0..n);
    let weights = Uniform::from(0..MAX_WEIGHT);
    let edges: Vec<[i32; 3]> = (0..e)
        .map(|_| {
            let src = nodes.sample(&mut rng);
            let dest = loop {
                let v = nodes.sample(&mut rng);
                if v != src {
                    break v;
                }
            };
            let weight = weights.sample(&mut rng);
            [src, dest, weight]
        })
        .collect();
    let total_weight: i32 = edges.iter().map(|[_, _, weight]| weight).sum();
    let difference = MIN_TOTAL_WEIGHT - total_weight;
    if difference > 0 {
        let weight_per_edge = difference / e + 1;
        edges
            .iter()
            .map(|[src, dest, weight]| [*src, *dest, *weight + weight_per_edge])
            .collect()
    } else {
        edges
    }
}
