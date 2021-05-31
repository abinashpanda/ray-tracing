use crate::vec_three::Vec3;

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn from(origin: &Vec3, direction: &Vec3) -> Ray {
        Ray {
            origin: Vec3::from(origin),
            direction: Vec3::from(direction),
        }
    }

    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + self.direction * t
    }
}
