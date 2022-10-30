mod generator;
mod parser;
use std::io;

fn main() {
    println!("Choose operating mode:\n[G] Generate, [S] Solver, [D] Change default solver");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap_or_default();
    match input.trim() {
        "G" => generator::run(),
        x => println!("{:?}", parser::run(x)),
    }
}
