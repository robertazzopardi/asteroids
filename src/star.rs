use crate::math::random_pos;
use rand::Rng;
use sdl2::{gfx::primitives::DrawRenderer, pixels::Color, render::Canvas, video::Window};

use crate::{math::Pos, window::Draw, window::HEIGHT, window::WIDTH};

const STAR_COUNT: u16 = 200;

pub struct Star {
    pos: Pos,
    r: i16,
}

impl Draw for Star {
    fn draw(&self, canvas: &Canvas<Window>) {
        let _ = canvas.filled_circle(self.pos.x as i16, self.pos.y as i16, self.r, Color::WHITE);
    }
}

impl Star {
    pub fn new_vec() -> Vec<Star> {
        let mut stars: Vec<Star> = Vec::new();
        for _ in 0..STAR_COUNT {
            stars.push(Star {
                pos: Pos::new(random_pos(0., WIDTH), random_pos(0., HEIGHT)),
                r: random_pos(1., 3.) as i16,
            });
        }

        return stars;
    }
}

pub fn draw_stars(stars: &mut Vec<Star>, canvas: &Canvas<Window>) {
    for star in stars.iter_mut() {
        if rand::thread_rng().gen::<f32>() < 0.01 {
            star.r = random_pos(1., 3.) as i16;
        }
        star.draw(canvas);
    }
}
