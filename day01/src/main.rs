use lib::{StopWatch, read_file};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut stopwatch = StopWatch::new();

    let rotations = get_rotations()?;

    stopwatch.start();

    let mut result = 0;
    let mut current_position = 50;
    let n = 100;

    for steps in &rotations {
        current_position = ((current_position + steps % n) + n) % n;

        if current_position == 0 {
            result += 1;
        }
    }

    println!("PART 1: {result}");

    stopwatch.stop();

    stopwatch.start();

    result = 0;
    current_position = 50;

    for steps in rotations {
        let next_position = current_position + steps;

        if next_position > 99 || next_position <= 0 {
            result += (next_position.abs() as f32 / 100.0).ceil() as i32;
        }

        current_position = ((next_position % n) + n) % n;
    }

    println!("PART 2: {result}");

    stopwatch.stop();

    Ok(())
}

fn get_rotations() -> Result<Vec<i32>, Box<dyn Error>> {
    read_file(|x| {
        x.lines()
            .map(|y| {
                let (dir, steps) = y.split_at(1);
                let steps = steps.parse::<i32>().unwrap();

                if dir.chars().next().unwrap() == 'R' {
                    steps
                } else {
                    -steps
                }
            })
            .collect()
    })
}
