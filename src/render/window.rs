use crate::{
    entity::{
        asteroid::{Asteroid, RemoveAsteroid},
        ship::body::Ship,
        star::Star,
    },
    math::functions::get_random_radius,
    math::vec2::wrap_verts,
    render::text::Text,
};
use rand::Rng;
use sdl2::{
    event::Event,
    keyboard::Keycode,
    pixels::Color,
    render::{Canvas, TextureCreator},
    sys::{SDL_GetPerformanceCounter, SDL_GetPerformanceFrequency},
    ttf::Sdl2TtfContext,
    video::{Window, WindowContext},
    EventPump,
};
use std::{path::Path, time::Duration};

pub const SIZE: f32 = 800.;
pub const MID_SIZE: f32 = SIZE / 2.;
const NANOS: u32 = 1_000_000_000u32 / 60;
const FILE_PATH: &str = "../../assets/open-sans/OpenSans-ExtraBold.ttf";

pub trait GetKey {
    fn get_keycode(&self) -> (Keycode, &Event);
}

impl GetKey for Event {
    fn get_keycode(&self) -> (Keycode, &Event) {
        if let Event::KeyDown { keycode, .. } | Event::KeyUp { keycode, .. } = self {
            return (keycode.unwrap(), self);
        }
        (Keycode::T, self)
    }
}

pub struct Win {
    canvas: Canvas<Window>,
    event_pump: EventPump,
    ttf_context: Sdl2TtfContext,
    texture_creator: TextureCreator<WindowContext>,
}

impl Win {
    pub fn new() -> Result<Win, String> {
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
        let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string()).unwrap();
        let texture_creator = canvas.texture_creator();

        Ok(Win {
            canvas,
            event_pump,
            ttf_context,
            texture_creator,
        })
    }

    pub fn reset(&mut self) -> Result<(), String> {
        let mut now = unsafe { SDL_GetPerformanceCounter() };
        let mut dt;

        // Create Entities
        let stars = &mut Star::new_vec();
        let ship = &mut Ship::new();
        let asteroids = &mut Asteroid::new_vec();

        let duration = Duration::new(0, NANOS);

        let mut score: u32 = 0;

        // Load a font
        let path: &Path = Path::new(FILE_PATH);
        let mut font = self.ttf_context.load_font(path, 28)?;
        font.set_style(sdl2::ttf::FontStyle::BOLD);

        let mut text = Text::new(score, &font, &self.texture_creator).unwrap();

        // Main loop
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
                        ship.do_action(&event);
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
                let (x, y) = get_random_radius();
                let new_asteroid = Asteroid::new(40, 100, x, y);
                asteroids.push(new_asteroid);
            }

            // Check collisions
            for i in (0..ship.get_lasers().len()).rev() {
                let index = asteroids
                    .iter_mut()
                    .position(|asteroid| asteroid.collision(ship.get_lasers()[i].get_pos()));

                if index != None {
                    asteroids.break_up(index.unwrap());

                    ship.remove_laser(i);

                    score += 10;

                    text = Text::new(score, &font, &self.texture_creator).unwrap();
                }
            }

            // Check if game over
            for asteroid in asteroids.iter_mut() {
                if ship.check_collision(asteroid) {
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

            self.canvas.set_draw_color(Color::BLACK);

            self.canvas
                .copy(&text.texture, None, Some(text.target))
                .unwrap();

            self.canvas.present();

            ::std::thread::sleep(duration);
        }

        Ok(())
    }
}
