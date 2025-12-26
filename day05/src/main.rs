use lib::{StopWatch, read_file};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut stopwatch = StopWatch::new();

    let (mut ranges, ids) = get_ids()?;

    stopwatch.start();

    let mut result = ids
        .into_iter()
        .filter(|id| ranges.iter().any(|range| range.contains(id)))
        .count() as u64;

    println!("PART 1: {result}");

    stopwatch.stop();

    stopwatch.start();

    ranges.sort_by_key(|r| r.start);

    let mut res: Vec<Range> = vec![];

    for range in ranges.drain(..) {
        if let Some(last) = res.last_mut()
            && last.is_overlap(&range)
        {
            last.merge(&range);
            continue;
        }

        res.push(range);
    }

    result = res.iter().fold(0, |acc, curr| acc + curr.count_ids());

    println!("PART 2: {result}");

    stopwatch.stop();

    Ok(())
}

#[derive(Eq, PartialEq, Hash)]
struct Range {
    start: u64,
    end: u64,
}

impl Range {
    fn contains(&self, id: &u64) -> bool {
        id >= &self.start && id <= &self.end
    }

    fn is_overlap(&self, range: &Range) -> bool {
        self.start <= range.end && range.start <= self.end
    }

    fn merge(&mut self, range: &Range) {
        self.start = self.start.min(range.start);
        self.end = self.end.max(range.end);
    }

    fn count_ids(&self) -> u64 {
        self.end - self.start + 1
    }
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
