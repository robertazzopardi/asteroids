use crate::window::GetKey;
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
}

/// The Ships Lasers
pub struct Laser {
    pos: Vec2,
    vel: Vec2,
    angle: f32,
    ddelta: f32,
}

/// Laser implementation
impl Laser {
    fn new(pos: Vec2, angle: f32) -> Self {
        Self {
            pos: Vec2::new(pos.x, pos.y),
            vel: Vec2::new(400., 400.),
            angle,
            ddelta: 0.,
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
    pub fn get_lasers(&mut self) -> &mut Vec<Laser> {
        &mut self.lasers
    }

    pub fn remove_laser(&mut self, index: usize) {
        self.lasers.remove(index);
    }

    pub fn do_action(&mut self, event: &Event) {
        let code = event.get_keycode();
        match code {
            (Keycode::Right | Keycode::Left, Event::KeyDown { .. }) => {
                self.rot = if code.0 == Keycode::Right {
                    ROTATION_AMOUNT
                } else {
                    -ROTATION_AMOUNT
                }
            }
            (Keycode::Up, Event::KeyDown { .. }) => {
                let cent = get_center(&self.verts);
                self.angle = (self.verts[2].y - cent.y).atan2(self.verts[2].x - cent.x);

                if self.vel.magnitude() < MAX_VELOCITY {
                    self.accel += 10.;
                } else {
                    self.accel = 0.;
                }
            }
            (Keycode::Space, Event::KeyDown { .. }) => {
                let cent = get_center(&self.verts);
                self.angle = (self.verts[2].y - cent.y).atan2(self.verts[2].x - cent.x);
                self.lasers
                    .push(Laser::new(self.verts[2].clone(), self.angle));
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
        rotate(&mut self.verts, self.rot * dt);
        rotate(&mut self.ghost_verts, self.rot * dt);

        // Decay Speed (Even though in space there is no friction, its a bit easier this way)
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
        // Verts for an isosceles triangle
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
            rot: 0.,
        }
    }

    /// Draw the ships verts to the canvas
    /// If the ghost verts are in bounds draw them too
    /// Draw the lasers if ther are any
    pub fn draw(&mut self, canvas: &Canvas<Window>) {
        // Draw ship verts
        let dxy = convert_to_xy_vec(&self.verts);
        let _ = canvas.filled_polygon(&dxy.0, &dxy.1, Color::WHITE);

        // Draw ghost ship verts
        if !self
            .verts
            .iter()
            .all(|f| f.x < SIZE && f.x > 0. && f.y < SIZE && f.y > 0.)
        {
            let dxy = convert_to_xy_vec(&self.ghost_verts);
            let _ = canvas.filled_polygon(&dxy.0, &dxy.1, Color::WHITE);
        }

        // Draw lasers
        for laser in self.lasers.iter_mut() {
            wrap_point(&mut laser.pos);
            laser.draw(canvas);
        }
    }
}
