use crate::math::{rotate, translate, Triangle};
use crate::{math::Pos, window::Draw, window::MID_HEIGHT, window::MID_WIDTH};
use sdl2::{event::Event, keyboard::Keycode, render::Canvas, video::Window};

const TURN_AMOUNT: f32 = 0.0174533 * 20.;

const SHIP_SCALE: f32 = 15.;

pub struct Ship {
    tri: Triangle,
}

struct Laser {}

impl Draw for Ship {
    fn draw(&self, canvas: &Canvas<Window>) {
        // let _ = canvas.filled_circle(self.tri.p2.x as i16, self.tri.p2.y as i16, 3, Color::RED);
        self.tri.draw(canvas);
    }
}

impl Ship {
    pub fn do_action(&mut self, event: Event) {
        match event {
            Event::KeyDown {
                keycode: Some(Keycode::Right),
                ..
            } => {
                let origin = self.tri.get_center();
                rotate(
                    &mut [&mut self.tri.p1, &mut self.tri.p2, &mut self.tri.p3],
                    origin,
                    TURN_AMOUNT,
                );
            }
            Event::KeyDown {
                keycode: Some(Keycode::Left),
                ..
            } => {
                let origin = self.tri.get_center();
                rotate(
                    &mut [&mut self.tri.p1, &mut self.tri.p2, &mut self.tri.p3],
                    origin,
                    -TURN_AMOUNT,
                );
            }
            Event::KeyDown {
                keycode: Some(Keycode::Up),
                ..
            } => {
                let cent = self.tri.get_center();
                println!("{} {}", cent.x, cent.y);

                let a = (self.tri.p3.y - cent.y).atan2(self.tri.p3.x - cent.x);

                translate(
                    &mut [&mut self.tri.p1, &mut self.tri.p2, &mut self.tri.p3],
                    a,
                );
            }
            Event::KeyDown {
                keycode: Some(Keycode::Space),
                ..
            } => {}
            _ => (),
        }
    }

    pub fn new() -> Ship {
        return Ship {
            tri: Triangle::new(
                Pos::new(
                    MID_WIDTH - 0.866 * SHIP_SCALE,
                    MID_HEIGHT + 0.5 * SHIP_SCALE,
                ),
                Pos::new(
                    MID_WIDTH + 0.866 * SHIP_SCALE,
                    MID_HEIGHT + 0.5 * SHIP_SCALE,
                ),
                Pos::new(MID_WIDTH, MID_HEIGHT - 1. * SHIP_SCALE),
            ),
        };
    }
}
