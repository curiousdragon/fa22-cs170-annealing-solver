use std::vec;

use crate::datatypes::Graph;
use crate::datatypes::Partition;
use crate::scorer::loss;
use rand::distributions::{Distribution, Uniform};
use rand::Rng;

pub fn init(g: &Graph, num_partitions: i32) -> Partition {
    let mut partitions = vec![0; g.n as usize];
    for i in 0..(g.n as usize) {
        partitions[i] = (i as i32) % num_partitions;
    }
    Partition {
        k: num_partitions,
        partitions,
    }
}

fn propose(g: &Graph, prev_p: &Partition, rng: &mut impl Rng) -> Partition {
    let nodes = Uniform::from(0..(g.n as usize));
    let mut proposed_p = prev_p.clone();

    let swap = rng.gen_bool(0.75);
    if swap {
        let i = nodes.sample(rng);
        let j = nodes.sample(rng);
        proposed_p.partitions[i] = prev_p.partitions[j];
        proposed_p.partitions[j] = prev_p.partitions[i];
    } else {
        let i = nodes.sample(rng);
        let parts = Uniform::from(0..prev_p.k);
        proposed_p.partitions[i] = parts.sample(rng);
    }

    proposed_p
}

fn temperature(r: &f64) -> f64 {
    // let lambda = 0.5_f64;
    // f64::exp(-r / lambda)

    let beta = 0.5;
    r / (1_f64 + beta * r)

    // *r
}

fn acceptance(g: &Graph, prev_p: &Partition, proposed_p: &Partition, temp: &f64) -> f64 {
    let prev_loss = loss(g, prev_p);
    let proposed_loss = loss(g, proposed_p);
    if proposed_loss < prev_loss {
        1_f64
    } else {
        f64::exp(-(proposed_loss - prev_loss) / temp)
    }
}

pub fn simulated_annealing(
    g: &Graph,
    iterations: usize,
    rng: &mut impl Rng,
    num_partitions: i32,
) -> Partition {
    let mut prev_p = init(g, num_partitions);
    for i in 0..iterations {
        let temp = temperature(&(1_f64 - ((i as f64) / (iterations as f64))));
        let proposed_p = propose(g, &prev_p, rng);
        let accept_prob = acceptance(g, &prev_p, &proposed_p, &temp);
        let prob = rng.gen_range(0_f64..1_f64);
        if prob <= accept_prob {
            prev_p = proposed_p;
        }
        // if i % 1000 == 0 {
        //     println!("{}", loss(g, &prev_p));
        // }
    }
    // println!("{}", loss(g, &prev_p));
    prev_p
}

pub fn run(g: &Graph, iterations: usize, rng: &mut impl Rng) -> Partition {
    // // let mut left = 0;
    // let mut left = 2;
    // // let mut right = g.n;
    // let mut right = 10;
    // while (right - left).abs() > 2 {
    //     let left_third = left + (right - left) / 3;
    //     let right_third = right - (right - left) / 3;

    //     let left_partition = simulated_annealing(g, iterations, rng, left_third);
    //     let right_partition = simulated_annealing(g, iterations, rng, right_third);
    //     if loss(g, &left_partition) < loss(g, &right_partition) {
    //         left = left_third;
    //     } else {
    //         right = right_third;
    //     }
    // }
    // simulated_annealing(g, iterations, rng, left)
    let low = 1;
    let high = 10;
    let mut best_partition = init(g, 1);
    let mut best_cost = f64::MAX;
    for k in low..high {
        let p = simulated_annealing(g, iterations, rng, k);
        let cost = loss(g, &p);
        if cost < best_cost {
            best_cost = cost;
            best_partition = p;
        }
    }
    best_partition
}
