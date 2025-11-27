use lib::{StopWatch, read_file};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut stopwatch = StopWatch::new();

    let input = read_file(|x| x)?;

    stopwatch.start();

    println!("{input}");

    stopwatch.stop();

    Ok(())
}
