use crate::{
    asteroid::{collision, divide_remove, Asteroid},
    math::{wrap_verts, UpdateVerts},
    ship::Ship,
    star::Star,
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
const NANOS: u32 = 1_000_000_000u32 / 60;

pub trait GetKey {
    fn get_keycode(&self) -> (Keycode, &Event);
}

impl GetKey for Event {
    fn get_keycode(&self) -> (Keycode, &Event) {
        match self {
            Event::KeyDown {
                keycode: Some(Keycode::Right),
                ..
            } => (Keycode::Right, self),
            Event::KeyDown {
                keycode: Some(Keycode::Left),
                ..
            } => (Keycode::Left, self),
            Event::KeyDown {
                keycode: Some(Keycode::Up),
                ..
            } => (Keycode::Up, self),
            Event::KeyDown {
                keycode: Some(Keycode::Space),
                ..
            } => (Keycode::Space, self),
            Event::KeyUp {
                keycode: Some(Keycode::Up),
                ..
            } => (Keycode::Up, self),
            _ => (Keycode::T, self),
        }
    }
}

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

        let duration = Duration::new(0, NANOS);

        'running: loop {
            // Get delta time
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

            for asteroid in asteroids.iter_mut() {
                asteroid.update();
            }

            // spawn asteroid randomly with low chance
            if rand::thread_rng().gen::<f32>() < 0.005 && asteroids.len() < 11 {
                let center = get_edge_pos();
                let new_asteroid = Asteroid::new(40, 100, center.0, center.1);
                asteroids.push(new_asteroid.clone());
            }

            // check collisions
            let mut to_remove: Vec<usize> = Vec::new();
            for i in 0..ship.get_lasers().len() {
                let index = asteroids.iter_mut().position(|f| {
                    collision(f.get_verts(), &ship.get_lasers()[i].get_pos())
                        || collision(f.get_ghost_verts(), &ship.get_lasers()[i].get_pos())
                });
                if index != None {
                    divide_remove(asteroids, index.unwrap());
                    to_remove.push(i);
                }
            }

            for i in to_remove {
                ship.remove_laser(i);
            }

            for asteroid in asteroids.iter_mut() {
                if ship.get_verts().iter_mut().any(|f| {
                    collision(asteroid.get_verts(), f) || collision(asteroid.get_ghost_verts(), f)
                }) {
                    break 'running;
                }
                if ship.get_ghost_verts().iter_mut().any(|f| {
                    collision(asteroid.get_verts(), f) || collision(asteroid.get_ghost_verts(), f)
                }) {
                    break 'running;
                }
            }

            // check wrapping
            wrap_verts(ship);

            asteroids.iter_mut().for_each(|f| wrap_verts(f));

            // draw entities
            stars.iter_mut().for_each(|f| f.draw(&self.canvas));
            ship.draw(&self.canvas);
            asteroids.iter_mut().for_each(|f| f.draw(&self.canvas));

            // let _=

            self.canvas.set_draw_color(Color::BLACK);

            self.canvas.present();
            ::std::thread::sleep(duration);
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
    } else if rand::thread_rng().gen_bool(0.5) {
        height = 0.;
    } else {
        height = SIZE;
    }

    (width, height)
}
