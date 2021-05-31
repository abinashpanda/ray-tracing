use crate::{aabb::AABB, hit_record::HitRecord, material::Material, ray::Ray, vec_three::Vec3};

pub enum Geometry {
    Sphere {
        center: Vec3,
        radius: f64,
        material: Material,
    },
}

impl Geometry {
    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<(HitRecord, &Material)> {
        match &self {
            &Geometry::Sphere {
                center,
                radius,
                material,
            } => match Geometry::_hit_sphere(ray, t_min, t_max, &center, *radius) {
                Some(hit_record) => Some((hit_record, material)),
                _ => None,
            },
        }
    }

    pub fn bounding_box(&self) -> Option<AABB> {
        match &self {
            Geometry::Sphere { center, radius, .. } => {
                Geometry::_bounding_box_sphere(&center, *radius)
            }
        }
    }

    fn _hit_sphere(
        ray: &Ray,
        t_min: f64,
        t_max: f64,
        center: &Vec3,
        radius: f64,
    ) -> Option<HitRecord> {
        let origin_to_center = ray.origin - *center;
        let a = ray.direction.length_squared();
        let half_b = Vec3::dot(&ray.direction, &origin_to_center);
        let c = origin_to_center.length_squared() - radius * radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let discriminant_sqrt = discriminant.sqrt();
        let mut root = (-half_b - discriminant_sqrt) / a;
        if root < t_min || t_max < root {
            root = (-half_b + discriminant_sqrt) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let t = root;
        let point = ray.at(t);
        let outward_normal = (point - *center) / radius;
        Some(HitRecord::new(t, &point, &outward_normal, ray))
    }

    fn _bounding_box_sphere(center: &Vec3, radius: f64) -> Option<AABB> {
        Some(AABB {
            minimum: *center - Vec3::new(radius, radius, radius),
            maximum: *center + Vec3::new(radius, radius, radius),
        })
    }
}
