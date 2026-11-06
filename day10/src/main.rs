use itertools::{self, Itertools};
use lib::{StopWatch, read_file};
use std::{collections::VecDeque, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let mut stopwatch = StopWatch::new();

    let machines = get_machines()?;

    stopwatch.start();

    let mut result = 0;

    for machine in &machines {
        // every combination without replacment of k (0 to buttons.len()) buttons, smallest set first
        for button_combination in machine.button_masks.iter().powerset() {
            // check if the combination produces the light pattern by XORing 0 with each button
            if button_combination
                .iter()
                .fold(0, |acc, button| acc ^ *button)
                == machine.light_mask
            {
                result = result + button_combination.len();
                break;
            }
        }
    }

    println!("PART 1: {result}");

    stopwatch.stop();

    stopwatch.start();

    result = 0;

    for machine in &machines {
        let mut k = 1;
        let mut found = false;

        while !found {
            // every combination with replacement of k buttons, with k > 0
            for button_combination in machine.buttons.iter().combinations_with_replacement(k) {
                // check if the combination produces the joltage targets
                let mut counters = machine.joltages.to_vec();

                for button in button_combination {
                    for counter_index in button {
                        counters[*counter_index as usize] -= 1;
                    }
                }

                if counters.iter().all(|x| *x == 0) {
                    found = true;
                    result += k;
                    break;
                }
            }

            k += 1;
        }
    }

    println!("PART 2: {result}");

    stopwatch.stop();

    Ok(())
}

#[derive(Clone)]
struct Machine {
    button_masks: Vec<i32>,
    buttons: Vec<Vec<i32>>,
    joltages: Vec<i32>,
    light_mask: i32,
}

impl Machine {
    fn new(indicator_lights: i32, buttons: Vec<Vec<i32>>, joltages: Vec<i32>) -> Self {
        Self {
            buttons: buttons.to_vec(),
            joltages,
            button_masks: buttons
                .iter()
                .map(|button| button.iter().fold(0, |acc, x| acc | (1 << x)))
                .collect(),
            light_mask: indicator_lights,
        }
    }
}

fn get_machines() -> Result<Vec<Machine>, Box<dyn Error>> {
    read_file(|x| {
        x.lines()
            .map(|line| {
                let mut components = line.split_whitespace().collect::<VecDeque<_>>();

                let lights = components.pop_front().unwrap();
                let indicator_lights = lights[1..lights.len() - 1]
                    .chars()
                    .rev()
                    .map(|x| if x == '#' { 1 } else { 0 })
                    .fold(0, |acc, bit| (acc << 1) | bit);

                let joltage = components.pop_back().unwrap();
                let joltages = joltage[1..joltage.len() - 1]
                    .split(",")
                    .map(|x| x.parse().unwrap())
                    .collect();

                let mut buttons = vec![];

                for button in components {
                    let indices = button[1..button.len() - 1]
                        .split(",")
                        .map(|x| x.parse().unwrap())
                        .collect();

                    buttons.push(indices);
                }

                Machine::new(indicator_lights, buttons, joltages)
            })
            .collect()
    })
}
