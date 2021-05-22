use crate::{hittable::HitRecord, ray::Ray, vec_three::Vec3};

pub trait Material {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Vec3, Ray)>;
}

pub struct LambertMaterial {
    pub albedo: Vec3,
}

impl Material for LambertMaterial {
    fn scatter(&self, _ray_in: &Ray, hit_record: &HitRecord) -> Option<(Vec3, Ray)> {
        let mut scattered_ray_direction = hit_record.normal + Vec3::random_unit_vector();
        if scattered_ray_direction.near_zero() {
            scattered_ray_direction = hit_record.normal;
        }
        let scattered_ray = Ray {
            origin: hit_record.point,
            direction: scattered_ray_direction,
        };
        Some((self.albedo, scattered_ray))
    }
}
