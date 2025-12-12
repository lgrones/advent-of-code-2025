use lib::{StopWatch, read_file};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut stopwatch = StopWatch::new();

    let (ranges, ids) = get_ids()?;

    stopwatch.start();

    let mut result = ids
        .iter()
        .filter(|id| ranges.iter().any(|x| x.is_in_range(*id)))
        .count();

    println!("PART 1: {result}");

    stopwatch.stop();

    stopwatch.start();

    result = 0;

    // Get overlapping ranges and create new ranges that do not overlap
    // then just calculate id count by end - start

    println!("PART 2: {result}");

    stopwatch.stop();

    Ok(())
}

fn get_ids() -> Result<(Vec<Range>, Vec<u64>), Box<dyn Error>> {
    let mut ranges = vec![];
    let mut ids = vec![];

    read_file(|x| {
        let lines = x.lines();
        let mut is_id = false;

        for line in lines {
            if line.is_empty() {
                is_id = true;
                continue;
            }

            if !is_id {
                let limits = line
                    .split("-")
                    .map(|y| y.parse().unwrap())
                    .collect::<Vec<_>>();

                ranges.push(Range {
                    start: limits[0],
                    end: limits[1],
                });
            } else {
                ids.push(line.parse::<u64>().unwrap());
            }
        }
    })?;

    Ok((ranges, ids))
}

struct Range {
    start: u64,
    end: u64,
}

impl Range {
    pub fn is_in_range(&self, id: &u64) -> bool {
        id >= &self.start && id <= &self.end
    }
}
