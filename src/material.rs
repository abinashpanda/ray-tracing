use crate::{hit_record::HitRecord, ray::Ray, vec_three::Vec3};
use rand::Rng;

#[derive(Clone, Copy)]
pub enum Material {
    Lambert { color: Vec3 },
    Metal { color: Vec3, fuzz: f32 },
    Dielectric { color: Vec3, refraction_index: f32 },
}

impl Material {
    pub fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Vec3, Ray)> {
        match self {
            &Material::Lambert { color } => {
                Some((color, Material::_scatter_lambertian(ray_in, hit_record)))
            }
            &Material::Metal { color, fuzz } => {
                match Material::_scatter_metal(ray_in, hit_record, fuzz) {
                    Some(ray) => Some((color, ray)),
                    _ => None,
                }
            }
            &Material::Dielectric {
                color,
                refraction_index,
            } => Some((
                color,
                Material::_scatter_dialectric(ray_in, hit_record, refraction_index),
            )),
        }
    }

    fn _scatter_lambertian(_ray_in: &Ray, hit_record: &HitRecord) -> Ray {
        let mut scattered_ray_direction = hit_record.normal + Vec3::random_unit_vector();
        if scattered_ray_direction.near_zero() {
            scattered_ray_direction = hit_record.normal;
        }
        Ray {
            origin: hit_record.point,
            direction: scattered_ray_direction,
        }
    }

    fn _scatter_metal(ray_in: &Ray, hit_record: &HitRecord, fuzz: f32) -> Option<Ray> {
        let reflected_ray = Vec3::reflect(&ray_in.direction.unit_vector(), &hit_record.normal)
            + Vec3::random_in_unit_sphere() * fuzz;
        if Vec3::dot(&reflected_ray, &hit_record.normal) > 0.0 {
            return Some(Ray {
                origin: hit_record.point,
                direction: reflected_ray,
            });
        }
        None
    }

    fn _scatter_dialectric(ray_in: &Ray, hit_record: &HitRecord, refraction_index: f32) -> Ray {
        let mut rng = rand::thread_rng();

        let refraction_ratio = match hit_record.is_front_face() {
            true => 1.0 / refraction_index,
            false => refraction_index,
        };

        let cos_theta =
            (Vec3::dot(&ray_in.direction.unit_vector(), &hit_record.normal) * -1.0).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let can_refract = refraction_ratio * sin_theta <= 1.0;
        let direction = match can_refract {
            true => {
                let reflect_prob = Material::_shlick_approx(cos_theta, refraction_ratio);
                match reflect_prob > rng.gen() {
                    true => -Vec3::reflect(&ray_in.direction.unit_vector(), &hit_record.normal),
                    false => Vec3::refract(
                        &ray_in.direction.unit_vector(),
                        &hit_record.normal,
                        refraction_ratio,
                    ),
                }
            }
            false => -Vec3::reflect(&ray_in.direction.unit_vector(), &hit_record.normal),
        };

        Ray {
            origin: hit_record.point,
            direction,
        }
    }

    fn _shlick_approx(cosine: f32, refraction_index: f32) -> f32 {
        let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}
