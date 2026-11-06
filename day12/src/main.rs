use lib::{StopWatch, read_file};
use std::{error::Error, println, vec};

fn main() -> Result<(), Box<dyn Error>> {
    let mut stopwatch = StopWatch::new();

    let (presents, regions) = get_presents()?;

    let mut result = 0;

    stopwatch.start();

    for region in regions {
        let mut min_area = 0;
        let mut max_area = 0;

        for i in 0..6 {
            min_area += presents[i] * region.quantities[i];
            max_area += 9 * region.quantities[i];
        }

        if min_area > region.area {
            continue;
        }

        if max_area <= region.area {
            result += 1;
        }
    }

    println!("PART 1: {result}");

    stopwatch.stop();

    Ok(())
}

struct Region {
    area: i32,
    quantities: Vec<i32>,
}

fn get_presents() -> Result<(Vec<i32>, Vec<Region>), Box<dyn std::error::Error>> {
    let mut presents = vec![];
    let mut regions = vec![];

    read_file(|x| {
        let mut present_area = 0;

        for line in x.lines() {
            if line.contains("x") {
                let mut iter = line.split(": ");

                let area = iter
                    .next()
                    .unwrap()
                    .split("x")
                    .map(|x| x.parse::<i32>().unwrap())
                    .product();

                let quantities = iter
                    .next()
                    .unwrap()
                    .split_whitespace()
                    .map(|x| x.parse().unwrap())
                    .collect();

                regions.push(Region { area, quantities });

                continue;
            }

            if line.contains(":") {
                present_area = 0;
                continue;
            }

            if line == "" {
                presents.push(present_area);
                continue;
            }

            present_area += line.chars().filter(|x| *x == '#').count() as i32;
        }
    })?;

    Ok((presents, regions))
}
