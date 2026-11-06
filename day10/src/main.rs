use itertools::{self, Itertools};
use lib::{StopWatch, read_file};
use std::{
    collections::{HashMap, VecDeque},
    error::Error,
};

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

    // Part 2 written by Claude (Opus 5.5)
    for machine in &machines {
        // every set of buttons pressed once each, as (effect on the counters, number of presses)
        let subsets: Vec<(Vec<i32>, usize)> = machine
            .buttons
            .iter()
            .powerset()
            .map(|subset| {
                let mut effect = vec![0; machine.joltages.len()];

                for button in &subset {
                    for counter_index in *button {
                        effect[*counter_index as usize] += 1;
                    }
                }

                (effect, subset.len())
            })
            .collect();

        result += fewest_presses(&machine.joltages, &subsets, &mut HashMap::new())
            .expect("machine should be solvable");
    }

    println!("PART 2: {result}");

    stopwatch.stop();

    Ok(())
}

/// Written by Claude (Opus 5.5)
///
/// Any solution splits into an odd layer (each button pressed 0 or 1 times) plus twice a
/// smaller solution, so target = subset + 2 * smaller, and smaller = (target - subset) / 2.
/// That is the same puzzle at half the size, which gives the recurrence:
///
///   f(0) = 0
///   f(target) = min over valid subsets of: presses + 2 * f((target - effect) / 2)
///
/// A subset is valid if every remaining counter is even and non-negative. Evenness is Part 1
/// again, with lights = target mod 2. Returns None if the target can't be reached.
fn fewest_presses(
    target: &[i32],
    subsets: &[(Vec<i32>, usize)],
    memo: &mut HashMap<Vec<i32>, Option<usize>>,
) -> Option<usize> {
    if target.iter().all(|x| *x == 0) {
        return Some(0);
    }

    if let Some(known) = memo.get(target) {
        return *known;
    }

    let mut best = None;

    // all matching subsets count, not just the smallest: fewest presses now isn't fewest overall
    for (effect, presses) in subsets {
        let remaining: Vec<i32> = target.iter().zip(effect).map(|(t, e)| t - e).collect();

        if remaining.iter().any(|x| *x < 0 || x % 2 != 0) {
            continue;
        }

        let halved: Vec<i32> = remaining.iter().map(|x| x / 2).collect();

        if let Some(rest) = fewest_presses(&halved, subsets, memo) {
            let total = presses + 2 * rest;
            best = Some(best.map_or(total, |current: usize| current.min(total)));
        }
    }

    memo.insert(target.to_vec(), best);

    best
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
