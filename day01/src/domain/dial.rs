use std::error::Error;

use lib::read_file;

use crate::domain::{method::PasswordMethod, rotation::Rotation};

pub struct Dial {
    method: PasswordMethod,
    rotations: Vec<Rotation>,
}

impl Dial {
    pub fn new(method: PasswordMethod) -> Result<Self, Box<dyn Error>> {
        let rotations = read_file(|x| {
            x.lines()
                .map(|y| {
                    let (direction, distance) = y.split_at(1);
                    Rotation::new(direction.chars().next().unwrap(), distance)
                })
                .collect()
        })?;

        Ok(Dial { method, rotations })
    }

    pub fn turn(&self) -> i32 {
        let mut result = 0;
        let mut current_position = 50;

        for rotation in &self.rotations {
            result += match self.method {
                PasswordMethod::Simple => rotation.turn(&mut current_position),
                PasswordMethod::Method0x434C49434B => {
                    rotation.turn_with_intermediates(&mut current_position)
                }
            }
        }

        result
    }
}
