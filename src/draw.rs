use crate::color;
use crate::physics::{Direction, Position};
use piston_window::types::Color;
use piston_window::{rectangle, text, Context, G2d, Glyphs, Transformed};

pub const BLOCK_SIZE: f64 = 25.0;
pub fn to_coord(game_coord: i32) -> f64 {
    (game_coord as f64) * BLOCK_SIZE
}
pub fn to_coord_u32(game_coord: i32) -> u32 {
    to_coord(game_coord) as u32
}
pub fn draw_text(
    ctx: &Context,
    graphics: &mut G2d,
    glyphs: &mut Glyphs,
    color: Color,
    pos: Position,
    text: &str,
) {
    text::Text::new_color(color, 20)
        .draw(
            text,
            glyphs,
            &ctx.draw_state,
            ctx.transform.trans(pos.x as f64, pos.y as f64),
            graphics,
        )
        .unwrap();
}

pub fn draw_block(color: Color, pos: &Position, ctx: &Context, graphics: &mut G2d) {
    let (x, y) = (to_coord(pos.x), to_coord(pos.y));
    rectangle(
        color,
        [x, y, BLOCK_SIZE, BLOCK_SIZE],
        ctx.transform,
        graphics,
    )
}

pub fn blocks_in_pixels(n: u32) -> u32 {
    n * BLOCK_SIZE as u32
}

pub fn draw_snake_head(
    ctx: &Context,
    graphics: &mut G2d,
    color: Color,
    pos: &Position,
    dir: &Direction,
) {
    draw_block(color, pos, ctx, graphics);

    fn draw_eye(ctx: &Context, graphics: &mut G2d, x: f64, y: f64) {
        rectangle(color::BACKGROUND, [x, y, 5.0, 5.0], ctx.transform, graphics)
    }

    let (x, y) = (
        blocks_in_pixels(pos.x as u32) as f64,
        blocks_in_pixels(pos.y as u32) as f64,
    );

    let block = blocks_in_pixels(1) as f64;

    match dir {
        Direction::Up => {
            draw_eye(ctx, graphics, x + 5.0, y + 5.0);
            draw_eye(ctx, graphics, x + block - 10.0, y + 5.0);
        }
        Direction::Right => {
            draw_eye(ctx, graphics, x + block - 10.0, y + 5.0);
            draw_eye(ctx, graphics, x + block - 10.0, y + block - 10.0);
        }
        Direction::Down => {
            draw_eye(ctx, graphics, x + 5.0, y + block - 10.0);
            draw_eye(ctx, graphics, x + block - 10.0, y + block - 10.0);
        }
        Direction::Left => {
            draw_eye(ctx, graphics, x + 5.0, y + 5.0);
            draw_eye(ctx, graphics, x + 5.0, y + block - 10.0);
        }
    }
}

pub fn draw_overlay(ctx: &Context, graphics: &mut G2d, color: Color, size: (u32, u32)) {
    rectangle(
        color,
        [
            0.0,
            0.0,
            blocks_in_pixels(size.0) as f64,
            blocks_in_pixels(size.1) as f64,
        ],
        ctx.transform,
        graphics,
    );
}
