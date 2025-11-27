use itertools::Itertools;
use lib::{StopWatch, read_file};
use once_cell::sync::Lazy;
use std::{collections::HashMap, error::Error, iter::repeat, vec};

#[derive(Clone)]
struct Coordinate {
    x: i8,
    y: i8,
}

static NUMBER_KEYPAD: Lazy<HashMap<char, Coordinate>> = Lazy::new(|| {
    HashMap::from([
        ('7', Coordinate { x: 0, y: 3 }),
        ('8', Coordinate { x: 1, y: 3 }),
        ('9', Coordinate { x: 2, y: 3 }),
        ('4', Coordinate { x: 0, y: 2 }),
        ('5', Coordinate { x: 1, y: 2 }),
        ('6', Coordinate { x: 2, y: 2 }),
        ('1', Coordinate { x: 0, y: 1 }),
        ('2', Coordinate { x: 1, y: 1 }),
        ('3', Coordinate { x: 2, y: 1 }),
        ('0', Coordinate { x: 1, y: 0 }),
        ('A', Coordinate { x: 2, y: 0 }),
    ])
});

static DIRECTIONAL_KEYPAD: Lazy<HashMap<char, Coordinate>> = Lazy::new(|| {
    HashMap::from([
        ('^', Coordinate { x: 1, y: 1 }),
        ('A', Coordinate { x: 2, y: 1 }),
        ('<', Coordinate { x: 0, y: 0 }),
        ('v', Coordinate { x: 1, y: 0 }),
        ('>', Coordinate { x: 2, y: 0 }),
    ])
});

fn main() -> Result<(), Box<dyn Error>> {
    let mut stopwatch = StopWatch::new();

    let codes = read_file(|x| {
        x.split('\n')
            .map(|s| s.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>()
    })?;

    print!("Precomputation ");

    stopwatch.start();

    let number_keypad = precompute_keypad(&NUMBER_KEYPAD, &Coordinate { x: 0, y: 0 });
    let directional_keypad = precompute_keypad(&DIRECTIONAL_KEYPAD, &Coordinate { x: 0, y: 1 });

    stopwatch.stop();

    stopwatch.start();

    let mut possible_paths;
    let mut result = 0;

    for code in codes {
        possible_paths = compute_paths(
            &number_keypad,
            &['A'].iter().chain(&code).copied().collect(),
        );

        for _ in 1..26 {
            let mut result = vec![];

            for possible_path in &possible_paths {
                result.extend(compute_paths(
                    &directional_keypad,
                    &['A'].iter().chain(possible_path).copied().collect(),
                ));
            }

            let min_length = result.iter().map(|x| x.len()).min().unwrap();

            possible_paths = result
                .into_iter()
                .filter(|x| x.len() == min_length)
                .collect();
        }

        let code_nr = code
            .into_iter()
            .collect::<String>()
            .trim_matches('A')
            .to_string();

        result += possible_paths[0].len() * usize::from_str_radix(&code_nr, 10).unwrap();
    }

    print!("{result} ");
    stopwatch.stop();

    Ok(())
}

fn compute_paths(
    keypad: &HashMap<(char, char), Vec<Vec<char>>>,
    sequence: &Vec<char>,
) -> Vec<Vec<char>> {
    let mut possible_paths = vec![];

    for keys in sequence.windows(2) {
        let paths = keypad.get(&(keys[0], keys[1])).unwrap();

        if possible_paths.is_empty() {
            possible_paths = paths.clone();
            continue;
        }

        let mut result = vec![];

        for possible_path in possible_paths {
            for path_section in paths {
                result.push(possible_path.iter().chain(path_section).cloned().collect());
            }
        }

        possible_paths = result;
    }

    let min_length = possible_paths.iter().map(|x| x.len()).min().unwrap();

    possible_paths
        .into_iter()
        .filter(|x| x.len() == min_length)
        .collect()
}

// returns a map of (from, to) -> Vec<shortest path>
fn precompute_keypad(
    keypad: &Lazy<HashMap<char, Coordinate>>,
    forbidden_pos: &Coordinate,
) -> HashMap<(char, char), Vec<Vec<char>>> {
    let mut result = HashMap::new();

    for positions in keypad.iter().cartesian_product(keypad.iter()) {
        result.insert(
            (*positions.0.0, *positions.1.0),
            get_shortest_paths(positions.0.1, positions.1.1, forbidden_pos),
        );
    }

    result
}

fn get_shortest_paths(
    from: &Coordinate,
    to: &Coordinate,
    forbidden_pos: &Coordinate,
) -> Vec<Vec<char>> {
    let (dx, dy) = (to.x - from.x, to.y - from.y);
    let mut moves = vec![];

    moves.extend(repeat(if dx > 0 { '>' } else { '<' }).take(dx.abs() as usize));
    moves.extend(repeat(if dy > 0 { '^' } else { 'v' }).take(dy.abs() as usize));

    moves
        .iter()
        .permutations(moves.len())
        .unique()
        .filter(|path| !panics(from.clone(), path, forbidden_pos))
        .map(|x| x.into_iter().chain(['A'].iter()).copied().collect())
        .collect()
}

fn panics(from: Coordinate, path: &Vec<&char>, forbidden_pos: &Coordinate) -> bool {
    let mut pos = from;

    for key in path {
        match key {
            '<' => {
                pos = Coordinate {
                    x: pos.x - 1,
                    y: pos.y,
                }
            }
            '>' => {
                pos = Coordinate {
                    x: pos.x + 1,
                    y: pos.y,
                }
            }
            'v' => {
                pos = Coordinate {
                    x: pos.x,
                    y: pos.y - 1,
                }
            }
            '^' => {
                pos = Coordinate {
                    x: pos.x,
                    y: pos.y + 1,
                }
            }
            _ => panic!("unknown direction"),
        }

        if pos.x == forbidden_pos.x && pos.y == forbidden_pos.y {
            return true;
        }
    }

    false
}
