mod card;
mod cli;
mod play;
mod player;
mod solver;
mod tasks;

use crate::cli::{parse, RunOutput};
use crate::solver::solve;
use std::io;
use std::time::Instant;

fn main() {
    let output = match run() {
        Some((result, duration)) => RunOutput::new(true, result, duration),
        None => RunOutput::new(false, false, 0),
    };
    println!("{}", output.to_json())
}

fn run() -> Option<(bool, u128)> {
    let input: String = io::read_to_string(io::stdin().lock()).ok()?;
    let state = parse(&input)?;

    let start = Instant::now();
    let result = solve(&state);
    let duration = start.elapsed();

    Some((result, duration.as_millis()))
}
