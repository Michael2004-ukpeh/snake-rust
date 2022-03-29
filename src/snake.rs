use crate::color::SNAKE;
use crate::draw::{draw_block, draw_snake_head};
use crate::physics::*;
use piston_window::{Context, G2d};
use rand;
use std::collections::LinkedList;

pub struct Snake {
    direction: Direction,
    head: Position,
    body: LinkedList<Position>,
    tail: Option<Position>,
}

impl Snake {
    pub fn new(head: Position) -> Snake {
        let direction: Direction = rand::random();
        let (x, y) = (head.x, head.y);
        let mut body: LinkedList<Position> = LinkedList::new();
        body.push_back(Position { x: x + 2, y });
        body.push_back(Position { x: x + 1, y });
        body.push_back(Position { x, y });
        // match direction {
        //     Direction::Up => {
        //         for i in 1..3 {
        //             body.push_back(Position { x, y: y + i })
        //         }
        //     }
        //     Direction::Down => {
        //         for i in 1..3 {
        //             body.push_back(Position { x, y: y - i })
        //         }
        //     }
        //     Direction::Right => {
        //         for i in 1..3 {
        //             body.push_back(Position { x: x - i, y })
        //         }
        //     }
        //     Direction::Left => {
        //         for i in 1..3 {
        //             body.push_back(Position { x: x + i, y })
        //         }
        //     }
        // }

        Self {
            direction: Direction::Right,
            head,
            body,
            tail: Some(Position { x: x - 1, y }),
        }
    }

    pub fn draw(&self, ctx: &Context, g: &mut G2d) {
        for block in &self.body {
            draw_block(SNAKE, block, &ctx, g);
        }
        draw_snake_head(&ctx, g, SNAKE, &self.head, &self.direction);
    }

    pub fn head_position(&self) -> (i32, i32) {
        let head_pos = self.head;
        (head_pos.x, head_pos.y)
    }

    pub fn move_forward(&mut self, dir: Option<Direction>) {
        match dir {
            Some(d) => self.direction = d,
            None => (),
        }

        let (x, y) = self.head_position();

        self.body.push_front(self.head.clone());

        match self.direction {
            Direction::Up => Position { x: x, y: y - 1 },
            Direction::Down => Position { x: x, y: y + 1 },
            Direction::Right => Position { x: x + 1, y: y },
            Direction::Left => Position { x: x - 1, y: y },
        };
        let removed_positon = self.body.pop_back().unwrap();
        self.tail = Some(removed_positon);
    }

    pub fn head_direction(&self) -> Direction {
        self.direction
    }

    pub fn restore_tail(&mut self) {
        let blk = self.tail.clone().unwrap();
        self.body.push_back(blk);
    }

    pub fn get_len(&self) -> usize {
        &self.body.len() - 2 + 1
    }

    pub fn overlap_tail(&self, x: i32, y: i32) -> bool {
        let mut ch = 0;
        for block in &self.body {
            if x == block.x && y == block.y {
                return true;
            }
            ch += 1;
            if ch == self.body.len() - 1 {
                break;
            }
        }
        return false;
    }

    pub fn next_head(&self, dir: Option<Direction>) -> (i32, i32) {
        let (head_x, head_y): (i32, i32) = self.head_position();

        let mut moving_dir = self.direction;
        match dir {
            Some(d) => moving_dir = d,
            None => {}
        }

        match moving_dir {
            Direction::Up => (head_x, head_y - 1),
            Direction::Down => (head_x, head_y + 1),
            Direction::Left => (head_x - 1, head_y),
            Direction::Right => (head_x + 1, head_y),
        }
    }
}
