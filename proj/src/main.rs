mod datatypes;
mod generator;
mod parser;
mod scorer;
mod solver;
use itertools::Itertools;
use rand::thread_rng;
use rayon::prelude::*;
use std::{fs, io};

fn run_solver_on_input(iterations: usize, filename: &str) -> String {
    let mut rng = thread_rng();
    let filepath = format!("inputs/{filename}");
    let g = parser::run(&filepath);
    let (p, cost) = solver::run(&g, iterations, &mut rng);
    parser::write_output(&format!("outputs/{filename}-output"), &g, &p);
    let score_info = format!(
        "{filepath}: [{4}] {0} = {1} + {2} + {3}\n",
        cost.loss,
        cost.weight_loss,
        cost.num_partition_loss,
        cost.partition_size_loss,
        datatypes::uniq_partitions(&p)
    );
    fs::write(&format!("scores/{filename}-score"), &score_info)
        .expect(&format!("Could not write to file: scores/{filename}-score"));
    score_info
}

fn run_solver_on_given_input(iterations: usize, filename: &str) {
    eprintln!("Running simulated annealing solver...");
    let score_info = run_solver_on_input(iterations, filename);
    eprintln!("\n{}", score_info);
    eprintln!("Running simulated annealing solver complete!");
}

fn run_solver_on_all_inputs(iterations: usize) {
    eprintln!("Running simulated annealing solver...");
    let paths = fs::read_dir("inputs").expect("No inputs folder");
    let filepaths = paths.collect_vec();
    eprintln!("{}", "\n".to_string().repeat(filepaths.len() - 1));
    filepaths.into_par_iter().for_each(|path| {
        let os_filename = path.expect("No input file").file_name();
        let filename = os_filename.to_str().expect("Invalid file name");
        run_solver_on_input(iterations, filename);
    });
    eprintln!();
    let paths = fs::read_dir("scores").expect("No scores folder");
    for path in paths {
        let score_info =
            fs::read_to_string(path.expect("No score file").path()).unwrap_or_default();
        eprintln!("{}", score_info.trim());
    }
    eprintln!("\nRunning simulated annealing solver complete!");
}

fn main() {
    let mut iterations = 100000; // 500000; // 200000; // 10000;

    loop {
        eprintln!("\nChoose operating mode:");
        eprintln!(
            concat!(
                "[G] Generate\n",
                "[A] Solve all ({iterations}it)\n",
                "[I] Solve a specific input ({iterations}it)\n",
                "[N] Set number of iterations"
            ),
            iterations = iterations
        );

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap_or_default();
        match input.trim() {
            "G" => generator::run(),
            "A" => run_solver_on_all_inputs(iterations),
            "I" => {
                eprintln!("[filename] Solve inputs/filename ({iterations}it)");
                let mut filename = String::new();
                io::stdin().read_line(&mut filename).unwrap_or_default();
                run_solver_on_given_input(iterations, &filename.trim());
            }
            "N" => {
                eprintln!("[n] Change the number of iterations to n");
                let mut n_input = String::new();
                io::stdin().read_line(&mut n_input).unwrap_or_default();
                iterations = n_input.trim().parse().expect("Input n is not a usize");
                continue;
            }
            _ => break,
        }
        break;
    }
}
