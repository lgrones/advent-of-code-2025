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

    result = graph.get_paths_with_dac_and_fft();

    println!("PART 2: {result}");

    stopwatch.stop();

    Ok(())
}

type DeviceId = usize;
type DeviceName = String;

struct Device {
    name: DeviceName,
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
            name: name.to_string(),
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

    fn get_paths(&self) -> i32 {
        traverse(
            &self,
            self.index["you"],
            &mut vec![false; self.devices.len()],
            false,
        )
    }

    fn get_paths_with_dac_and_fft(&self) -> i32 {
        traverse(
            &self,
            self.index["svr"],
            &mut vec![false; self.devices.len()],
            true,
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
    device_id: DeviceId,
    visited: &mut Vec<bool>,
    with_dac_and_fft: bool,
) -> i32 {
    if visited[device_id] {
        return 0;
    }

    if graph.devices[device_id].name == "out" {
        return (!with_dac_and_fft || (visited[graph.index["dac"]] && visited[graph.index["fft"]]))
            as i32;
    }

    visited[device_id] = true;

    let mut result = 0;
    for &next in &graph.devices[device_id].outputs {
        result += traverse(graph, next, visited, with_dac_and_fft);
    }

    visited[device_id] = false;

    result
}
