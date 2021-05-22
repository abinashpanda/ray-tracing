extern crate image;
extern crate rand;

use image::{ImageBuffer, RgbImage};
use rand::Rng;
use ray_tracing::{
    camera::Camera, hittable::HittableList, image::write_color, ray_color, sphere::Sphere,
    vec_three::Vec3,
};

const IMAGE_ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: u32 = 1024;
const IMAGE_HEIGHT: u32 = ((IMAGE_WIDTH as f64) / IMAGE_ASPECT_RATIO) as u32;
const SAMPLES_PER_PIXEL: u8 = 20;

fn main() {
    let mut rng = rand::thread_rng();

    let mut img: RgbImage = ImageBuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    let origin = Vec3::new(0.0, 0.0, 0.0);
    let camera = Camera::new(IMAGE_ASPECT_RATIO, 2.0, 1.0, &origin);

    let mut world = HittableList::new();
    world.add_object(Box::new(Sphere::new((0.0, 0.0, -1.0), 0.5)));
    world.add_object(Box::new(Sphere::new((0.0, -100.5, -1.0), 100.0)));

    for i in 1..IMAGE_WIDTH {
        for mut j in 1..IMAGE_HEIGHT {
            let mut color = Vec3::new(0.0, 0.0, 0.0);
            for _ in 0..SAMPLES_PER_PIXEL {
                let random_num: f64 = rng.gen();

                let u = (i as f64 + random_num) / ((IMAGE_WIDTH as f64) - 1.0);
                let v = (j as f64 + random_num) / ((IMAGE_HEIGHT as f64) - 1.0);

                let ray = camera.get_origin_ray(u, v);
                color = color + ray_color(&ray, &world);
            }

            // subtract IMAGE_HEIGHT - j as the we want to move the origin from top left to bottom left
            j = IMAGE_HEIGHT - j;
            write_color(&mut img, i, j, &color, SAMPLES_PER_PIXEL);
        }
    }

    img.save("output/ray_traced_image.png").unwrap();
}
