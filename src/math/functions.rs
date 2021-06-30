use crate::render::window::MID_SIZE;
use rand::Rng;

pub fn rand_angle() -> f32 {
    2. * std::f32::consts::PI * rand::thread_rng().gen::<f32>()
}

pub fn get_random_radius() -> (f32, f32) {
    let angle: f64 = rand::thread_rng().gen::<f64>() * std::f64::consts::PI * 2.;

    let r = MID_SIZE as f64;

    let x = r * angle.cos() + MID_SIZE as f64;
    let y = r * angle.sin() + MID_SIZE as f64;

    (x as f32, y as f32)
}
