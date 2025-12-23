use lib::{StopWatch, read_file};
use std::{collections::HashSet, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let mut stopwatch = StopWatch::new();

    let junction_boxes = get_junction_boxes()?;

    stopwatch.start();

    let distances = compute_distances(&junction_boxes)
        .into_iter()
        .take(1000)
        .collect::<Vec<_>>();

    let mut circuits = junction_boxes
        .iter()
        .map(|x| Circuit::new(x))
        .collect::<Vec<_>>();

    for (a, b) in &distances {
        let mut removed_circuits = Vec::new();

        circuits.retain(|x| {
            if x.has(a) || x.has(b) {
                removed_circuits.push(x.clone());
                false
            } else {
                true
            }
        });

        let mut merge_circuit = removed_circuits.iter().nth(0).unwrap().to_owned();

        merge_circuit.merge(removed_circuits.iter().nth(1));

        circuits.push(merge_circuit);
    }

    circuits.sort_by(|a, b| b.junction_boxes.len().cmp(&a.junction_boxes.len()));

    let result = circuits
        .iter()
        .take(3)
        .fold(1, |acc, curr| acc * curr.junction_boxes.len());

    println!("PART 1: {result}");

    stopwatch.stop();

    stopwatch.start();

    // Should be easy enough, as long as the time complexity doesn't explode
    // println!("PART 2: {input}");

    stopwatch.stop();

    Ok(())
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct JunctionBox {
    x: i32,
    y: i32,
    z: i32,
}

impl JunctionBox {
    fn new(dimensions: &str) -> Self {
        let d = dimensions
            .split(',')
            .map(|x| x.parse().unwrap())
            .collect::<Vec<_>>();

        let [x, y, z]: [_; 3] = d.try_into().unwrap();

        JunctionBox { x, y, z }
    }

    fn distance(&self, other: &JunctionBox) -> f32 {
        f32::sqrt(
            ((self.x - other.x) as f32).powf(2.0)
                + ((self.y - other.y) as f32).powf(2.0)
                + ((self.z - other.z) as f32).powf(2.0),
        )
    }
}

#[derive(PartialEq, Clone)]
struct Circuit {
    junction_boxes: HashSet<JunctionBox>,
}

impl Circuit {
    fn new(junction_box: &JunctionBox) -> Self {
        Circuit {
            junction_boxes: HashSet::from_iter([junction_box.clone()]),
        }
    }

    fn has(&self, other: &JunctionBox) -> bool {
        self.junction_boxes.contains(other)
    }

    fn merge(&mut self, other: Option<&Circuit>) {
        if other.is_none() {
            return;
        }

        self.junction_boxes
            .extend(other.unwrap().junction_boxes.to_owned())
    }
}

fn get_junction_boxes() -> Result<Vec<JunctionBox>, Box<dyn Error>> {
    read_file(|x| x.lines().map(|y| JunctionBox::new(y)).collect())
}

fn compute_distances(junction_boxes: &Vec<JunctionBox>) -> Vec<(JunctionBox, JunctionBox)> {
    let mut pairs: Vec<_> = (0..junction_boxes.len())
        .flat_map(|i| {
            (i + 1..junction_boxes.len())
                .map(move |j| (junction_boxes[i].clone(), junction_boxes[j].clone()))
        })
        .collect();

    pairs.sort_by(|(a1, b1), (a2, b2)| a1.distance(b1).partial_cmp(&a2.distance(b2)).unwrap());

    pairs
}
