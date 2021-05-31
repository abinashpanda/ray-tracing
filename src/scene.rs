use crate::aabb::AABB;
use crate::geometry::Geometry;
use crate::hit_record::HitRecord;
use crate::{material::Material, ray::Ray};

pub struct Scene {
    objects: Vec<Geometry>,
}

impl Scene {
    pub fn new() -> Self {
        Scene { objects: vec![] }
    }

    pub fn add_object(&mut self, object: Geometry) {
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<(HitRecord, &Material)> {
        let mut hit_record: Option<(HitRecord, &Material)> = None;
        let mut closest_so_far = t_max;

        for object in self.objects.iter() {
            if let Some((temp_hit_record, material)) = object.hit(ray, t_min, closest_so_far) {
                closest_so_far = temp_hit_record.t;
                hit_record = Some((temp_hit_record, &material));
            };
        }

        hit_record
    }

    pub fn bounding_box(&self) -> Option<AABB> {
        let mut temp_box: Option<AABB> = None;

        for object in self.objects.iter() {
            temp_box = match object.bounding_box() {
                Some(bounding_box) => match temp_box {
                    Some(temp_box_value) => {
                        Some(AABB::surrounding_box(&bounding_box, &temp_box_value))
                    }
                    None => Some(bounding_box),
                },
                None => None,
            }
        }

        temp_box
    }
}
