use crate::{hittable::HitRecord, ray::Ray, vec_three::Vec3};

pub trait Material {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Vec3, Ray)>;
}

pub struct LambertMaterial {
    pub color: Vec3,
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
        Some((self.color, scattered_ray))
    }
}

pub struct MetalMaterial {
    pub color: Vec3,
}

impl Material for MetalMaterial {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Vec3, Ray)> {
        let reflected_ray = Vec3::reflect(&ray_in.direction.unit_vector(), &hit_record.normal);
        if Vec3::dot(&reflected_ray, &hit_record.normal) > 0.0 {
            return Some((
                self.color,
                Ray {
                    origin: hit_record.point,
                    direction: reflected_ray,
                },
            ));
        }
        None
    }
}
