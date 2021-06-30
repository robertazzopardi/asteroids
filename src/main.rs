mod entity;
mod math;
mod render;

use render::window::Win;

fn main() -> Result<(), String> {
    Win::new().unwrap().reset()
}
