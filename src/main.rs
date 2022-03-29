extern crate find_folder;
extern crate piston;
extern crate piston_window;
extern crate rand;

mod color;
mod draw;
mod game;
mod physics;
mod snake;
use crate::game::Game;
use color::BACKGROUND;
use draw::blocks_in_pixels;
use find_folder::*;
use piston_window::types::Color;
use piston_window::*;

const WINDOW_TITLE: &'static str = "Snake Xenzia";
const WIDTH: u32 = 25;
const HEIGHT: u32 = 25;
fn main() {
    let size = [blocks_in_pixels(WIDTH), blocks_in_pixels(HEIGHT)];

    let mut window: PistonWindow = WindowSettings::new(WINDOW_TITLE, size)
        .resizable(true)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let assets = Search::ParentsThenKids(3, 3).for_folder("assets").unwrap();

    let ref font = assets.join("retro-gaming.ttf");
    let mut glyphs = window.load_font(font).unwrap();

    let mut game: Game = Game::new(WIDTH, HEIGHT);
    game.start();

    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.key_pressed(key);
            game.key_controls(key);
        }

        window.draw_2d(&event, |ctx, graphics, device| {
            clear(BACKGROUND, graphics);
            game.draw(&ctx, graphics, &mut glyphs);
            glyphs.factory.encoder.flush(device);
        });

        event.update(|arg| {
            game.update(arg.dt);
        });
    }
}
