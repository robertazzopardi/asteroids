use sdl2::{
    event::Event, keyboard::Keycode, pixels::Color, render::Canvas, video::Window, EventPump, Sdl,
};
use std::time::Duration;

use crate::{
    ship::Ship,
    star::{draw_stars, Star},
};

pub const WIDTH: f32 = 800.;
pub const HEIGHT: f32 = 800.;
pub const MID_WIDTH: f32 = WIDTH / 2.;
pub const MID_HEIGHT: f32 = HEIGHT / 2.;

pub trait Draw {
    fn draw(&self, canvas: &Canvas<Window>);
}

// pub trait Translate {
//     fn translate(&mut self, angle: f32);
// }

pub struct Win {
    sdl_context: Sdl,
    canvas: Canvas<Window>,
    event_pump: EventPump,
}

impl Win {
    pub fn new() -> Win {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window("Asteroids", 800, 800)
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();

        canvas.set_draw_color(Color::BLACK);
        canvas.clear();
        canvas.present();

        let mut event_pump = sdl_context.event_pump().unwrap();

        return Win {
            sdl_context,
            canvas,
            event_pump,
        };
    }

    pub fn main_loop(&mut self, mut stars: Vec<Star>, mut ship: Ship) {
        'running: loop {
            self.canvas.clear();

            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,
                    _ => ship.do_action(event),
                }
            }

            // The rest of the game loop goes here...

            // let _m = canvas.hline(0, 800, 400, Color::WHITE);
            // let _l = canvas.vline(400, 0, 800, Color::WHITE);
            draw_stars(&mut stars, &self.canvas);
            ship.draw(&self.canvas);

            self.canvas.set_draw_color(Color::BLACK);

            self.canvas.present();
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }
}
