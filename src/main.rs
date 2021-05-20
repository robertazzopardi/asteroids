extern crate sdl2;

mod asteroid;
mod math;
mod ship;
mod star;
mod window;

use crate::ship::Ship;
use crate::star::Star;
use crate::window::Win;

fn main() {
    let ship = Ship::new();

    let stars: Vec<Star> = Star::new_vec();

    // let mut asteroids: [Asteroid; 10] = [];

    let mut window = Win::new();

    window.main_loop(stars, ship);
}
