use lib::StopWatch;
use std::error::Error;

mod domain;

use crate::domain::{dial::Dial, method::PasswordMethod};

fn main() -> Result<(), Box<dyn Error>> {
    let mut stopwatch = StopWatch::new();

    stopwatch.start();

    let mut result = Dial::new(PasswordMethod::Simple)?.turn();

    println!("PART 1: {result}");

    stopwatch.stop();

    stopwatch.start();

    result = Dial::new(PasswordMethod::Method0x434C49434B)?.turn();

    println!("PART 2: {result}");

    stopwatch.stop();

    Ok(())
}
