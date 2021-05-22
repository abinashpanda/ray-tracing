extern crate image;

use image::{ImageBuffer, RgbImage};
use ray_tracing::{image::write_color, ray::Ray, ray_color, vec_three::Vec3};

const IMAGE_ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: u32 = 1024;
const IMAGE_HEIGHT: u32 = ((IMAGE_WIDTH as f64) / IMAGE_ASPECT_RATIO) as u32;

fn main() {
    let viewport_height = 2.0;
    let viewport_width = viewport_height * IMAGE_ASPECT_RATIO;
    let focal_length = 1.0;

    let origin = Vec3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - (horizontal / 2.0) - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    let mut img: RgbImage = ImageBuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    for i in 1..IMAGE_WIDTH {
        for j in 1..IMAGE_HEIGHT {
            let u = (i as f64) / ((IMAGE_WIDTH as f64) - 1.0);
            let v = (j as f64) / ((IMAGE_HEIGHT as f64) - 1.0);

            // as the camera is present in the origin, the direction vector would the position of the point
            let direction = lower_left_corner + horizontal * u + vertical * v;

            let ray = Ray { origin, direction };
            let color = ray_color(&ray);

            // subtract IMAGE_HEIGHT - j as the we want to move the origin from top left to bottom left
            write_color(&mut img, i, IMAGE_HEIGHT - j, &color);
        }
    }

    img.save("output/ray_traced_image.png").unwrap();
}
