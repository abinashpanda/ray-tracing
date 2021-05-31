extern crate image as image_crate;
extern crate rand;

pub mod aabb;
pub mod camera;
pub mod geometry;
pub mod hit_record;
pub mod image;
pub mod material;
pub mod ray;
pub mod scene;
pub mod vec_three;

use crate::image::write_color;
use camera::Camera;
use image_crate::RgbImage;
use pbr::ProgressBar;
use rand::Rng;
use ray::Ray;
use scene::Scene;
use vec_three::Vec3;

pub const IMAGE_ASPECT_RATIO: f32 = 3.0 / 2.0;
pub const IMAGE_WIDTH: u32 = 800;
pub const IMAGE_HEIGHT: u32 = ((IMAGE_WIDTH as f32) / IMAGE_ASPECT_RATIO) as u32;
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

const T_MIN: f32 = 0.001;
const T_MAX: f32 = f32::MAX;

pub fn ray_trace(camera: &Camera, world: &Scene, img: &mut RgbImage) {
    let mut rng = rand::thread_rng();
    let mut pb =
        ProgressBar::new((IMAGE_WIDTH * IMAGE_HEIGHT * SAMPLES_PER_PIXEL / PB_INCREMENT) as u64);
    let mut count = 0;

    let mut image_buffer = vec![vec![Vec3::zero(); IMAGE_HEIGHT as usize]; IMAGE_WIDTH as usize];

    for i in 0..IMAGE_WIDTH {
        for j in 0..IMAGE_HEIGHT {
            let mut color = Vec3::zero();
            for _ in 0..SAMPLES_PER_PIXEL {
                count += 1;

                let random_num: f32 = rng.gen();

                let u = (i as f32 + random_num) / ((IMAGE_WIDTH as f32) - 1.0);
                let v = (j as f32 + random_num) / ((IMAGE_HEIGHT as f32) - 1.0);

                // limit the number rays to be 10
                let mut depth: u8 = MAX_RAYS;
                let ray = camera.get_origin_ray(u, v);
                color = color + ray_color(&ray, &world, &mut depth);

                if count % PB_INCREMENT == 0 {
                    pb.inc();
                }
            }

            image_buffer[i as usize][j as usize] = color;
        }
    }
    pb.finish_print("done");

    for i in 0..IMAGE_WIDTH - 1 {
        for j in 0..IMAGE_HEIGHT - 1 {
            // subtract IMAGE_HEIGHT - j as the we want to move the origin from top left to bottom left
            let color = image_buffer[i as usize][j as usize];
            write_color(
                img,
                i as u32,
                IMAGE_HEIGHT - 1 - j as u32,
                &color,
                SAMPLES_PER_PIXEL,
            );
        }
    }
}

pub fn ray_color(ray: &Ray, world: &Scene, depth: &mut u8) -> Vec3 {
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
