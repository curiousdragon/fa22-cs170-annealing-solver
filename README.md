# Fall 2022 CS 170 Simulated Annealing Solver

A staff solver for testing the fall 2022 CS 170 project pre-release.

## Rust Installation

See the [Rust instructions](https://www.rust-lang.org/tools/install)
for installing Rust.

As per the instructions, for macOS:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Usage

In the `proj` folder, run
```bash
cargo run --release
```

The program will prompt you with:
```bash
Choose operating mode:
[G] Generate
[A] Solve all (100000it)
[I] Solve a specific input (100000it)
[N] Set number of iterations
```

Type `A` to solve all inputs in the `inputs/` folder.

Type `I` to solve a specific input in the `inputs/` folder, e.g.
```bash
[filename] Solve inputs/filename (100000it)
```
Input `good_small.in` to run the solver on `inputs/good_small.in`.

Type `N` to change the number of iterations the solver is run for.
```bash
[n] Change the number of iterations to n
```
and then input the number of iterations desired, e.g. `5000`.
Your current output should then look like the following:
```bash
Choose operating mode:
[G] Generate
[A] Solve all (100000it)
[I] Solve a specific input (100000it)
[N] Set number of iterations
N
[n] Change the number of iterations to n
5000

Choose operating mode:
[G] Generate
[A] Solve all (5000it)
[I] Solve a specific input (5000it)
[N] Set number of iterations
```
upon which you can select which option you'd like to go with as before.

If there are no options you would like to select, use `Ctrl-C` to exit.
