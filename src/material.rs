use crate::{hittable::HitRecord, ray::Ray, vec_three::Vec3};
use rand::Rng;

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
    pub color: Vec3,
    pub refraction_index: f64,
}

impl Material for DielectricMaterial {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Vec3, Ray)> {
        let mut rng = rand::thread_rng();

        let refraction_ratio = match hit_record.is_front_face() {
            true => 1.0 / self.refraction_index,
            false => self.refraction_index,
        };

        let cos_theta =
            (Vec3::dot(&ray_in.direction.unit_vector(), &hit_record.normal) * -1.0).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let can_refract = refraction_ratio * sin_theta <= 1.0;
        let direction = match can_refract {
            true => {
                let reflect_prob = DielectricMaterial::shlick_approx(cos_theta, refraction_ratio);
                match reflect_prob > rng.gen() {
                    true => -Vec3::reflect(&ray_in.direction.unit_vector(), &hit_record.normal),
                    false => DielectricMaterial::refract(
                        &ray_in.direction.unit_vector(),
                        &hit_record.normal,
                        refraction_ratio,
                    ),
                }
            }
            false => -Vec3::reflect(&ray_in.direction.unit_vector(), &hit_record.normal),
        };

        Some((
            self.color,
            Ray {
                origin: hit_record.point,
                direction,
            },
        ))
    }
}

impl DielectricMaterial {
    pub fn shlick_approx(cosine: f64, refraction_index: f64) -> f64 {
        let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }

    pub fn refract(unit_vector: &Vec3, normal: &Vec3, refraction_ratio: f64) -> Vec3 {
        let uv = *unit_vector;
        let n = *normal;
        let cos_theta = (Vec3::dot(unit_vector, normal) * -1.0).min(1.0);
        let r_perp = (uv + n * cos_theta) * refraction_ratio;
        let r_parallel = n * (-(1.0 - r_perp.length_squared()).abs().sqrt());
        r_perp + r_parallel
    }
}
