use crate::vec_three::Vec3;

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn from(origin: &Vec3, direction: &Vec3) -> Ray {
        Ray {
            origin: Vec3::from_vec(origin),
            direction: Vec3::from_vec(direction),
        }
    }

    pub fn at(self, t: f64) -> Vec3 {
        self.origin + self.direction * t
    }
}
