use lib::{StopWatch, read_file};
use std::collections::HashMap;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut stopwatch = StopWatch::new();

    let graph = get_graph()?;

    stopwatch.start();

    let mut result = graph.get_paths();

    println!("PART 1: {result}");

    stopwatch.stop();

    stopwatch.start();

    result = graph.get_paths_through_dac_and_fft();

    println!("PART 2: {result}");

    stopwatch.stop();

    Ok(())
}

type DeviceId = usize;
type DeviceName = String;

struct Device {
    outputs: Vec<DeviceId>,
}

struct Graph {
    devices: Vec<Device>,
    index: HashMap<DeviceName, DeviceId>,
}

impl Graph {
    fn new() -> Self {
        Self {
            devices: Vec::new(),
            index: HashMap::new(),
        }
    }

    fn get_or_insert(&mut self, name: &str) -> DeviceId {
        if let Some(&id) = self.index.get(name) {
            return id;
        }

        let id = self.devices.len();

        self.devices.push(Device {
            outputs: Vec::new(),
        });
        self.index.insert(name.to_string(), id);

        id
    }

    fn add_edge(&mut self, from: &str, to: &str) {
        let from_id = self.get_or_insert(from);
        let to_id = self.get_or_insert(to);

        self.devices[from_id].outputs.push(to_id);
    }

    fn get_paths(&self) -> u64 {
        traverse(
            &self,
            self.index["you"],
            self.index["out"],
            &mut vec![None; self.devices.len()],
        )
    }

    fn get_paths_through_dac_and_fft(&self) -> u64 {
        traverse(
            &self,
            self.index["svr"],
            self.index["fft"],
            &mut vec![None; self.devices.len()],
        ) * traverse(
            &self,
            self.index["fft"],
            self.index["dac"],
            &mut vec![None; self.devices.len()],
        ) * traverse(
            &self,
            self.index["dac"],
            self.index["out"],
            &mut vec![None; self.devices.len()],
        ) + traverse(
            &self,
            self.index["svr"],
            self.index["dac"],
            &mut vec![None; self.devices.len()],
        ) * traverse(
            &self,
            self.index["dac"],
            self.index["fft"],
            &mut vec![None; self.devices.len()],
        ) * traverse(
            &self,
            self.index["fft"],
            self.index["out"],
            &mut vec![None; self.devices.len()],
        )
    }
}

fn get_graph() -> Result<Graph, Box<dyn std::error::Error>> {
    let mut devices = Graph::new();

    read_file(|x| {
        x.lines().for_each(|y| {
            let mut iter = y.split(": ");
            let name = iter.next().unwrap();

            for out in iter.next().unwrap().split_whitespace() {
                devices.add_edge(name, out);
            }
        });
    })?;

    Ok(devices)
}

fn traverse(
    graph: &Graph,
    node: DeviceId,
    target: DeviceId,
    visited: &mut Vec<Option<u64>>,
) -> u64 {
    if node == target {
        return 1;
    }

    if let Some(total) = visited[node] {
        return total;
    }

    let mut result = 0;
    for &next in &graph.devices[node].outputs {
        result += traverse(graph, next, target, visited);
    }

    visited[node] = Some(result);

    result
}
