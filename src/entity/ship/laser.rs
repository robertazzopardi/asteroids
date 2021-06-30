use crate::math::vec2::Vec2;
use sdl2::{gfx::primitives::DrawRenderer, pixels::Color, render::Canvas, video::Window};

/// The Ships Lasers
pub struct Laser {
    pub pos: Vec2,
    vel: Vec2,
    angle: f32,
    pub ddelta: f32,
}

/// Laser implementation
impl Laser {
    pub fn new(pos: Vec2, angle: f32) -> Self {
        Self {
            pos: Vec2::new(pos.x, pos.y),
            vel: Vec2::new(400., 400.),
            angle,
            ddelta: 0.,
        }
    }

    pub fn draw(&mut self, canvas: &Canvas<Window>) {
        let _ = canvas.filled_circle(self.pos.x as i16, self.pos.y as i16, 4, Color::WHITE);
    }

    pub fn update(&mut self, dt: f32) {
        self.ddelta += self.vel.magnitude() * dt;

        self.pos.x += self.vel.x * dt * self.angle.cos();
        self.pos.y += self.vel.y * dt * self.angle.sin();
    }

    pub fn get_pos(&self) -> &Vec2 {
        &self.pos
    }
}
