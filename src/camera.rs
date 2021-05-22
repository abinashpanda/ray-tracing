use crate::{ray::Ray, vec_three::Vec3};

pub struct Camera {
    origin: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Vec3,
}

impl Camera {
    pub fn new(aspect_ratio: f64, viewport_height: f64, focal_length: f64, origin: &Vec3) -> Self {
        let origin = *origin;
        let viewport_width = viewport_height * aspect_ratio;
        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport_height, 0.0);
        let lower_left_corner =
            origin - (horizontal / 2.0) - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }

    pub fn get_origin_ray(&self, u: f64, v: f64) -> Ray {
        // as the camera is present in the origin, the direction vector would the position of the point
        let direction = self.lower_left_corner + self.horizontal * u + self.vertical * v;

        Ray {
            origin: self.origin,
            direction,
        }
    }
}
