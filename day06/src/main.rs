use lib::{StopWatch, read_file};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut stopwatch = StopWatch::new();

    let homework = get_homework()?;

    stopwatch.start();

    let mut result = homework
        .iter()
        .map(|values| Problem::horizontal(values).calculate())
        .fold(0, |acc, x| acc + x);

    println!("PART 1: {result}");

    stopwatch.stop();

    stopwatch.start();

    result = homework
        .iter()
        .map(|values| Problem::vertical(values).calculate())
        .fold(0, |acc, x| acc + x);

    println!("PART 2: {result}");

    stopwatch.stop();

    Ok(())
}

#[derive(PartialEq)]
enum Operation {
    None,
    Add,
    Multiply,
}

struct Problem {
    values: Vec<u64>,
    operation: Operation,
}

impl Problem {
    fn horizontal(items: &Vec<String>) -> Self {
        let mut values = vec![];
        let mut operation = Operation::None;

        for value in items {
            if let Ok(v) = value.trim().parse() {
                values.push(v);
                continue;
            }

            operation = match value.trim() {
                "+" => Operation::Add,
                "*" => Operation::Multiply,
                _ => Operation::None,
            };
        }

        Problem { values, operation }
    }

    fn vertical(items: &Vec<String>) -> Self {
        let values = items[..items.len() - 1].to_vec();
        let mut vertical = vec![String::new(); values.len()];

        for i in 0..values[0].len() {
            for value in values.iter().map(|x| x.chars().collect::<Vec<_>>()) {
                vertical[i] += value[i].to_string().as_str();
            }
        }

        vertical.push(items.last().unwrap().to_string());

        Problem::horizontal(&vertical)
    }

    fn calculate(&self) -> u64 {
        if self.operation == Operation::None {
            panic!("Can only calculate with a valid operation")
        }

        if self.operation == Operation::Add {
            self.values.iter().fold(0, |acc, x| acc + x)
        } else {
            self.values.iter().fold(1, |acc, x| acc * x)
        }
    }
}

fn get_homework() -> Result<Vec<Vec<String>>, Box<dyn Error>> {
    read_file(|x| {
        let lines = x
            .lines()
            .map(|y| y.chars().collect())
            .collect::<Vec<Vec<char>>>();

        let mut result = vec![];
        let mut values = vec![String::new(); lines.len()];

        for i in 0..lines[0].len() {
            let column = lines.iter().map(|line| line[i]).collect::<Vec<_>>();

            if column.iter().all(|y| y == &' ') {
                result.push(values);
                values = vec![String::new(); lines.len()];
                continue;
            }

            for (j, c) in column.iter().enumerate() {
                values[j] += c.to_string().as_str();
            }
        }

        result.push(values);
        result
    })
}
