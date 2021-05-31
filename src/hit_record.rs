use crate::{ray::Ray, vec_three::Vec3};

pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f64,
    front_face: bool,
}

impl HitRecord {
    pub fn new(t: f64, point: &Vec3, outward_normal: &Vec3, ray: &Ray) -> Self {
        let mut normal = *outward_normal;
        let front_face;
        if Vec3::dot(&ray.direction, outward_normal) > 0.0 {
            front_face = false;
            normal = -normal;
        } else {
            front_face = true;
        }

        HitRecord {
            t,
            point: *point,
            normal,
            front_face,
        }
    }

    pub fn is_front_face(&self) -> bool {
        self.front_face
    }
}
