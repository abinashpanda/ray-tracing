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

use std::{
    sync::{Arc, Mutex},
    thread::{self, JoinHandle},
};

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
const NUM_CHUNKS: u8 = 12;

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
    let pb = Arc::new(Mutex::new(ProgressBar::new(
        (IMAGE_WIDTH * IMAGE_HEIGHT * SAMPLES_PER_PIXEL / PB_INCREMENT) as u64,
    )));
    let count = Arc::new(Mutex::new(0));

    let chunk_size_width = IMAGE_WIDTH / (NUM_CHUNKS as u32);
    let chunk_size_height = IMAGE_HEIGHT / (NUM_CHUNKS as u32);

    let mut handles = vec![];

    let image_buffer = Arc::new(Mutex::new(vec![
        vec![Vec3::zero(); IMAGE_HEIGHT as usize];
        IMAGE_WIDTH as usize
    ]));
    let camera = Arc::new(Mutex::new(camera.clone()));
    let mut new_world = Scene::new();
    for object in world.objects.iter() {
        new_world.add_object(object.clone());
    }
    let new_world = Arc::new(Mutex::new(new_world));

    for chunk_x in 0..NUM_CHUNKS {
        for chunk_y in 0..NUM_CHUNKS {
            let camera = Arc::clone(&camera);
            let new_world = Arc::clone(&new_world);
            let image_buffer = Arc::clone(&image_buffer);

            let count = Arc::clone(&count);
            let pb = Arc::clone(&pb);

            let handle: JoinHandle<()> = thread::spawn(move || {
                let start_x = (chunk_size_width * (chunk_x as u32)).max(1);
                let end_x = (start_x + chunk_size_width).min(IMAGE_WIDTH);
                let start_y = (chunk_size_height * (chunk_y as u32)).max(1);
                let end_y = (start_y + chunk_size_height).min(IMAGE_HEIGHT);
                let mut rng = rand::thread_rng();

                let mut image_buffer = image_buffer.lock().unwrap();
                let mut count = count.lock().unwrap();
                let mut pb = pb.lock().unwrap();

                for i in start_x..end_x {
                    for j in start_y..end_y {
                        let mut color = Vec3::zero();

                        for _ in 0..SAMPLES_PER_PIXEL {
                            *count += 1;

                            let random_num: f32 = rng.gen();

                            let u = (i as f32 + random_num) / ((IMAGE_WIDTH as f32) - 1.0);
                            let v = (j as f32 + random_num) / ((IMAGE_HEIGHT as f32) - 1.0);

                            // limit the number rays to be 10
                            let mut depth: u8 = MAX_RAYS;
                            let camera = camera.lock().unwrap();
                            let new_world = new_world.lock().unwrap();
                            let ray = camera.get_origin_ray(u, v);
                            color = color + ray_color(&ray, &new_world, &mut depth);

                            if *count % PB_INCREMENT == 0 {
                                pb.inc();
                            }
                        }

                        image_buffer[i as usize][j as usize] = color;
                    }
                }
            });

            handles.push(handle);
        }
    }

    handles
        .into_iter()
        .for_each(|handle| handle.join().unwrap());

    for i in 0..IMAGE_WIDTH - 1 {
        for j in 0..IMAGE_HEIGHT - 1 {
            // subtract IMAGE_HEIGHT - j as the we want to move the origin from top left to bottom left
            let image_buffer = image_buffer.lock().unwrap();
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
