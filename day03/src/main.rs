use lib::{StopWatch, read_file};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut stopwatch = StopWatch::new();

    let banks = get_banks()?;

    stopwatch.start();

    let mut result = 0;

    for mut bank in banks {
        let joltage;
        let (max, max_index) = search_max(&bank, 0);

        if max_index == bank.len() - 1 {
            bank.reverse();
            let (max2, _) = search_max(&bank, 1);
            joltage = max2 * 10 + max;
        } else {
            let (max2, _) = search_max(&bank, max_index + 1);
            joltage = max * 10 + max2;
        }

        result += joltage;
    }

    println!("PART 1: {result}");

    stopwatch.stop();

    Ok(())
}

fn search_max(bank: &Vec<char>, start: usize) -> (u32, usize) {
    let mut max = 0;
    let mut max_index = 0;

    for i in start..bank.len() {
        let curr = bank[i].to_digit(10).unwrap();

        if max < curr {
            max = curr;
            max_index = i;
        }
    }

    (max, max_index)
}

fn get_banks() -> Result<Vec<Vec<char>>, Box<dyn Error>> {
    read_file(|x| x.lines().map(|y| y.chars().collect()).collect())
}
