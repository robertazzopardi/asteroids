use crate::{math::Vec2, window::SIZE};
use rand::Rng;
use sdl2::{gfx::primitives::DrawRenderer, pixels::Color, render::Canvas, video::Window};

const STAR_COUNT: usize = 200;

#[derive(Clone)]
pub struct Star {
    pos: Vec2,
    r: i16,
}

impl Star {
    pub fn new(pos: Vec2, r: i16) -> Self {
        Self { pos, r }
    }

    pub fn new_vec() -> Vec<Star> {
        let mut stars: Vec<Star> = Vec::new();
        for _ in 0..STAR_COUNT {
            stars.push(Star::new(
                Vec2::new(
                    rand::thread_rng().gen_range(0..SIZE as u16) as f32,
                    rand::thread_rng().gen_range(0..SIZE as u16) as f32,
                ),
                rand::thread_rng().gen_range(1..3) as i16,
            ));
        }

        stars
    }

    pub fn draw(&mut self, canvas: &Canvas<Window>) {
        let _ = canvas.filled_circle(self.pos.x as i16, self.pos.y as i16, self.r, Color::WHITE);
    }
}
