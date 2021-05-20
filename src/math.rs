use rand::Rng;
use sdl2::{gfx::primitives::DrawRenderer, pixels::Color, render::Canvas, video::Window};

use crate::window::Draw;

pub struct Pos {
    pub x: f32,
    pub y: f32,
}

impl Pos {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

pub struct Triangle {
    pub p1: Pos,
    pub p2: Pos,
    pub p3: Pos,
}

// impl Translate for Triangle {
//     fn translate(&mut self, angle: f32) {
//         self.p1.x += 1. * angle.cos();
//         self.p1.y += 1. * angle.sin();

//         self.p2.x += 1. * angle.cos();
//         self.p2.y += 1. * angle.sin();

//         self.p3.x += 1. * angle.cos();
//         self.p3.y += 1. * angle.sin();
//     }
// }

pub fn translate(points: &mut [&mut Pos], angle: f32) {
    for point in points.iter_mut() {
        point.x += 1. * angle.cos();
        point.y += 1. * angle.sin();
    }
}

impl Draw for Triangle {
    fn draw(&self, canvas: &Canvas<Window>) {
        let _ = canvas.filled_trigon(
            self.p1.x as i16,
            self.p1.y as i16,
            self.p2.x as i16,
            self.p2.y as i16,
            self.p3.x as i16,
            self.p3.y as i16,
            Color::WHITE,
        );
    }
}

impl Triangle {
    pub fn new(p1: Pos, p2: Pos, p3: Pos) -> Self {
        Self { p1, p2, p3 }
    }

    pub fn get_center(&self) -> Pos {
        let x = (self.p1.x + self.p2.x + self.p3.x) / 3.;
        let y = (self.p1.y + self.p2.y + self.p3.y) / 3.;
        return Pos::new(x, y);
    }
}

pub fn rotate(points: &mut [&mut Pos], origin: Pos, angle: f32) {
    let cos = angle.cos();
    let sin = angle.sin();

    for pos in points.iter_mut() {
        let px = pos.x as f32 - origin.x;
        let py = pos.y as f32 - origin.y;

        let tmp_x = origin.x + cos * px - sin * py;
        pos.y = origin.y + sin * px + cos * py;
        pos.x = tmp_x;
    }
}

pub fn random_pos(min: f32, max: f32) -> f32 {
    return rand::thread_rng().gen_range(min..max);
}
