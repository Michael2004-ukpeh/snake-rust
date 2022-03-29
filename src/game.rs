
use piston_window::*;
use rand::{thread_rng, Rng};

use crate::color::*;
use crate::draw::*;
use crate::physics::{Direction, Position};
use crate::snake::Snake;

const MOVING_PERIOD: f64 = 0.1;
const RESTART_TIME: f64 = 1.0;

const FPS: f64 = 10.0;

fn fps_in_ms(fps: f64) -> f64 {
    1.0 / fps
}

fn calc_random_pos(width: u32, height: u32) -> Position {
    let mut rng = rand::thread_rng();

    Position {
        x: rng.gen_range(0..width as i32),
        y: rng.gen_range(0..height as i32),
    }
}
pub struct Game {
    snake: Snake,
    food: Position,
    food_exist: bool,
    size: (u32, u32),
    waiting_time: f64,
    score: u32,
    game_over: bool,
    paused: bool,
}

impl Game {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            snake: Snake::new(calc_random_pos(width, height)),
            food: calc_random_pos(width, height),
            food_exist: true,
            size: (width, height),
            waiting_time: 0.0,
            score: 0,
            game_over: false,
            paused: true,
        }
    }

    pub fn start(&mut self) {
        self.paused = false;
    }

    pub fn pause(&mut self) {
        self.paused = true;
    }

    pub fn toggle_game_state(&mut self) {
        if self.paused {
            self.start();
        } else {
            self.pause();
        }
    }
    pub fn key_controls(&mut self, key: Key) {
        match key {
            Key::R => self.restart(),
            Key::P | Key::Space => self.toggle_game_state(),
            _ => {}
        }
    }

    pub fn draw(&self, ctx: &Context, graphics: &mut G2d, glyphs: &mut Glyphs) {
        draw_block(FRUIT, &self.food, ctx, graphics);
        self.snake.draw(&ctx, graphics);
        draw_text(
            &ctx,
            graphics,
            glyphs,
            SCORE_TEXT,
            Position { x: 0, y: 20 },
            &self.get_score().to_string(),
        );
        if self.game_over {
            draw_overlay(&ctx, graphics, OVERLAY, self.size)
        }
    }

    pub fn get_score(&self) -> u32 {
        self.score
    }
    pub fn calc_score(&mut self) {
        self.score = (self.snake.get_len() * 10) as u32;
    }

    fn restart(&mut self) {
        *self = Game::new(self.size.0, self.size.1);
        self.start();
    }
    pub fn key_pressed(&mut self, key: Key) {
        if self.game_over {
            return;
        }

        let dir = match key {
            Key::Up => Some(Direction::Up),
            Key::Down => Some(Direction::Down),
            Key::Left => Some(Direction::Left),
            Key::Right => Some(Direction::Right),
            _ => Some(self.snake.head_direction()),
        };

        if let Some(dir) = dir {
            if dir == self.snake.head_direction().opposite() {
                return;
            }
        }
        self.update_snake(dir);
    }

    fn check_if_snake_alive(&self, dir: Option<Direction>) -> bool {
        let (next_x, next_y) = self.snake.next_head(dir);
        if self.snake.overlap_tail(next_x, next_y) {
            return false;
        }

        next_x > 0
            && next_y > 0
            && next_x < (self.size.0 - 1) as i32
            && next_y < (self.size.1 - 1) as i32
    }

    pub fn check_eating(&mut self) {
        let (head_x, head_y): (i32, i32) = self.snake.head_position();
        if self.food_exist && self.food.x == head_x && self.food.y == head_y {
            self.food_exist = false;
            self.snake.restore_tail();
        }
    }

    pub fn update_snake(&mut self, dir: Option<Direction>) {
        if self.check_if_snake_alive(dir) {
            self.snake.move_forward(dir);
            self.check_eating();
        } else {
            self.game_over = true;
        }
        self.waiting_time = 0.0;
    }
    pub fn update(&mut self, delta_time: f64) {
        self.waiting_time += delta_time;

        if self.game_over {
            if self.waiting_time > RESTART_TIME {
                self.restart();
            }
            return;
        }
        if !self.food_exist {
            self.food = calc_random_pos(self.size.0, self.size.1);
        }

        if self.waiting_time > MOVING_PERIOD {
            self.update_snake(None);
        }

        if self.snake.head_position() == (self.food.x, self.food.y) {
            self.food = calc_random_pos(self.size.0, self.size.1);
            self.calc_score();
        }
    }
}
