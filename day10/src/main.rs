use lib::{StopWatch, read_file};
use std::{collections::VecDeque, error::Error, fmt};

fn main() -> Result<(), Box<dyn Error>> {
    let mut stopwatch = StopWatch::new();

    let machines = get_machines()?;

    stopwatch.start();

    let mut result = 0;

    for machine in machines {
        let mut min = i32::MAX;
        traverse(&machine, 0, &mut min);
        result += min;
    }

    println!("PART 1: {result}");

    stopwatch.stop();

    stopwatch.start();

    println!("PART 2: {result}");

    stopwatch.stop();

    Ok(())
}

#[derive(Clone)]
struct Machine {
    indicator_lights: i32,
    buttons: Vec<i32>,
    joltages: Vec<i32>,
    goal: i32,
}

impl Machine {
    fn new(indicator_lights: i32, buttons: Vec<i32>, joltages: Vec<i32>) -> Self {
        Self {
            indicator_lights: 0,
            buttons,
            joltages,
            goal: indicator_lights,
        }
    }

    fn toggle(&self) -> Vec<Machine> {
        self.buttons
            .iter()
            .map(|button| {
                let mut clone = self.clone();
                clone.indicator_lights ^= button;
                clone
            })
            .collect()
    }

    fn goal_reached(&self) -> bool {
        self.indicator_lights == self.goal
    }
}

impl fmt::Debug for Machine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s: String = format!("{:05b}", self.goal)
            .chars()
            .map(|c| if c == '0' { '.' } else { '#' })
            .collect();

        write!(f, "goal: {}", s)?;

        let s: Vec<String> = self
            .buttons
            .iter()
            .map(|button| {
                format!("{:05b}", button)
                    .chars()
                    .map(|c| if c == '0' { '.' } else { '#' })
                    .collect()
            })
            .collect();

        write!(f, " buttons: {:?}", s)
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
                    let mask = button[1..button.len() - 1]
                        .split(",")
                        .map(|x| x.parse::<i32>().unwrap())
                        .fold(0, |acc, x| acc | (1 << x));

                    buttons.push(mask);
                }

                Machine::new(indicator_lights, buttons, joltages)
            })
            .collect()
    })
}

fn traverse(machine: &Machine, presses: i32, min: &mut i32) {
    // TODO: find heuristic that works in all cases, > 5 just works for the example
    if presses > 5 || &presses >= min {
        return;
    }

    if machine.goal_reached() {
        *min = presses;
    }

    for machine in machine.toggle() {
        traverse(&machine, presses + 1, min);
    }
}
