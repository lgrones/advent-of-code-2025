use lib::{StopWatch, read_file};
use std::{collections::HashSet, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let mut stopwatch = StopWatch::new();

    let map = get_map()?;

    stopwatch.start();

    let mut result = 0;
    let x = map[0].iter().enumerate().find(|x| x.1 == &'S').unwrap().0;

    let mut tachyons = HashSet::from([Tachyon { x, y: 0 }]);

    while !tachyons.is_empty() {
        let mut next = vec![];

        while let Some(tachyon) = tachyons.iter().next().cloned() {
            tachyons.remove(&tachyon);
            let beamed = tachyon.beam(&map);

            if beamed.len() == 2 {
                result += 1;
            }

            next.extend(beamed);
        }

        tachyons.extend(next);
    }

    println!("PART 1: {result}");

    stopwatch.stop();

    stopwatch.start();

    result = traverse(Tachyon { x, y: 0 }, &map);

    println!("PART 2: {result}");

    stopwatch.stop();

    Ok(())
}

#[derive(Hash, Eq, PartialEq, Clone)]
struct Tachyon {
    x: usize,
    y: usize,
}

impl Tachyon {
    fn beam(mut self, map: &Vec<Vec<char>>) -> Vec<Tachyon> {
        self.y += 1;

        if self.y == map.len() {
            return vec![];
        }

        if map[self.y][self.x] == '.' {
            return vec![self];
        }

        vec![
            Tachyon {
                x: self.x - 1,
                y: self.y,
            },
            Tachyon {
                x: self.x + 1,
                y: self.y,
            },
        ]
    }
}

fn get_map() -> Result<Vec<Vec<char>>, Box<dyn Error>> {
    read_file(|x| x.lines().map(|y| y.chars().collect()).collect())
}

fn traverse(tachyon: Tachyon, map: &Vec<Vec<char>>) -> i32 {
    let tachyons = tachyon.beam(map);

    if tachyons.is_empty() {
        return 1;
    }

    let mut result = 0;

    for tachyon in tachyons {
        result += traverse(tachyon, map)
    }

    result
}
