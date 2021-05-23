use rand::Rng;

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
    pub fuzz: f64,
}

impl Material for MetalMaterial {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Vec3, Ray)> {
        let reflected_ray = Vec3::reflect(&ray_in.direction.unit_vector(), &hit_record.normal)
            + Vec3::random_in_unit_sphere() * self.fuzz;
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

pub struct DielectricMaterial {
    pub refraction_index: f64,
}

impl Material for DielectricMaterial {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Vec3, Ray)> {
        let mut rng = rand::thread_rng();

        let refraction_ratio = if hit_record.is_front_face() {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = &ray_in.direction.unit_vector();
        let cos_theta = Vec3::dot(&-*unit_direction, &hit_record.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let schilck_approx =
            DielectricMaterial::reflectance(cos_theta, refraction_ratio) > rng.gen();

        let refracted_ray = if cannot_refract || schilck_approx {
            Vec3::reflect(unit_direction, &hit_record.normal)
        } else {
            Vec3::refract(unit_direction, &hit_record.normal, refraction_ratio)
        };

        Some((
            Vec3::identity(),
            Ray {
                origin: hit_record.point,
                direction: refracted_ray,
            },
        ))
    }
}

impl DielectricMaterial {
    pub fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
        let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}
