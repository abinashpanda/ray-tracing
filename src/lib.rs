pub mod camera;
pub mod hittable;
pub mod image;
pub mod ray;
pub mod sphere;
pub mod vec_three;

use hittable::{Hittable, HittableList};
use ray::Ray;
use vec_three::Vec3;

const WHITE_COLOR: Vec3 = Vec3 {
    x: 1.0,
    y: 1.0,
    z: 1.0,
};
const SKY_BLUE_COLOR: Vec3 = Vec3 {
    x: 0.5,
    y: 0.7,
    z: 1.0,
};

const T_MIN: f64 = 0.01;
const T_MAX: f64 = 100.0;

pub fn ray_color(ray: &Ray, world: &HittableList) -> Vec3 {
    if let Some(hit_record) = world.hit(ray, T_MIN, T_MAX) {
        return (hit_record.normal + 1.0) * 0.5;
    }

    sky_color(&ray)
}

fn sky_color(ray: &Ray) -> Vec3 {
    let unit_direction = ray.direction.unit_vector();
    // normalize the t to be a value between 0 and 1
    let t = 0.5 * (unit_direction.y + 1.0);
    // interpolate from sky_blue_color to white using the y
    WHITE_COLOR * (1.0 - t) + SKY_BLUE_COLOR * t
}
