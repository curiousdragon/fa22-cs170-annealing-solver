use std::collections::HashMap;

use crate::datatypes::Graph;
use crate::datatypes::Loss;
use crate::datatypes::Partition;

pub fn loss(g: &Graph, parts: &Partition) -> Loss {
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

    let k = (*partitions.iter().max().unwrap() + 1) as f64;
    let num_partition_loss = 100_f64 * f64::exp(0.5_f64 * k);

    let mut partition_sizes = HashMap::<i32, i32>::new();
    for pi in partitions {
        *partition_sizes.entry(*pi).or_default() += 1;
    }

    let partition_size_loss = f64::exp(
        70_f64
            * partition_sizes
                .values()
                .map(|size| ((*size as f64) / (g.n as f64) - 1_f64 / k).powi(2))
                .sum::<f64>()
                .sqrt(),
    );

    Loss {
        weight_loss,
        num_partition_loss,
        partition_size_loss,
        loss: weight_loss + num_partition_loss + partition_size_loss,
    }
}
