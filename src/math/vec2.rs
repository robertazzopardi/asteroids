use crate::render::window::SIZE;

#[derive(Clone)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn magnitude(&self) -> f32 {
        (self.x.powf(2.) + self.y.powf(2.)).sqrt()
    }

    pub fn wrap_point(&mut self) {
        if self.y < 0. {
            self.y += SIZE;
        }
        if self.y > SIZE {
            self.y -= SIZE;
        }
        if self.x < 0. {
            self.x += SIZE;
        }
        if self.x > SIZE {
            self.x -= SIZE;
        }
    }
}

pub trait UpdateVerts {
    fn get_verts(&mut self) -> &mut Vec<Vec2>;
    fn get_ghost_verts(&mut self) -> &mut Vec<Vec2>;
    fn swap(&mut self);
}

pub fn wrap_verts<T: UpdateVerts>(main: &mut T) {
    let mut dx = 0.;
    let mut dy = 0.;

    for Vec2 { x, y } in main.get_verts().iter() {
        if y < &0. {
            dy = SIZE;
        }
        if y > &SIZE {
            dy = -SIZE;
        }
        if x < &0. {
            dx = SIZE;
        }
        if x > &SIZE {
            dx = -SIZE;
        }
    }

    if (dx - dy).abs() > 0. {
        for i in 0..main.get_ghost_verts().len() {
            main.get_ghost_verts()[i].x = main.get_verts()[i].x + dx;
            main.get_ghost_verts()[i].y = main.get_verts()[i].y + dy;
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

pub trait Vec2Vec {
    fn get_center(&self) -> Vec2;
    fn rotate(&mut self, angle: f32);
    fn convert_to_xy_vec(&self) -> (Vec<i16>, Vec<i16>);
    fn collision(&self, point: &Vec2) -> bool;
}

impl Vec2Vec for Vec<Vec2> {
    fn get_center(&self) -> Vec2 {
        let sum_x: f32 = self.iter().map(|f| f.x).sum();
        let sum_y: f32 = self.iter().map(|f| f.y).sum();

        let x = sum_x / self.len() as f32;
        let y = sum_y / self.len() as f32;

        Vec2::new(x, y)
    }

    fn rotate(&mut self, angle: f32) {
        let origin = self.get_center();
        let cos = angle.cos();
        let sin = angle.sin();

        for pos in self.iter_mut() {
            let px = pos.x - origin.x;
            let py = pos.y - origin.y;

            let tmp_x = origin.x + cos * px - sin * py;
            pos.y = origin.y + sin * px + cos * py;
            pos.x = tmp_x;
        }
    }

    fn convert_to_xy_vec(&self) -> (Vec<i16>, Vec<i16>) {
        let mut array_x = Vec::new();
        let mut array_y = Vec::new();

        for v in self {
            array_x.push(v.x as i16);
            array_y.push(v.y as i16);
        }

        (array_x, array_y)
    }

    fn collision(&self, point: &Vec2) -> bool {
        let mut collision = false;
        let mut j = self.len() - 1;

        for i in 0..self.len() {
            if ((self[i].y > point.y) != (self[j].y > point.y))
                && (point.x
                    < (self[j].x - self[i].x) * (point.y - self[i].y) / (self[j].y - self[i].y)
                        + self[i].x)
            {
                collision = !collision;
            }
            j = i;
        }

        collision
    }
}
