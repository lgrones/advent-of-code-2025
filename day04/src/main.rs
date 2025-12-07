use lib::{StopWatch, read_file};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut stopwatch = StopWatch::new();

    let mut rolls = get_rolls()?;

    stopwatch.start();

    let mut result = get_removable_rolls(&rolls).len();

    println!("PART 1: {result}");

    stopwatch.stop();

    stopwatch.start();

    result = 0;
    let mut removable_rolls = get_removable_rolls(&rolls);

    while !removable_rolls.is_empty() {
        for (y, x) in removable_rolls {
            rolls[y][x] = '.';
            result += 1;
        }

        removable_rolls = get_removable_rolls(&rolls);
    }

    println!("PART 2: {result}");

    stopwatch.stop();

    Ok(())
}

fn get_rolls() -> Result<Vec<Vec<char>>, Box<dyn Error>> {
    read_file(|x| x.lines().map(|y| y.chars().collect()).collect())
}

fn get_removable_rolls(rolls: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut indexes = vec![];

    for y in 0..rolls.len() {
        for x in 0..rolls[0].len() {
            if rolls[y][x] == '.' {
                continue;
            }

            let mut roll_count = 0;

            for i in -1isize..=1 {
                for j in -1isize..=1 {
                    let has_tile = rolls
                        .get((y as isize + i) as usize)
                        .and_then(|line| line.get((x as isize + j) as usize))
                        .map(|roll| roll == &'@');

                    if let Some(is_roll) = has_tile
                        && is_roll
                    {
                        roll_count += 1;
                    }
                }
            }

            if roll_count <= 4 {
                indexes.push((y, x));
            }
        }
    }

    indexes
}
