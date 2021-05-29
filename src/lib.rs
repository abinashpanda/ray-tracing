extern crate image as image_crate;
extern crate rand;

pub mod camera;
pub mod hittable;
pub mod image;
pub mod material;
pub mod ray;
pub mod sphere;
pub mod vec_three;

use crate::image::write_color;
use camera::Camera;
use hittable::{Hittable, HittableList};
use image_crate::RgbImage;
use pbr::ProgressBar;
use rand::Rng;
use ray::Ray;
use vec_three::Vec3;

pub const IMAGE_ASPECT_RATIO: f64 = 3.0 / 2.0;
pub const IMAGE_WIDTH: u32 = 800;
pub const IMAGE_HEIGHT: u32 = ((IMAGE_WIDTH as f64) / IMAGE_ASPECT_RATIO) as u32;
pub const SAMPLES_PER_PIXEL: u32 = 20;
const PB_INCREMENT: u32 = 1000;
const MAX_RAYS: u8 = 50;

const SKY_START_COLOR: Vec3 = Vec3 {
    x: 0.5,
    y: 0.7,
    z: 1.0,
};
const SKY_END_COLOR: Vec3 = Vec3 {
    x: 1.0,
    y: 1.0,
    z: 1.0,
};

const T_MIN: f64 = 0.001;
const T_MAX: f64 = f64::MAX;

pub fn ray_trace(camera: &Camera, world: &HittableList, img: &mut RgbImage) {
    let mut rng = rand::thread_rng();
    let mut pb =
        ProgressBar::new((IMAGE_WIDTH * IMAGE_HEIGHT * SAMPLES_PER_PIXEL / PB_INCREMENT) as u64);
    let mut count = 0;

    for i in 1..IMAGE_WIDTH {
        for mut j in 1..IMAGE_HEIGHT {
            let mut color = Vec3::zero();
            for _ in 0..SAMPLES_PER_PIXEL {
                count += 1;

                let random_num: f64 = rng.gen();

                let u = (i as f64 + random_num) / ((IMAGE_WIDTH as f64) - 1.0);
                let v = (j as f64 + random_num) / ((IMAGE_HEIGHT as f64) - 1.0);

                // limit the number rays to be 10
                let mut depth: u8 = MAX_RAYS;
                let ray = camera.get_origin_ray(u, v);
                color = color + ray_color(&ray, &world, &mut depth);

                if count % PB_INCREMENT == 0 {
                    pb.inc();
                }
            }

            // subtract IMAGE_HEIGHT - j as the we want to move the origin from top left to bottom left
            j = IMAGE_HEIGHT - j;
            write_color(img, i as u32, j as u32, &color, SAMPLES_PER_PIXEL);
        }
    }
    pb.finish_print("done");
}

pub fn ray_color(ray: &Ray, world: &HittableList, depth: &mut u8) -> Vec3 {
    if *depth == 0 {
        return Vec3::zero();
    }

    if let Some((hit_record, material)) = world.hit(ray, T_MIN, T_MAX) {
        return match material.scatter(ray, &hit_record) {
            Some((attenuation, scattered_ray)) => {
                *depth -= 1;
                ray_color(&scattered_ray, world, depth) * attenuation
            }
            None => Vec3::zero(),
        };
    }

    sky_color(&ray)
}

fn sky_color(ray: &Ray) -> Vec3 {
    let unit_direction = ray.direction.unit_vector();
    // normalize the t to be a value between 0 and 1
    let t = 0.5 * (unit_direction.y + 1.0);
    // interpolate from sky start color to end color using the y
    SKY_END_COLOR * (1.0 - t) + SKY_START_COLOR * t
}
