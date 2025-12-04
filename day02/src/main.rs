use lib::{StopWatch, read_file};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut stopwatch = StopWatch::new();

    let ranges = get_ranges()?;

    stopwatch.start();

    let mut result = 0;

    for range in &ranges {
        for i in range.start..=range.end {
            let digits = i.to_string();

            if digits.len() % 2 == 1 {
                continue;
            }

            let (a, b) = digits.split_at(digits.len() / 2);

            if a == b {
                result += i;
            }
        }
    }

    println!("PART 1: {result}");

    stopwatch.stop();

    stopwatch.start();

    result = 0;

    for range in &ranges {
        for i in range.start..=range.end {
            let digits = i.to_string().chars().collect::<Vec<_>>();

            for chunk_size in 1..=(digits.len() as f64 / 2.0).floor() as usize {
                let mut chunks = digits.chunks(chunk_size);

                let pattern = chunks.next().unwrap().iter().collect::<String>();

                let mut valid = false;

                while let Some(p) = chunks.next() {
                    if p.iter().collect::<String>() != pattern {
                        valid = true;
                        break;
                    }
                }

                if !valid {
                    result += i;
                    break;
                }
            }
        }
    }

    println!("PART 2: {result}");

    stopwatch.stop();

    Ok(())
}

struct Range {
    start: u64,
    end: u64,
}

fn get_ranges() -> Result<Vec<Range>, Box<dyn Error>> {
    read_file(|x| {
        x.split(",")
            .map(|y| {
                let mut block = y.split("-");
                Range {
                    start: block.next().unwrap().parse().unwrap(),
                    end: block.next().unwrap().parse().unwrap(),
                }
            })
            .collect::<Vec<_>>()
    })
}
