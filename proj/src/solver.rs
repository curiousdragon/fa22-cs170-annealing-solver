use crate::datatypes::Graph;
use crate::datatypes::Partition;
use crate::scorer::loss;
use rand::distributions::{Distribution, Uniform};
use rand::Rng;

pub fn init(g: &Graph) -> Partition {
    // let mut rng = thread_rng();
    // let between = Uniform::from(0..g.n);
    // let partitions: HashMap<i32, i32> = (0..g.n).map(|i| (i, between.sample(&mut rng))).collect();
    // let k = partitions.values().iter().max();
    Partition {
        k: 1,
        partitions: vec![0; g.n as usize],
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
        let parts = Uniform::from(0..=prev_p.k);
        let i_partition = parts.sample(rng);
        proposed_p.partitions.insert(i, i_partition);
        if i_partition == prev_p.k {
            proposed_p.k += 1;
        }
    }

    proposed_p
}

fn temperature(r: &f64) -> f64 {
    // let lambda = 0.5_f64;
    // f64::exp(-r / lambda)

    // let beta = 0.5;
    // r / (1_f64 + beta * r)
    *r
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

pub fn simulated_annealing(g: &Graph, iterations: usize, rng: &mut impl Rng) -> Partition {
    let mut prev_p = init(g);
    // let mut temp = 1_f64;
    for i in 0..iterations {
        let temp = temperature(&(1_f64 - ((i as f64) / (iterations as f64))));
        let proposed_p = propose(g, &prev_p, rng);
        let accept_prob = acceptance(g, &prev_p, &proposed_p, &temp);
        let prob = rng.gen_range(0_f64..1_f64);
        if prob <= accept_prob {
            prev_p = proposed_p;
        }
        // temp = temp - temperature(&temp);
        if i % 1000 == 0 {
            println!("{} {}", loss(g, &prev_p), accept_prob);
        }
    }
    prev_p
}
