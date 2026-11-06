use lib::{StopWatch, read_file};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut stopwatch = StopWatch::new();

    let tiles = get_tiles()?;

    stopwatch.start();

    let mut result = 0;

    for i in 0..tiles.len() - 1 {
        for j in i + 1..tiles.len() {
            result = result.max(Rectangle::new(tiles[i], tiles[j]).area());
        }
    }

    println!("PART 1: {result}");

    stopwatch.stop();

    stopwatch.start();

    result = 0;
    let polygon = Polygon::new(&tiles);

    for i in 0..tiles.len() - 1 {
        for j in i + 1..tiles.len() {
            let rect = Rectangle::new(tiles[i], tiles[j]);

            let area = rect.area();
            if area <= result {
                continue;
            }

            if polygon.intersects(&rect) || !polygon.contains(&rect) {
                continue;
            }

            result = area;
        }
    }

    println!("PART 2: {result}");

    stopwatch.stop();

    Ok(())
}

#[derive(Clone, Copy)]
struct Tile {
    x: u64,
    y: u64,
}

struct Rectangle {
    min_x: u64,
    max_x: u64,
    min_y: u64,
    max_y: u64,
}

impl Rectangle {
    fn new(c1: Tile, c2: Tile) -> Self {
        Rectangle {
            min_x: c1.x.min(c2.x),
            max_x: c1.x.max(c2.x),
            min_y: c1.y.min(c2.y),
            max_y: c1.y.max(c2.y),
        }
    }

    fn area(&self) -> u64 {
        let width = self.max_x - self.min_x + 1;
        let height = self.max_y - self.min_y + 1;

        width * height
    }
}

struct Edge {
    low: u64,
    high: u64,
    c: u64,
}

struct Polygon {
    vertical_edges: Vec<Edge>,
    horizontal_edges: Vec<Edge>,
}

impl Polygon {
    fn new(tiles: &[Tile]) -> Self {
        let mut vertical_edges = vec![];
        let mut horizontal_edges = vec![];

        for (t1, t2) in tiles.iter().zip(tiles.iter().cycle().skip(1)) {
            if t1.x == t2.x {
                vertical_edges.push(Edge {
                    low: t1.y.min(t2.y),
                    high: t1.y.max(t2.y),
                    c: t1.x,
                });
            } else {
                horizontal_edges.push(Edge {
                    low: t1.x.min(t2.x),
                    high: t1.x.max(t2.x),
                    c: t1.y,
                });
            }
        }

        Polygon {
            vertical_edges,
            horizontal_edges,
        }
    }

    fn intersects(&self, rect: &Rectangle) -> bool {
        let Rectangle {
            min_x,
            max_x,
            min_y,
            max_y,
        } = rect;

        self.vertical_edges
            .iter()
            .any(|Edge { low, high, c }| min_x < c && c < max_x && low < max_y && min_y < high)
            || self
                .horizontal_edges
                .iter()
                .any(|Edge { low, high, c }| min_y < c && c < max_y && low < max_x && min_x < high)
    }

    fn contains(&self, rect: &Rectangle) -> bool {
        let Rectangle { min_x, min_y, .. } = rect;

        let intersections = self
            .vertical_edges
            .iter()
            .filter(|&Edge { low, high, c }| c > min_x && low <= min_y && min_y < high)
            .count();

        intersections % 2 == 1
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
