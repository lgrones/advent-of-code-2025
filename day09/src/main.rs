use lib::{StopWatch, read_file};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut stopwatch = StopWatch::new();

    let tiles = get_tiles()?;

    stopwatch.start();

    let mut result = 0;

    for i in 0..tiles.len() - 1 {
        for j in i + 1..tiles.len() {
            result = result.max(tiles[i].area(&tiles[j]));
        }
    }

    println!("PART 1: {result}");

    stopwatch.stop();

    stopwatch.start();

    println!("PART 2: {result}");

    stopwatch.stop();

    Ok(())
}

struct Tile {
    x: u64,
    y: u64,
}

impl Tile {
    fn area(&self, other: &Tile) -> u64 {
        let width = self.x.max(other.x) - self.x.min(other.x) + 1;
        let height = self.y.max(other.y) - self.y.min(other.y) + 1;

        width * height
    }
}

fn get_tiles() -> Result<Vec<Tile>, Box<dyn Error>> {
    read_file(|x| {
        x.lines()
            .map(|y| {
                let coords = y.split(",").collect::<Vec<_>>();

                Tile {
                    x: coords[0].parse().unwrap(),
                    y: coords[1].parse().unwrap(),
                }
            })
            .collect()
    })
}
