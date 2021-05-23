use crate::math::{convert_to_xy_vec, get_center, rotate, wrap_point, UpdateVerts};
use crate::{math::Vec2, window::MID_SIZE};
use sdl2::{
    event::Event, gfx::primitives::DrawRenderer, keyboard::Keycode, pixels::Color, render::Canvas,
    video::Window,
};
use std::mem;

pub const SHIP_SCALE: f32 = 7.;

pub struct Ship {
    pub verts: Vec<Vec2>,
    pub ghost_verts: Vec<Vec2>,
    vel: Vec2,
    accel: f32,
    angle: f32,
    pub lasers: Vec<Laser>,
}

pub struct Laser {
    pub pos: Vec2,
    vel: Vec2,
    angle: f32,
    ddelta: f32,
}

impl Laser {
    fn new(pos: Vec2, vel: Vec2, angle: f32, ddelta: f32) -> Self {
        Self {
            pos,
            vel,
            angle,
            ddelta,
        }
    }

    fn draw(&mut self, canvas: &Canvas<Window>) {
        let _ = canvas.filled_circle(self.pos.x as i16, self.pos.y as i16, 4, Color::WHITE);
    }

    fn update(&mut self, dt: f32) {
        self.ddelta += self.vel.magnitude() * dt;

        self.pos.x += self.vel.x * dt * self.angle.cos();
        self.pos.y += self.vel.y * dt * self.angle.sin();
    }
}

impl UpdateVerts for Ship {
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

impl Ship {
    pub fn do_action(&mut self, event: &Event, dt: &f32) {
        match event {
            Event::KeyDown {
                keycode: Some(Keycode::Right),
                ..
            } => {
                let origin = get_center(&self.verts);
                rotate(&mut self.verts, origin.clone(), 10. * dt);
                rotate(&mut self.ghost_verts, origin, 10. * dt);
            }
            Event::KeyDown {
                keycode: Some(Keycode::Left),
                ..
            } => {
                let origin = get_center(&self.verts);
                rotate(&mut self.verts, origin.clone(), -10. * dt);
                rotate(&mut self.ghost_verts, origin, -10. * dt);
            }
            Event::KeyDown {
                keycode: Some(Keycode::Up),
                ..
            } => {
                let cent = get_center(&self.verts);
                self.angle = (self.verts[2].y - cent.y).atan2(self.verts[2].x - cent.x);

                if self.accel < 20. {
                    self.accel += 10.;
                }
            }
            Event::KeyDown {
                keycode: Some(Keycode::Space),
                ..
            } => {
                let cent = get_center(&self.verts);
                self.angle = (self.verts[2].y - cent.y).atan2(self.verts[2].x - cent.x);
                self.lasers.push(Laser::new(
                    Vec2::new(self.verts[2].x, self.verts[2].y),
                    Vec2::new(400., 400.),
                    self.angle,
                    0.,
                ));
            }
            Event::KeyUp {
                keycode: Some(Keycode::Up),
                ..
            } => {
                self.accel = 0.;
            }
            _ => (),
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.vel.x *= 0.97;
        self.vel.y *= 0.97;

        self.vel.x += self.accel * dt * 50.;
        self.vel.y += self.accel * dt * 50.;

        for (vert, ghost) in self.verts.iter_mut().zip(self.ghost_verts.iter_mut()) {
            vert.x += self.vel.x * dt * self.angle.cos();
            vert.y += self.vel.y * dt * self.angle.sin();

            ghost.x += self.vel.x * dt * self.angle.cos();
            ghost.y += self.vel.y * dt * self.angle.sin();
        }

        // update lasers
        for laser in self.lasers.iter_mut() {
            laser.update(dt);
        }

        self.lasers.retain(|f| f.ddelta < 1000.);
    }

    pub fn new() -> Ship {
        let verts = vec![
            Vec2::new(MID_SIZE - 2.5 * SHIP_SCALE, MID_SIZE + 2.5 * SHIP_SCALE),
            Vec2::new(MID_SIZE + 2.5 * SHIP_SCALE, MID_SIZE + 2.5 * SHIP_SCALE),
            Vec2::new(MID_SIZE, MID_SIZE - 5. * SHIP_SCALE),
        ];
        Ship {
            verts: verts.clone(),
            ghost_verts: verts,
            vel: Vec2::new(0., 0.),
            accel: 0.,
            angle: 0.,
            lasers: Vec::new(),
        }
    }

    pub fn draw(&mut self, canvas: &Canvas<Window>) {
        let dxy = convert_to_xy_vec(&self.verts);
        let _ = canvas.filled_polygon(&dxy.0, &dxy.1, Color::WHITE);

        let dxy = convert_to_xy_vec(&self.ghost_verts);
        let _ = canvas.filled_polygon(&dxy.0, &dxy.1, Color::WHITE);

        for laser in self.lasers.iter_mut() {
            wrap_point(&mut laser.pos);
            laser.draw(canvas);
        }
    }
}