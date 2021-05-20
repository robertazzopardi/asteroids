use sdl2::{gfx::primitives::DrawRenderer, pixels::Color, render::Canvas, video::Window};

use crate::{math::Pos, window::Draw};

struct Asteroid {
    pos: Pos,
}

impl Asteroid {
    fn new(pos: Pos) -> Self {
        Self { pos }
    }
}

impl Draw for Asteroid {
    fn draw(&self, canvas: &Canvas<Window>) {
        let vec_x = &[self.pos.x as i16];
        let vec_y = &[self.pos.y as i16];

        let _ = canvas.filled_polygon(vec_x, vec_y, Color::BLACK);
        let _ = canvas.aa_polygon(vec_x, vec_y, Color::WHITE);
    }
}
