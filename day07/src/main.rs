use lib::{StopWatch, read_file};
use std::{
    collections::{HashMap, HashSet},
    error::Error,
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut stopwatch = StopWatch::new();

    let map = get_map()?;
    let start = map[0].iter().position(|x| x == &'S').unwrap();

    stopwatch.start();

    let mut result = move_down(&map, start);

    println!("PART 1: {result}");

    stopwatch.stop();

    stopwatch.start();

    result = traverse(&Tachyon { x: start, y: 0 }, &map, &mut HashMap::new());

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
    fn beam(&self, map: &Vec<Vec<char>>) -> Vec<Tachyon> {
        let mut next = self.clone();
        next.y += 1;

        if next.y == map.len() {
            return vec![];
        }

        if map[next.y][next.x] == '.' {
            return vec![next];
        }

        vec![
            Tachyon {
                x: next.x - 1,
                y: next.y,
            },
            Tachyon {
                x: next.x + 1,
                y: next.y,
            },
        ]
    }
}

fn get_map() -> Result<Vec<Vec<char>>, Box<dyn Error>> {
    read_file(|x| x.lines().map(|y| y.chars().collect()).collect())
}

fn move_down(map: &Vec<Vec<char>>, start: usize) -> u64 {
    let mut result = 0;
    let mut tachyons = HashSet::from([Tachyon { x: start, y: 0 }]);

    while !tachyons.is_empty() {
        let mut next = vec![];

        while let Some(tachyon) = tachyons.iter().next().cloned() {
            tachyons.remove(&tachyon);
            let beamed = tachyon.beam(map);

            if beamed.len() == 2 {
                result += 1;
            }

            next.extend(beamed);
        }

        tachyons.extend(next);
    }

    result
}

fn traverse(tachyon: &Tachyon, map: &Vec<Vec<char>>, cache: &mut HashMap<Tachyon, u64>) -> u64 {
    if cache.contains_key(tachyon) {
        return *cache.get(tachyon).unwrap();
    }

    let tachyons = tachyon.beam(map);

    if tachyons.is_empty() {
        return 1;
    }

    let mut result = 0;

    for tachyon in &tachyons {
        result += traverse(&tachyon, map, cache);
    }

    if tachyons.len() == 2 {
        cache.insert(tachyon.clone(), result);
    }

    result
}
