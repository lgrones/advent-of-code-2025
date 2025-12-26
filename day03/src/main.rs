use lib::{StopWatch, read_file};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut stopwatch = StopWatch::new();

    let banks = get_banks()?;

    stopwatch.start();

    let mut result = banks
        .iter()
        .fold(0, |acc, curr| acc + curr.search_max_with_n_places(2));

    println!("PART 1: {result}");

    stopwatch.stop();

    stopwatch.start();

    result = banks
        .iter()
        .fold(0, |acc, curr| acc + curr.search_max_with_n_places(12));

    println!("PART 2: {result}");

    stopwatch.stop();

    Ok(())
}

struct Bank {
    batteries: Vec<u64>,
}

impl Bank {
    fn search_max_with_n_places(&self, n: usize) -> u64 {
        let mut index = 0;
        let mut result = 0;

        for exponent in (0..n).rev() {
            let bank = &self.batteries[index..self.batteries.len() - exponent];
            let max = bank.iter().max().unwrap();

            index = 1 + self
                .batteries
                .iter()
                .enumerate()
                .position(|(i, x)| i >= index && x == max)
                .unwrap();

            result += 10u64.pow(exponent as u32) * max;
        }

        result
    }
}

fn get_banks() -> Result<Vec<Bank>, Box<dyn Error>> {
    read_file(|x| {
        x.lines()
            .map(|y| Bank {
                batteries: y.chars().map(|z| z.to_digit(10).unwrap() as u64).collect(),
            })
            .collect()
    })
}
