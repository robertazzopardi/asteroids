use crate::window::SIZE;
use rand::Rng;
use std::f32::consts::PI;

#[derive(Clone, PartialEq, Debug)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

pub trait UpdateVerts {
    fn get_verts(&mut self) -> &mut Vec<Vec2>;
    fn get_ghost_verts(&mut self) -> &mut Vec<Vec2>;

    fn swap(&mut self);
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn magnitude(&self) -> f32 {
        (self.x.powf(2.) + self.y.powf(2.)).sqrt()
    }
}

pub fn rand_angle() -> f32 {
    2. * PI * rand::thread_rng().gen::<f32>()
}

pub fn get_center(verts: &[Vec2]) -> Vec2 {
    let sum_x: f32 = verts.iter().map(|f| f.x).sum();
    let sum_y: f32 = verts.iter().map(|f| f.y).sum();

    let x = sum_x / verts.len() as f32;
    let y = sum_y / verts.len() as f32;

    Vec2::new(x, y)
}

pub fn rotate(verts: &mut Vec<Vec2>, angle: f32) {
    let origin = get_center(verts);
    let cos = angle.cos();
    let sin = angle.sin();

    for pos in verts.iter_mut() {
        let px = pos.x - origin.x;
        let py = pos.y - origin.y;

        let tmp_x = origin.x + cos * px - sin * py;
        pos.y = origin.y + sin * px + cos * py;
        pos.x = tmp_x;
    }
}

pub fn wrap_point(point: &mut Vec2) {
    if point.y < 0. {
        point.y += SIZE;
    }
    if point.y > SIZE {
        point.y -= SIZE;
    }
    if point.x < 0. {
        point.x += SIZE;
    }
    if point.x > SIZE {
        point.x -= SIZE;
    }
}

// pub fn point_collision(vec1: &Vec2, vec2: &Vec2, radius: &f32) -> bool {
//     ((vec1.x - vec2.x) * (vec1.x - vec2.x) + (vec1.y - vec2.y) * (vec1.y - vec2.y)).sqrt() < *radius
// }

// Algorithm from https://stackoverflow.com/a/2922778/8742929
// Raytracing like
// pub fn polygon_collision(verts: &[Vec2], point: &Vec2) -> bool {
//     let mut collision = false;
//     let mut j = verts.len() - 1;

//     for i in 0..verts.len() {
//         if ((verts[i].y > point.y) != (verts[j].y > point.y))
//             && (point.x
//                 < (verts[j].x - verts[i].x) * (point.y - verts[i].y) / (verts[j].y - verts[i].y)
//                     + verts[i].x)
//         {
//             collision = !collision;
//         }
//         j = i;
//     }

//     collision
// }

pub fn wrap_verts<T: UpdateVerts>(main: &mut T) {
    let mut x = 0.;
    let mut y = 0.;

    if main.get_verts().iter().any(|f| f.y < 0.) {
        y = SIZE;
    }
    if main.get_verts().iter().any(|f| f.y > SIZE) {
        y = -SIZE;
    }
    if main.get_verts().iter().any(|f| f.x < 0.) {
        x = SIZE;
    }
    if main.get_verts().iter().any(|f| f.x > SIZE) {
        x = -SIZE;
    }

    if (x - y).abs() > 0. {
        for i in 0..main.get_ghost_verts().len() {
            main.get_ghost_verts()[i].x = main.get_verts()[i].x + x;
            main.get_ghost_verts()[i].y = main.get_verts()[i].y + y;
        }
    }

    if main
        .get_verts()
        .iter()
        .all(|f| (f.y < 0. || f.y > SIZE) || (f.x < 0. || f.x > SIZE))
    {
        main.swap();
    }
}

pub fn convert_to_xy_vec(vec: &[Vec2]) -> (Vec<i16>, Vec<i16>) {
    let mut array_x = Vec::new();
    let mut array_y = Vec::new();

    for v in vec {
        array_x.push(v.x as i16);
        array_y.push(v.y as i16);
    }

    (array_x, array_y)
}
