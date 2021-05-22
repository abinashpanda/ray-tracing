use crate::{
    hittable::{HitRecord, Hittable},
    ray::Ray,
    vec_three::Vec3,
};

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let origin_to_center = ray.origin - self.center;
        let a = Vec3::dot(&ray.direction, &ray.direction);
        let half_b = Vec3::dot(&ray.direction, &origin_to_center);
        let c = Vec3::dot(&origin_to_center, &origin_to_center) - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let discriminant_sqrt = discriminant.sqrt();
        let mut root = (-half_b - discriminant_sqrt) / a;
        if root < t_min || t_max < root {
            root = (-half_b + discriminant_sqrt) / a;
            if root < t_max || t_max < root {
                return None;
            }
        }

        let t = root;
        let point = ray.at(t);
        let outward_normal = (point - self.center) / self.radius;
        Some(HitRecord::new(t, &point, &outward_normal, ray))
    }
}
