use crate::ray::Ray;
use crate::vec_three::Vec3;

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

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        HittableList { objects: vec![] }
    }

    pub fn add_object(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut hit_record: Option<HitRecord> = None;
        let mut closest_so_far = t_max;

        for object in self.objects.iter() {
            if let Some(temp_hit_record) = object.hit(ray, t_min, closest_so_far) {
                closest_so_far = temp_hit_record.t;
                hit_record = Some(temp_hit_record);
            };
        }

        hit_record
    }
}
