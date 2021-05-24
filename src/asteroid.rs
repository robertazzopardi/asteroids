use crate::{
    math::{convert_to_xy_vec, get_center, rand_angle, UpdateVerts, Vec2},
    window::{get_edge_pos, MID_SIZE, SIZE},
};
use rand::Rng;
use sdl2::{gfx::primitives::DrawRenderer, pixels::Color, render::Canvas, video::Window};
use std::mem;

const ASTEROID_COUNT: u16 = 3;
pub const ASTEROID_VERTS: usize = 20;

#[derive(Clone)]
pub struct Asteroid {
    verts: Vec<Vec2>,
    ghost_verts: Vec<Vec2>,
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
    }
}

impl Asteroid {
    // pub fn get_verts(&self) -> &Vec<Vec2> {
    //     &self.verts
    // }

    // pub fn get_ghost_verts(&self) -> &Vec<Vec2> {
    //     &self.ghost_verts
    // }

    pub fn new_vec() -> Vec<Asteroid> {
        let mut asteroids: Vec<Asteroid> = Vec::new();
        for _ in 0..ASTEROID_COUNT {
            let center = get_edge_pos();
            asteroids.push(Asteroid::new(40, 100, center.0, center.1));
        }

        asteroids
    }

    pub fn update(&mut self) {
        let vel_x = self.vel.x * self.angle.cos();
        let vel_y = self.vel.y * self.angle.sin();

        for i in 0..self.verts.len() {
            self.verts[i].x += vel_x;
            self.verts[i].y += vel_y;

            self.ghost_verts[i].x += vel_x;
            self.ghost_verts[i].y += vel_y;
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
                rand::thread_rng().gen_range(0.7..1.7),
                rand::thread_rng().gen_range(0.7..1.7),
            ),
            angle: angle_to_center,
            divided: false,
        }
    }

    pub fn draw(&mut self, canvas: &Canvas<Window>) {
        let dxy = convert_to_xy_vec(&self.verts);
        let _ = canvas.filled_polygon(&dxy.0, &dxy.1, Color::BLACK);
        let _ = canvas.aa_polygon(&dxy.0, &dxy.1, Color::WHITE);

        if !self
            .verts
            .iter()
            .all(|f| f.x < SIZE && f.x > 0. && f.y < SIZE && f.y > 0.)
        {
            let dxy = convert_to_xy_vec(&self.ghost_verts);
            let _ = canvas.filled_polygon(&dxy.0, &dxy.1, Color::BLACK);
            let _ = canvas.aa_polygon(&dxy.0, &dxy.1, Color::WHITE);
        }
    }

    pub fn collision(&mut self, point: &Vec2) -> bool {
        let mut collision = false;
        let mut j = self.verts.len() - 1;

        for i in 0..self.verts.len() {
            if trace(&self.verts, i, point, j) || trace(&self.ghost_verts, i, point, j) {
                collision = !collision;
            }
            j = i;
        }

        collision
    }
}

#[inline]
fn trace(verts: &[Vec2], i: usize, point: &Vec2, j: usize) -> bool {
    ((verts[i].y > point.y) != (verts[j].y > point.y))
        && (point.x
            < (verts[j].x - verts[i].x) * (point.y - verts[i].y) / (verts[j].y - verts[i].y)
                + verts[i].x)
}

pub fn divide_remove(asteroids: &mut Vec<Asteroid>, index: usize) {
    let asteroid = asteroids.remove(index);

    if !asteroid.divided {
        let center = get_center(&asteroid.verts);
        for _ in 0..rand::thread_rng().gen_range(2..4) {
            let new_asteroid = &mut Asteroid::new(20, 50, center.x, center.y);
            new_asteroid.divided = true;
            new_asteroid.angle = rand_angle();

            asteroids.push(new_asteroid.clone());
        }
    }

    if asteroids.is_empty() {
        for _ in 0..2 {
            let center = get_edge_pos();
            asteroids.push(Asteroid::new(40, 100, center.0, center.1));
        }
    }
}
