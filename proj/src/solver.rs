use crate::datatypes::Graph;
use crate::datatypes::Loss;
use crate::datatypes::Partition;
use crate::scorer::loss;
use itertools::Itertools;
use rand::distributions::{Distribution, Uniform};
use rand::Rng;
use tqdm::tqdm;

pub fn init(g: &Graph, num_partitions: i32) -> Partition {
    Partition {
        partitions: (0..g.n).map(|i| i % num_partitions).collect_vec(),
    }
}

fn propose(g: &Graph, prev_p: &Partition, rng: &mut impl Rng) -> Partition {
    let nodes = Uniform::from(0..(g.n as usize));
    let mut proposed_p = prev_p.clone();
    let prev_part = &prev_p.partitions;

    let swap = rng.gen_bool(0.75);
    if swap {
        let i = nodes.sample(rng);
        let j = nodes.sample(rng);
        proposed_p.partitions[i] = prev_part[j];
        proposed_p.partitions[j] = prev_part[i];
    } else {
        let i = nodes.sample(rng);
        let max_part = *prev_part.iter().max().unwrap() + 1;
        let parts = Uniform::from(0..max_part);
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
    if proposed_loss.loss < prev_loss.loss {
        1_f64
    } else {
        f64::exp(-(proposed_loss.loss - prev_loss.loss) / temp)
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
    }
    prev_p
}

pub fn run(g: &Graph, iterations: usize, rng: &mut impl Rng) -> (Partition, Loss) {
    // initialize with all nodes in the same partition
    let mut best_partition = init(g, 1);
    let mut best_cost = loss(g, &best_partition);

    // iteration through other possible numbers of partitions
    let low = 2;
    let high = 20;
    for k in tqdm(low..high) {
        let p = simulated_annealing(g, iterations, rng, k);
        let cost = loss(g, &p);
        if cost.loss < best_cost.loss {
            best_cost = cost;
            best_partition = p;
        }
    }
    (best_partition, best_cost)
}
