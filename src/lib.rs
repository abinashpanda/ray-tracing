extern crate image as image_crate;
extern crate rand;

pub mod camera;
pub mod hittable;
pub mod image;
pub mod ray;
pub mod sphere;
pub mod vec_three;

use crate::image::write_color;
use camera::Camera;
use hittable::{Hittable, HittableList};
use image_crate::RgbImage;
use rand::Rng;
use ray::Ray;
use vec_three::Vec3;

pub const IMAGE_ASPECT_RATIO: f64 = 16.0 / 9.0;
pub const IMAGE_WIDTH: u32 = 1024;
pub const IMAGE_HEIGHT: u32 = ((IMAGE_WIDTH as f64) / IMAGE_ASPECT_RATIO) as u32;
pub const SAMPLES_PER_PIXEL: u8 = 20;

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
const T_MAX: f64 = 1000.0;

pub fn ray_trace(camera: &Camera, world: &HittableList, img: &mut RgbImage) {
    let mut rng = rand::thread_rng();

    for i in 1..IMAGE_WIDTH {
        for mut j in 1..IMAGE_HEIGHT {
            let mut color = Vec3::new(0.0, 0.0, 0.0);
            for _ in 0..SAMPLES_PER_PIXEL {
                let random_num: f64 = rng.gen();

                let u = (i as f64 + random_num) / ((IMAGE_WIDTH as f64) - 1.0);
                let v = (j as f64 + random_num) / ((IMAGE_HEIGHT as f64) - 1.0);

                let mut depth: u8 = 50;
                let ray = camera.get_origin_ray(u, v);
                color = color + ray_color(&ray, &world, &mut depth);
            }

            // subtract IMAGE_HEIGHT - j as the we want to move the origin from top left to bottom left
            j = IMAGE_HEIGHT - j;
            write_color(img, i, j, &color, SAMPLES_PER_PIXEL);
        }
    }
}

pub fn ray_color(ray: &Ray, world: &HittableList, depth: &mut u8) -> Vec3 {
    if *depth == 0 {
        return Vec3::new(0.0, 0.0, 0.0);
    }

    if let Some(hit_record) = world.hit(ray, T_MIN, T_MAX) {
        let target = hit_record.point + hit_record.normal + Vec3::random_in_unit_sphere();
        let ray_to_target = Ray {
            origin: hit_record.point,
            direction: target - hit_record.point,
        };
        *depth -= 1;
        return ray_color(&ray_to_target, world, depth) * 0.5;
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
