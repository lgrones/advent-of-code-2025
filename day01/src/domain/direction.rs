pub enum Direction {
    Right,
    Left,
}

impl Direction {
    pub fn from_char(c: char) -> Self {
        match c {
            'R' => Direction::Right,
            'L' => Direction::Left,
            _ => panic!("Cannot construct direction from {c}"),
        }
    }

    pub fn click(&self) -> i32 {
        match *self {
            Direction::Left => -1,
            Direction::Right => 1,
        }
    }
}
