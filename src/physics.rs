

use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

// get postion
#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn move_to_dir(&mut self, dir: Direction) {
        match dir {
            Direction::Up => self.y -= 1,
            Direction::Right => self.x += 1,
            Direction::Left => self.x -= 1,
            Direction::Down => self.y += 1,
        }
    }
}

//possible directions
#[derive(Copy, Debug, Clone, PartialEq)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

// implementing opposite
impl Direction {
    pub fn opposite(&self) -> Direction {
        match *self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}
// Random positioning at start of the game
impl Distribution<Direction> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Direction {
        match rng.gen_range(0..=3) {
            0 => Direction::Up,
            1 => Direction::Right,
            2 => Direction::Down,
            _ => Direction::Left,
        }
    }
}
