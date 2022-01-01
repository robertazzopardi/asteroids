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
    ttf::Sdl2TtfContext,
    video::{Window, WindowContext},
    EventPump, TimerSubsystem,
};
use std::path::Path;

pub const SIZE: f32 = 800.;
pub const MID_SIZE: f32 = SIZE / 2.;
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
    timer_subsystem: TimerSubsystem,
}

impl Win {
    pub fn new() -> Result<Win, String> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;

        let window = video_subsystem
            .window("Asteroids", SIZE as u32, SIZE as u32)
            .position_centered()
            .build()
            .unwrap();

        let canvas = window.into_canvas().present_vsync().build().unwrap();

        let texture_creator = canvas.texture_creator();

        Ok(Win {
            canvas,
            event_pump: sdl_context.event_pump()?,
            ttf_context: sdl2::ttf::init().map_err(|e| e.to_string())?,
            texture_creator,
            timer_subsystem: sdl_context.timer()?,
        })
    }

    pub fn reset(&mut self) -> Result<(), String> {
        // let mut dt = 0.;

        // Create Entities
        let stars = &mut Star::new_vec();
        let ship = &mut Ship::new();
        let asteroids = &mut Asteroid::new_vec();

        // Load a font
        let path: &Path = Path::new(FILE_PATH);
        let mut font = self.ttf_context.load_font(path, 28)?;
        font.set_style(sdl2::ttf::FontStyle::BOLD);

        let mut score: u32 = 0;
        let mut text = Text::new(score, &font, &self.texture_creator)?;

        let mut last_update = self.timer_subsystem.ticks();

        // Main loop
        'running: loop {
            let start = self.timer_subsystem.performance_counter();

            // Do event loop
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

            // spawn asteroid randomly with low chance
            if rand::thread_rng().gen::<f32>() < 0.005 && asteroids.len() < 11 {
                let (x, y) = get_random_radius();
                asteroids.push(Asteroid::new(40, 100, x, y));
            }

            // Check collisions
            for i in (0..ship.get_lasers().len()).rev() {
                if let Some(index) = asteroids
                    .iter_mut()
                    .position(|asteroid| asteroid.collision(ship.get_lasers()[i].get_pos()))
                {
                    asteroids.break_up(index);
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

            // Do physics loop
            let current: u32 = self.timer_subsystem.ticks();
            let dt = (current - last_update) as f32 / 1000.;

            // Set updated time
            last_update = current;

            ship.update(dt);
            for asteroid in asteroids.iter_mut() {
                asteroid.update();
            }

            // Do rendering loop
            self.canvas.clear();

            // check wrapping
            wrap_verts(ship);
            asteroids.iter_mut().for_each(wrap_verts);

            // Render
            stars.iter_mut().for_each(|f| f.draw(&self.canvas));
            ship.draw(&self.canvas);
            asteroids.iter_mut().for_each(|f| f.draw(&self.canvas));

            self.canvas.set_draw_color(Color::BLACK);

            // Draw text
            self.canvas.copy(&text.texture, None, Some(text.target))?;

            // Display
            self.canvas.present();

            let end = self.timer_subsystem.performance_counter();
            let _elapsed =
                (end - start) as f32 / self.timer_subsystem.performance_frequency() as f32;
            // println!("{}", (1. / _elapsed).round());
        }

        Ok(())
    }
}
