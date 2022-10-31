use itertools::Itertools;
use rand::distributions::{Distribution, Uniform};
use std::fs;

const MAXIMUM_WEIGHT: i32 = 1000;
// const EDGE_COUNT: usize = 5000;
const N_SMALL: usize = 50;
const N_MEDIUM: usize = 250;
const N_LARGE: usize = 1000;
// const K_EXP: usize = 1;
// const V_EXP: usize = 5;
// const K_COEFFICIENT: usize = 15;

const NUM_SAMPLES: usize = 10;

pub fn run() {
    // s; n=100, 15<=E<=25
    // m; n=250, 45<=E<=55
    // l; n=1000, 195<=E<=5000

    generate_and_write(N_SMALL, 15, 25, 0);
    generate_and_write(N_MEDIUM, 45, 55, NUM_SAMPLES);
    generate_and_write(N_LARGE, 195, 5000, NUM_SAMPLES * 2);
}

fn generate_and_write(n: usize, uniform_low: usize, uniform_high: usize, start_index: usize) {
    // |V|
    // |E|
    // i j w
    let edge_size = Uniform::from(uniform_low..uniform_high);
    let mut rng = rand::thread_rng();
    for i in start_index..start_index + NUM_SAMPLES {
        let e = edge_size.sample(&mut rng);
        let edges = generate_edges(n.try_into().unwrap(), e);
        let out = [n, e]
            .map(|x| format!("{x}"))
            .into_iter()
            .chain(edges.iter().map(|[i, j, w]| format!("{i} {j} {w}")))
            .join("\n")
            + "\n";
        fs::write(format!("inputs/input{i}"), out).expect("Could not write to file");
    }
}

fn generate_edges(n: i32, e: usize) -> Vec<[i32; 3]> {
    let nodes = Uniform::from(0..n);
    let weights = Uniform::from(0..MAXIMUM_WEIGHT);
    let mut rng = rand::thread_rng();
    (0..e)
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
        .collect()
}
