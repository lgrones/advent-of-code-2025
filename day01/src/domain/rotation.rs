use crate::domain::direction::Direction;

pub struct Rotation {
    direction: Direction,
    distance: i32,
}

impl Rotation {
    pub fn new(direction: char, distance: &str) -> Self {
        Rotation {
            direction: Direction::from_char(direction),
            distance: distance.parse().unwrap(),
        }
    }

    pub fn turn(&self, current_position: &mut i32) -> i32 {
        for _ in 0..self.distance {
            *current_position += self.direction.click();
            *current_position = (*current_position + 100) % 100;
        }

        (*current_position == 0) as i32
    }

    pub fn turn_with_intermediates(&self, current_position: &mut i32) -> i32 {
        let mut result = 0;

        for _ in 0..self.distance {
            *current_position += self.direction.click();
            *current_position = (*current_position + 100) % 100;
            result += (*current_position == 0) as i32
        }

        result
    }
}
