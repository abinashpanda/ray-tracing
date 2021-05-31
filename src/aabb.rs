use crate::{ray::Ray, vec_three::Vec3};

pub struct AABB {
    pub minimum: Vec3,
    pub maximum: Vec3,
}

impl AABB {
    pub fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> bool {
        // TODO: use the optimized hit method
        let mut t_min_found = t_min;
        let mut t_max_found = t_max;
        for a in 0..3 {
            let t_min_intersection =
                (self.minimum.get(a) - ray.origin.get(a)) / ray.direction.get(a);
            let t_max_intersection =
                (self.maximum.get(a) - ray.origin.get(a)) / ray.direction.get(a);
            let t0 = t_min_intersection.min(t_max_intersection);
            let t1 = t_min_intersection.max(t_max_intersection);
            t_min_found = t_min_found.max(t0);
            t_max_found = t_max_found.min(t1);
            if t_max_found < t_min_found {
                return false;
            }
        }

        true
    }

    pub fn surrounding_box(box_0: &AABB, box_1: &AABB) -> AABB {
        let minimum = Vec3::new(
            box_0.minimum.x.min(box_1.minimum.x),
            box_0.minimum.y.min(box_1.minimum.y),
            box_0.minimum.z.min(box_1.minimum.z),
        );

        let maximum = Vec3::new(
            box_0.maximum.x.max(box_1.maximum.x),
            box_0.maximum.y.max(box_1.maximum.y),
            box_0.maximum.z.max(box_1.maximum.z),
        );

        AABB { minimum, maximum }
    }
}
