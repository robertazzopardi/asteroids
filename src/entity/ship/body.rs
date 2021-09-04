use crate::{
    entity::asteroid::Asteroid,
    math::vec2::{UpdateVerts, Vec2, Vec2Vec},
    render::window::{GetKey, MID_SIZE, SIZE},
};
use sdl2::{
    event::Event, gfx::primitives::DrawRenderer, keyboard::Keycode, pixels::Color, render::Canvas,
    video::Window,
};
use std::mem;

use super::laser::Laser;

pub const SHIP_SCALE: f32 = 7.;
const MAX_VELOCITY: f32 = 700.;
const ROTATION_AMOUNT: f32 = 4.;

/// The Players Ship
pub struct Ship {
    verts: Vec<Vec2>,
    ghost_verts: Vec<Vec2>,
    vel: Vec2,
    accel: f32,
    angle: f32,
    lasers: Vec<Laser>,
    rot: f32,
    firing: bool,
}

/// Implement UpdateVerts Trait for the Ship
impl UpdateVerts for Ship {
    fn get_verts(&mut self) -> &mut Vec<Vec2> {
        &mut self.verts
    }

    fn get_ghost_verts(&mut self) -> &mut Vec<Vec2> {
        &mut self.ghost_verts
    }

    /// Swap the Main verts for the ship with the ghost verts
    fn swap(&mut self) {
        mem::swap(&mut self.verts, &mut self.ghost_verts);
        mem::swap(&mut self.ghost_verts, &mut self.verts.clone());
    }
}

/// Implementation for Ship
impl Ship {
    pub fn get_lasers(&self) -> &Vec<Laser> {
        &self.lasers
    }

    pub fn remove_laser(&mut self, index: usize) {
        self.lasers.remove(index);
    }

    pub fn do_action(&mut self, event: &Event) {
        match event.get_keycode() {
            (Keycode::Right | Keycode::Left, Event::KeyDown { .. }) => {
                self.rot = if event.get_keycode().0 == Keycode::Right {
                    ROTATION_AMOUNT
                } else {
                    -ROTATION_AMOUNT
                }
            }
            (Keycode::Up, Event::KeyDown { .. }) => {
                let cent = self.verts.get_center();
                self.angle = (self.verts[2].y - cent.y).atan2(self.verts[2].x - cent.x);

                if self.vel.magnitude() < MAX_VELOCITY {
                    self.accel += 10.;
                } else {
                    self.accel = 0.;
                }
            }
            (Keycode::Space, Event::KeyDown { .. }) => {
                if !self.firing {
                    let cent = self.verts.get_center();
                    self.angle = (self.verts[2].y - cent.y).atan2(self.verts[2].x - cent.x);
                    self.lasers
                        .push(Laser::new(self.verts[2].clone(), self.angle));
                    self.firing = true;
                }
            }
            (Keycode::Space, Event::KeyUp { .. }) => {
                self.firing = false;
            }
            (Keycode::Up, Event::KeyUp { .. }) => {
                self.accel = 0.;
            }
            (Keycode::Right | Keycode::Left, Event::KeyUp { .. }) => {
                self.rot = 0.;
            }
            _ => (),
        }
    }

    pub fn update(&mut self, dt: f32) {
        // Update Ships Rotation
        self.verts.rotate(self.rot * dt);
        self.ghost_verts.rotate(self.rot * dt);

        // Decay Speed (Even though in space there is no friction)
        self.vel.x *= 0.98;
        self.vel.y *= 0.98;

        // Accelerate
        let dv = self.accel * dt * 50.;
        self.vel.x += dv;
        self.vel.y += dv;

        // Direction
        let vel_x = self.vel.x * dt * self.angle.cos();
        let vel_y = self.vel.y * dt * self.angle.sin();

        // Update verts
        for (vert, ghost) in self.verts.iter_mut().zip(&mut self.ghost_verts) {
            vert.x += vel_x;
            ghost.x += vel_x;
            vert.y += vel_y;
            ghost.y += vel_y;
        }

        // Update lasers
        for laser in self.lasers.iter_mut() {
            laser.update(dt);
        }

        self.lasers.retain(|f| f.ddelta < 1000.);
    }

    /// Creates new default ship instance
    pub fn new() -> Ship {
        let p1 = MID_SIZE + 2.5 * SHIP_SCALE;
        // Verts for an isosceles triangle
        let verts = vec![
            Vec2::new(MID_SIZE - 2.5 * SHIP_SCALE, p1),
            Vec2::new(p1, p1),
            Vec2::new(MID_SIZE, MID_SIZE - 5. * SHIP_SCALE),
        ];
        Ship {
            verts: verts.clone(),
            ghost_verts: verts,
            vel: Vec2::new(0., 0.),
            accel: 0.,
            angle: 0.,
            lasers: Vec::new(),
            rot: 0.,
            firing: false,
        }
    }

    /// Draw the ships verts to the canvas
    /// If the ghost verts are in bounds draw them too
    /// Draw the lasers if ther are any
    pub fn draw(&mut self, canvas: &Canvas<Window>) {
        // Draw ship verts
        let (x, y) = self.verts.convert_to_xy_vec();
        let _ = canvas.filled_polygon(&x, &y, Color::WHITE);

        // Draw ghost ship verts
        if !self
            .verts
            .iter()
            .all(|f| f.x < SIZE && f.x > 0. && f.y < SIZE && f.y > 0.)
        {
            let (x, y) = self.ghost_verts.convert_to_xy_vec();
            let _ = canvas.filled_polygon(&x, &y, Color::WHITE);
        }

        // Draw lasers
        for laser in self.lasers.iter_mut() {
            laser.pos.wrap_point();
            laser.draw(canvas);
        }
    }

    pub fn check_collision(&mut self, asteroid: &mut Asteroid) -> bool {
        self.get_verts()
            .iter()
            .any(|point| asteroid.collision(point))
            || self
                .get_ghost_verts()
                .iter()
                .any(|point| asteroid.collision(point))
    }
}
