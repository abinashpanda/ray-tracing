extern crate image;

use image::{ImageBuffer, RgbImage};
use ray_tracing::{image::write_color, vec_three::Vec3};

const IMAGE_WIDTH: u32 = 512;
const IMAGE_HEIGHT: u32 = 512;

fn main() {
    let mut img: RgbImage = ImageBuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    for i in 1..IMAGE_WIDTH {
        for j in 1..IMAGE_HEIGHT {
            let r = (i as f64) / ((IMAGE_WIDTH as f64) - 1.0);
            let g = (j as f64) / ((IMAGE_HEIGHT as f64) - 1.0);
            let b = 0.25 as f64;
            let color = Vec3::from((r, g, b));

            // subtract IMAGE_HEIGHT - j as the we want to move the origin from top left to bottom left
            write_color(&mut img, i, IMAGE_HEIGHT - j, &color);
        }
    }

    img.save("output/ray_traced_image.png").unwrap();
}
