extern crate sdl2;

mod asteroid;
mod math;
mod ship;
mod star;
mod window;

use window::Win;

fn main() {
    Win::new().reset();
}
