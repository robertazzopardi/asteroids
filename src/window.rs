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
    rect::Rect,
    render::{Canvas, Texture, TextureCreator, TextureQuery},
    sys::{SDL_GetPerformanceCounter, SDL_GetPerformanceFrequency},
    ttf::Font,
    ttf::Sdl2TtfContext,
    video::{Window, WindowContext},
    EventPump,
};
use std::path::Path;
use std::time::Duration;

pub const SIZE: f32 = 800.;
pub const MID_SIZE: f32 = SIZE / 2.;
const NANOS: u32 = 1_000_000_000u32 / 60;
const FILE_PATH: &str = "../../assets/open-sans/OpenSans-ExtraBold.ttf";

/// Handle the annoying Rect i32
macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    )
);

pub trait GetKey {
    fn get_keycode(&self) -> (Keycode, &Event);
}

impl GetKey for Event {
    fn get_keycode(&self) -> (Keycode, &Event) {
        match self {
            // Right
            Event::KeyDown {
                keycode: Some(Keycode::Right),
                ..
            }
            | Event::KeyUp {
                keycode: Some(Keycode::Right),
                ..
            } => (Keycode::Right, self),
            // Left
            Event::KeyDown {
                keycode: Some(Keycode::Left),
                ..
            }
            | Event::KeyUp {
                keycode: Some(Keycode::Left),
                ..
            } => (Keycode::Left, self),
            // Up
            Event::KeyDown {
                keycode: Some(Keycode::Up),
                ..
            }
            | Event::KeyUp {
                keycode: Some(Keycode::Up),
                ..
            } => (Keycode::Up, self),
            // Space
            Event::KeyDown {
                keycode: Some(Keycode::Space),
                ..
            } => (Keycode::Space, self),
            // None for now
            _ => (Keycode::T, self),
        }
    }
}

pub struct Win {
    canvas: Canvas<Window>,
    event_pump: EventPump,
    ttf_context: Sdl2TtfContext,
    texture_creator: TextureCreator<WindowContext>,
}

struct Text<'a> {
    texture: Texture<'a>,
    target: Rect,
}

impl<'a> Text<'_> {
    fn new(
        score: u32,
        font: &Font,
        texture_creator: &'a TextureCreator<WindowContext>,
    ) -> Result<Text<'a>, String> {
        let text = "Score: ".to_string() + &score.to_string();

        let surface = font
            .render(&text)
            // .blended(Color::RGBA(255, 0, 0, 255))
            .blended(Color::WHITE)
            .map_err(|e| e.to_string())?;

        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;

        let TextureQuery { width, height, .. } = texture.query();

        // If the example text is too big for the screen, downscale it (and center irregardless)
        let padding = 64;
        let target = get_centered_rect(width, height, SIZE as u32 - padding, SIZE as u32 - padding);

        Ok(Text { texture, target })
    }
}

// Scale fonts to a reasonable size when they're too big (though they might look less smooth)
fn get_centered_rect(rect_width: u32, rect_height: u32, cons_width: u32, cons_height: u32) -> Rect {
    let wr = rect_width as f32 / cons_width as f32;
    let hr = rect_height as f32 / cons_height as f32;

    let (w, h) = if wr > 1f32 || hr > 1f32 {
        println!("Scaling down! The text will look worse!");
        if wr > hr {
            let h = (rect_height as f32 / wr) as i32;
            (cons_width as i32, h)
        } else {
            let w = (rect_width as f32 / hr) as i32;
            (w, cons_height as i32)
        }
    } else {
        (rect_width as i32, rect_height as i32)
    };

    let cx = (SIZE as i32 - w) / 2;
    let cy = (SIZE as i32 - h) / 2;
    rect!(cx, cy - (SIZE * 0.9) as i32 / 2, w, h)
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

        let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
        let texture_creator = canvas.texture_creator();

        Ok(Win {
            canvas,
            event_pump,
            ttf_context,
            texture_creator,
        })
    }

    pub fn reset(&mut self) -> Result<(), String> {
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

        let mut score: u32 = 0;

        // font
        // Load a font
        let path: &Path = Path::new(FILE_PATH);
        let mut font = self.ttf_context.load_font(path, 28)?;
        font.set_style(sdl2::ttf::FontStyle::BOLD);

        let mut text = Text::new(score, &font, &self.texture_creator).unwrap();

        // font

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
                let center = get_edge_pos();
                let new_asteroid = Asteroid::new(40, 100, center.0, center.1);
                asteroids.push(new_asteroid.clone());
            }

            // Check collisions
            let mut to_remove: Vec<usize> = Vec::new();
            for i in 0..ship.get_lasers().len() {
                let index = asteroids.iter_mut().position(|f| {
                    collision(f.get_verts(), ship.get_lasers()[i].get_pos())
                        || collision(f.get_ghost_verts(), ship.get_lasers()[i].get_pos())
                });
                if index != None {
                    divide_remove(asteroids, index.unwrap());
                    to_remove.push(i);
                }
            }

            // If lasers have hit asteroids remove them
            for i in to_remove {
                ship.remove_laser(i);
                score += 10;

                text = Text::new(score, &font, &self.texture_creator).unwrap();
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

            self.canvas.copy(&text.texture, None, Some(text.target))?;

            self.canvas.present();
            ::std::thread::sleep(duration);
        }

        Ok(())
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
