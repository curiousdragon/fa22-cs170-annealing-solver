mod datatypes;
mod generator;
mod parser;
mod scorer;
mod solver;
use std::{fs, io};

use rand::thread_rng;

fn run_solver_on_input(iterations: usize, input_name: &str) {
    println!("Running simulated annealing solver...\n");
    let mut rng = thread_rng();
    let filepath = format!("inputs/{input_name}");
    let g = parser::run(&filepath);
    let init_p = solver::init(&g);
    let init_cost = scorer::loss(&g, &init_p);
    let p = solver::simulated_annealing(&g, iterations, &mut rng);
    let cost = scorer::loss(&g, &p);
    println!("{filepath}: {init_cost} -> {cost}");
    println!();
    println!("Running simulated annealing solver complete!");
}

fn run_solver_on_all_inputs(iterations: usize) {
    println!("Running simulated annealing solver...\n");
    let mut rng = thread_rng();
    let paths = fs::read_dir("inputs").expect("No inputs folder");
    for path in paths {
        let os_path = path.expect("No input file").path();
        let filepath = os_path.to_str().expect("Invalid input filename");
        let g = parser::run(filepath);
        let init_p = solver::init(&g);
        let init_cost = scorer::loss(&g, &init_p);
        let p = solver::simulated_annealing(&g, iterations, &mut rng);
        let cost = scorer::loss(&g, &p);
        println!("{filepath}: {init_cost} -> {cost}");
    }
    println!("Running simulated annealing solver complete!");
}

fn main() {
    println!("Choose operating mode:");
    println!("[G] Generate, [D] Default solve (10000), [#] Solve with # iterations");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap_or_default();
    match input.trim() {
        "G" => generator::run(),
        "D" => run_solver_on_all_inputs(10000),
        // x => run_solver_on_inputs(x.parse().expect("Not an int")),
        x => run_solver_on_input(500000, x),
    }
}
