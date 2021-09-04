use crate::render::window::SIZE;
use sdl2::{
    pixels::Color,
    rect::Rect,
    render::{Texture, TextureCreator, TextureQuery},
    ttf::Font,
    video::WindowContext,
};

pub struct Text<'a> {
    pub texture: Texture<'a>,
    pub target: Rect,
}

impl<'a> Text<'_> {
    pub fn new(
        score: u32,
        font: &Font,
        texture_creator: &'a TextureCreator<WindowContext>,
    ) -> Result<Text<'a>, String> {
        let text = "Score: ".to_string() + &score.to_string();

        let surface = font
            .render(&text)
            .blended(Color::WHITE)
            .map_err(|e| e.to_string())?;

        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;

        let TextureQuery { width, height, .. } = texture.query();

        // If the example text is too big for the screen, downscale it (and center irregardless)
        let padding = 64;
        Ok(Text {
            texture,
            target: get_centered_rect(width, height, SIZE as u32 - padding, SIZE as u32 - padding),
        })
    }
}

/// Handle the annoying Rect i32
macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    )
);

/// Scale fonts to a reasonable size when they're too big (though they might look less smooth)
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
