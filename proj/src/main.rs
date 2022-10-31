mod datatypes;
mod generator;
mod parser;
mod scorer;
mod solver;
use itertools::Itertools;
use rayon::prelude::*;
use std::{fs, io};

use rand::thread_rng;

fn run_solver_on_input(iterations: usize, input_name: &str) {
    println!("Running simulated annealing solver...\n");
    let mut rng = thread_rng();
    let filepath = format!("inputs/{input_name}");
    let g = parser::run(&filepath);
    let p = solver::run(&g, iterations, &mut rng);
    parser::write_output(&format!("outputs/{input_name}-output"), &g, &p);
    let cost = scorer::loss(&g, &p);
    println!("{filepath}: {cost}");
    println!();
    println!("Running simulated annealing solver complete!");
}

fn run_solver_on_all_inputs(iterations: usize) {
    println!("Running simulated annealing solver...\n");
    let paths = fs::read_dir("inputs").expect("No inputs folder");
    let filepaths = paths.collect_vec();
    filepaths.into_par_iter().for_each(|path| {
        let mut rng = thread_rng();
        let expected_file = path.expect("No input file");
        let os_filename = expected_file.file_name();
        let os_path = expected_file.path();
        let filename = os_filename.to_str().expect("Invalid input filename");
        let filepath = os_path.to_str().expect("Invalid input filepath");
        let g = parser::run(filepath);
        let p = solver::run(&g, iterations, &mut rng);
        parser::write_output(&format!("outputs/{filename}-output"), &g, &p);
        let cost = scorer::loss(&g, &p);
        println!("{filepath}: {cost}");
    });
    println!("Running simulated annealing solver complete!");
}

fn main() {
    let iterations = 200000; // 10000;

    println!("Choose operating mode:");
    println!("[G] Generate, [D] Default solve (10000), [#] Solve with # iterations");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap_or_default();
    match input.trim() {
        "G" => generator::run(),
        "D" => run_solver_on_all_inputs(iterations),
        // x => run_solver_on_inputs(x.parse().expect("Not an int")),
        x => run_solver_on_input(500000, x),
    }
}
