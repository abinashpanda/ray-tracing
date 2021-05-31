use crate::{ray::Ray, vec_three::Vec3};

pub struct Camera {
    origin: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Vec3,
    lens_radius: f32,
}

impl Camera {
    pub fn new(
        aspect_ratio: f32,
        vertical_field_of_view: f32,
        look_from: &Vec3,
        look_at: &Vec3,
        vup: &Vec3,
        aperature: f32,
        focus_dist: f32,
    ) -> Self {
        let theta = vertical_field_of_view.to_radians();
        let height = (theta / 2.0).tan();
        let viewport_height = 2.0 * height;
        let viewport_width = viewport_height * aspect_ratio;

        let w = (*look_from - *look_at).unit_vector();
        let u = Vec3::cross(vup, &w).unit_vector();
        let v = Vec3::cross(&w, &u);

        let origin = *look_from;

        let horizontal = u * viewport_width * focus_dist;
        let vertical = v * viewport_height * focus_dist;
        let lower_left_corner = origin - (horizontal / 2.0) - vertical / 2.0 - w * focus_dist;

        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
            lens_radius: aperature / 2.0,
        }
    }

    pub fn get_origin_ray(&self, u: f32, v: f32) -> Ray {
        let rand_direction = Vec3::random_in_unit_disk() * self.lens_radius;
        let offset = u * rand_direction.x + v * rand_direction.y;
        // as the camera is present in the origin, the direction vector would the position of the point
        let direction =
            self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin - offset;

        Ray {
            origin: self.origin + offset,
            direction,
        }
    }
}
