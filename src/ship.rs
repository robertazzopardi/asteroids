use crate::{math::Vec2, window::MID_SIZE};
use crate::{
    math::{convert_to_xy_vec, get_center, rotate, wrap_point, UpdateVerts},
    window::SIZE,
};
use sdl2::{
    event::Event, gfx::primitives::DrawRenderer, keyboard::Keycode, pixels::Color, render::Canvas,
    video::Window,
};
use std::mem;

pub const SHIP_SCALE: f32 = 7.;

pub struct Ship {
    verts: Vec<Vec2>,
    ghost_verts: Vec<Vec2>,
    vel: Vec2,
    accel: f32,
    angle: f32,
    lasers: Vec<Laser>,
}

pub struct Laser {
    pos: Vec2,
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

    pub fn get_pos(&self) -> &Vec2 {
        &self.pos
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
    pub fn get_lasers(&mut self) -> &mut Vec<Laser> {
        &mut self.lasers
    }

    pub fn get_verts(&self) -> &Vec<Vec2> {
        &self.verts
    }

    pub fn get_ghost_verts(&self) -> &Vec<Vec2> {
        &self.ghost_verts
    }

    pub fn remove_laser(&mut self, index: usize) {
        self.lasers.remove(index);
    }

    pub fn do_action(&mut self, event: &Event, dt: &f32) {
        match event {
            Event::KeyDown {
                keycode: Some(Keycode::Right),
                ..
            } => {
                rotate(&mut self.verts, 10. * dt);
                rotate(&mut self.ghost_verts, 10. * dt);
            }
            Event::KeyDown {
                keycode: Some(Keycode::Left),
                ..
            } => {
                rotate(&mut self.verts, -10. * dt);
                rotate(&mut self.ghost_verts, -10. * dt);
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

        let dv = self.accel * dt * 50.;
        self.vel.x += dv;
        self.vel.y += dv;

        let vel_x = self.vel.x * dt * self.angle.cos();
        let vel_y = self.vel.y * dt * self.angle.sin();

        for i in 0..self.verts.len() {
            self.verts[i].x += vel_x;
            self.verts[i].y += vel_y;

            self.ghost_verts[i].x += vel_x;
            self.ghost_verts[i].y += vel_y;
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

        // if self.verts != self.ghost_verts {
        if !self
            .verts
            .iter()
            .all(|f| f.x < SIZE && f.x > 0. && f.y < SIZE && f.y > 0.)
        {
            let dxy = convert_to_xy_vec(&self.ghost_verts);
            let _ = canvas.filled_polygon(&dxy.0, &dxy.1, Color::WHITE);
        }

        for laser in self.lasers.iter_mut() {
            wrap_point(&mut laser.pos);
            laser.draw(canvas);
        }
    }
}
