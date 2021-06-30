use crate::{
    math::{
        functions::{get_random_radius, rand_angle},
        vec2::{UpdateVerts, Vec2, Vec2Vec},
    },
    render::window::{MID_SIZE, SIZE},
};
use rand::Rng;
use sdl2::{gfx::primitives::DrawRenderer, pixels::Color, render::Canvas, video::Window};
use std::mem;

pub const ASTEROID_VERTS: usize = 20;
const ASTEROID_COUNT: u16 = 3;
const SPEED_MIN: f32 = 1.;
static mut SPEED_MAX: f32 = 1.7;

#[derive(Clone)]
pub struct Asteroid {
    verts: Vec<Vec2>,
    ghost_verts: Vec<Vec2>,
    vel: Vec2,
    angle: f32,
    divided: bool,
}

pub trait RemoveAsteroid<Asteroid> {
    fn break_up(&mut self, index: usize);
}

impl RemoveAsteroid<Asteroid> for Vec<Asteroid> {
    fn break_up(&mut self, index: usize) {
        divide_remove(self, index);
    }
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
    pub fn new_vec() -> Vec<Asteroid> {
        let mut asteroids: Vec<Asteroid> = Vec::new();
        for _ in 0..ASTEROID_COUNT {
            let (x, y) = get_random_radius();
            asteroids.push(Asteroid::new(40, 100, x, y));
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

    pub fn new(min_r: u16, max_r: u16, center_x: f32, center_y: f32) -> Asteroid {
        let mut verts = Vec::new();

        for i in 0..ASTEROID_VERTS {
            let radius = rand::thread_rng().gen_range(min_r..max_r) as f32;

            let angle = (i as f32 / ASTEROID_VERTS as f32) * 6.28318;

            verts.push(Vec2::new(
                radius * angle.sin() + center_x,
                radius * angle.cos() + center_y,
            ));
        }

        let center = verts.get_center();
        let angle_to_center = (MID_SIZE - center.y).atan2(MID_SIZE - center.x);

        let vel_x = unsafe { SPEED_MIN..SPEED_MAX };
        let vel_y = unsafe { SPEED_MIN..SPEED_MAX };

        Asteroid {
            verts: verts.clone(),
            ghost_verts: verts,
            vel: Vec2::new(
                rand::thread_rng().gen_range(vel_x),
                rand::thread_rng().gen_range(vel_y),
            ),
            angle: angle_to_center,
            divided: false,
        }
    }

    /// Draw the asteroid
    pub fn draw(&mut self, canvas: &Canvas<Window>) {
        // Main verts
        let (x, y) = self.verts.convert_to_xy_vec();
        let _ = canvas.filled_polygon(&x, &y, Color::BLACK);
        let _ = canvas.aa_polygon(&x, &y, Color::WHITE);

        // Draw ghost verts if they are on the screen
        if !self
            .verts
            .iter()
            .all(|f| f.x < SIZE && f.x > 0. && f.y < SIZE && f.y > 0.)
        {
            let (x, y) = self.ghost_verts.convert_to_xy_vec();
            let _ = canvas.filled_polygon(&x, &y, Color::BLACK);
            let _ = canvas.aa_polygon(&x, &y, Color::WHITE);
        }
    }

    pub fn collision(&mut self, point: &Vec2) -> bool {
        collision(self.get_verts(), point) || collision(self.get_ghost_verts(), point)
    }
}

fn collision(verts: &mut Vec<Vec2>, point: &Vec2) -> bool {
    let mut collision = false;
    let mut j = verts.len() - 1;

    for i in 0..verts.len() {
        // if trace(verts, i, point, j) {
        if ((verts[i].y > point.y) != (verts[j].y > point.y))
            && (point.x
                < (verts[j].x - verts[i].x) * (point.y - verts[i].y) / (verts[j].y - verts[i].y)
                    + verts[i].x)
        {
            collision = !collision;
        }
        j = i;
    }

    collision
}

fn divide_remove(asteroids: &mut Vec<Asteroid>, index: usize) {
    unsafe { SPEED_MAX += 0.1 }

    let asteroid = asteroids.remove(index);

    if !asteroid.divided {
        let Vec2 { x, y } = asteroid.verts.get_center();
        for _ in 0..rand::thread_rng().gen_range(2..4) {
            let new_asteroid = &mut Asteroid::new(20, 50, x, y);
            new_asteroid.divided = true;
            new_asteroid.angle = rand_angle();

            asteroids.push(new_asteroid.clone());
        }
    }

    if asteroids.is_empty() {
        for _ in 0..2 {
            let (x, y) = get_random_radius();
            asteroids.push(Asteroid::new(40, 100, x, y));
        }
    }
}
