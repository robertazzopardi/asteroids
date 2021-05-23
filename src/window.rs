use crate::{
    asteroid::{divide_remove, draw_asteroids, Asteroid},
    math::{polygon_collision, wrap_verts},
    ship::Ship,
    star::{draw_stars, Star},
};
use rand::Rng;
use sdl2::{
    event::Event,
    keyboard::Keycode,
    pixels::Color,
    render::Canvas,
    sys::{SDL_GetPerformanceCounter, SDL_GetPerformanceFrequency},
    video::Window,
    EventPump,
};
use std::time::Duration;

pub const SIZE: f32 = 800.;
pub const MID_SIZE: f32 = SIZE / 2.;

pub struct Win {
    canvas: Canvas<Window>,
    event_pump: EventPump,
}

impl Win {
    pub fn new() -> Win {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window("Asteroids", SIZE as u32, SIZE as u32)
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();

        canvas.set_draw_color(Color::BLACK);
        canvas.clear();
        canvas.present();

        let event_pump = sdl_context.event_pump().unwrap();

        Win { canvas, event_pump }
    }

    pub fn reset(&mut self) {
        let mut now;
        let mut dt;

        unsafe {
            now = SDL_GetPerformanceCounter();
        }

        // Create Entities
        let stars = &mut Star::new_vec();
        let ship = &mut Ship::new();
        let asteroids = &mut Asteroid::new_vec();

        'running: loop {
            let last = now;
            unsafe {
                now = SDL_GetPerformanceCounter();
                dt = (((now - last) * 1000) / SDL_GetPerformanceFrequency()) as f32 * 0.001;
            }

            self.canvas.clear();

            // check events
            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,
                    _ => {
                        ship.do_action(&event, &dt);
                    }
                }
            }

            // The rest of the game loop goes here...

            // update entities
            ship.update(dt);

            for i in 0..asteroids.len() {
                asteroids[i].update();
            }

            // spawn random asteroid with 1% chance
            // if rand::thread_rng().gen::<f32>() < 0.01 {
            //     let center = get_edge_pos();
            //     let new_asteroid = Asteroid::new(40, 100, center.0, center.1);
            //     asteroids.push(new_asteroid.clone());
            //     ghost_asteroids.push(new_asteroid);
            // }

            // check collisions
            let mut to_remove: Vec<usize> = Vec::new();
            for i in 0..ship.lasers.len() {
                let index = asteroids
                    .iter()
                    .position(|f| polygon_collision(&f.verts, &ship.lasers[i].pos));
                if index != None {
                    divide_remove(asteroids, index.unwrap());
                    to_remove.push(i);
                }
            }

            for i in to_remove {
                ship.lasers.remove(i);
            }

            for asteroid in asteroids.iter() {
                if ship.verts.iter().zip(&ship.ghost_verts).any(|f| {
                    polygon_collision(&asteroid.verts, f.0)
                        || polygon_collision(&asteroid.verts, f.1)
                }) {
                    break 'running;
                }
            }

            // check wrapping
            wrap_verts(ship);

            for i in 0..asteroids.len() {
                wrap_verts(&mut asteroids[i]);
            }

            // draw entities
            draw_stars(stars, &self.canvas);
            ship.draw(&self.canvas);
            draw_asteroids(asteroids, &self.canvas);

            self.canvas.set_draw_color(Color::BLACK);

            self.canvas.present();
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }
}

pub fn get_edge_pos() -> (f32, f32) {
    let mut width = rand::thread_rng().gen_range(0..SIZE as u16) as f32;
    let mut height = rand::thread_rng().gen_range(0..SIZE as u16) as f32;
    if rand::thread_rng().gen_bool(0.5) {
        if rand::thread_rng().gen_bool(0.5) {
            width = 0.;
        } else {
            width = SIZE;
        }
    } else {
        if rand::thread_rng().gen_bool(0.5) {
            height = 0.;
        } else {
            height = SIZE;
        }
    }

    // println!("{} {}", width, height);

    (width, height)
}
