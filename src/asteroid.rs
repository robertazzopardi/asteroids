use crate::{
    math::{convert_to_xy_vec, get_center, rand_angle, UpdateVerts, Vec2},
    window::{get_edge_pos, MID_SIZE},
};
use rand::Rng;
use sdl2::{gfx::primitives::DrawRenderer, pixels::Color, render::Canvas, video::Window};
use std::mem;

const ASTEROID_COUNT: u16 = 3;
pub const ASTEROID_VERTS: usize = 20;

#[derive(Clone)]
pub struct Asteroid {
    pub verts: Vec<Vec2>,
    pub ghost_verts: Vec<Vec2>,
    vel: Vec2,
    angle: f32,
    divided: bool,
}

impl UpdateVerts for Asteroid {
    fn get_verts(&mut self) -> &mut Vec<Vec2> {
        &mut self.verts
    }

    fn get_ghost_verts(&mut self) -> &mut Vec<Vec2> {
        &mut self.ghost_verts
    }

    fn swap(&mut self) {
        mem::swap(&mut self.verts, &mut self.ghost_verts);

        for i in 0..self.verts.len() {
            self.ghost_verts[i] = self.verts[i].clone();
        }
    }
}

impl Asteroid {
    pub fn new_vec() -> Vec<Asteroid> {
        let mut asteroids: Vec<Asteroid> = Vec::new();
        for _ in 0..ASTEROID_COUNT {
            let center = get_edge_pos();
            asteroids.push(Asteroid::new(40, 100, center.0, center.1));
        }

        asteroids
    }

    pub fn update(&mut self) {
        for i in 0..self.verts.len() {
            self.verts[i].x += self.vel.x * self.angle.cos();
            self.verts[i].y += self.vel.y * self.angle.sin();

            self.ghost_verts[i].x += self.vel.x * self.angle.cos();
            self.ghost_verts[i].y += self.vel.y * self.angle.sin();
        }
    }

    pub fn new(min: u16, max: u16, center_x: f32, center_y: f32) -> Asteroid {
        let mut verts = Vec::new();

        for i in 0..ASTEROID_VERTS {
            let radius = rand::thread_rng().gen_range(min..max) as f32;

            let angle = (i as f32 / ASTEROID_VERTS as f32) * 6.28318;

            verts.push(Vec2::new(
                radius * angle.sin() + center_x,
                radius * angle.cos() + center_y,
            ));
        }

        let center = get_center(&verts);
        let angle_to_center = (MID_SIZE - center.y).atan2(MID_SIZE - center.x);

        Asteroid {
            verts: verts.clone(),
            ghost_verts: verts,
            vel: Vec2::new(
                rand::thread_rng().gen_range(0.5..1.5),
                rand::thread_rng().gen_range(0.5..1.5),
            ),
            angle: angle_to_center,
            divided: false,
        }
    }

    fn draw(&mut self, canvas: &Canvas<Window>) {
        let dxy = convert_to_xy_vec(&self.verts);
        let _ = canvas.filled_polygon(&dxy.0, &dxy.1, Color::BLACK);
        let _ = canvas.aa_polygon(&dxy.0, &dxy.1, Color::WHITE);

        let dxy = convert_to_xy_vec(&self.ghost_verts);
        let _ = canvas.filled_polygon(&dxy.0, &dxy.1, Color::BLACK);
        let _ = canvas.aa_polygon(&dxy.0, &dxy.1, Color::WHITE);
    }
}

pub fn divide_remove(asteroids: &mut Vec<Asteroid>, index: usize) {
    let asteroid = asteroids.remove(index);

    // if all big asteroids have been destroyed
    // if asteroids.iter().all(|f| f.divided) {
    //     let center = get_edge_pos();
    //     let new_big = Asteroid::new(40, 100, center.0, center.1);
    //     asteroids.push(new_big.clone());
    // }

    if !asteroid.divided {
        let center = get_center(&asteroid.verts);
        for _ in 0..rand::thread_rng().gen_range(2..4) {
            let new_asteroid = &mut Asteroid::new(20, 50, center.x, center.y);
            new_asteroid.divided = true;
            new_asteroid.angle = rand_angle();

            asteroids.push(new_asteroid.clone());
        }
    }

    if asteroids.len() == 0 {
        for _ in 0..2 {
            let center = get_edge_pos();
            let new_asteroid = Asteroid::new(40, 100, center.0, center.1);
            asteroids.push(new_asteroid.clone());
        }
    }
}

pub fn draw_asteroids(asteroids: &mut Vec<Asteroid>, canvas: &Canvas<Window>) {
    for asteroid in asteroids.iter_mut() {
        asteroid.draw(canvas);
    }
}
