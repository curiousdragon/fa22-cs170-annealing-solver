use std::collections::HashMap;

use crate::datatypes::Graph;
use crate::datatypes::Partition;

pub fn loss(g: &Graph, parts: &Partition) -> f64 {
    let partitions = &parts.partitions;

    let weight_loss = (0..partitions.len())
        .map(|i| match g.edges.get(&(i as i32)) {
            Some(adj) => adj
                .iter()
                .map(|(j, w)| {
                    if partitions[i] == partitions[(*j as usize)] {
                        *w as f64
                    } else {
                        0_f64
                    }
                })
                .sum::<f64>(),
            None => 0_f64,
        })
        .sum::<f64>()
        / 2_f64;

    // let k = partition_sizes.len() as f64;
    let k = parts.k as f64;
    let num_partition_loss = 15_f64 * f64::exp(k);

    let mut partition_sizes = HashMap::<i32, i32>::new();
    for pi in partitions {
        *partition_sizes.entry(*pi).or_default() += 1;
    }

    let partition_diff_loss = f64::exp(
        5_f64
            * partition_sizes
                .values()
                .map(|size| ((*size as f64) / (g.n as f64) - 1_f64 / k).powi(2))
                .sum::<f64>()
                .sqrt(),
    );

    weight_loss + num_partition_loss + partition_diff_loss
}
