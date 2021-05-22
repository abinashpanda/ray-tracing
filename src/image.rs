use crate::vec_three::Vec3;
use image::{Rgb, RgbImage};

pub fn write_color(img: &mut RgbImage, x: u32, y: u32, color: &Vec3, samples_per_pixel: u8) -> () {
    let scale = 1.0 / (samples_per_pixel as f64);
    let r = (scale * color.x).sqrt();
    let g = (scale * color.y).sqrt();
    let b = (scale * color.z).sqrt();

    let ir = (255.99 * r.clamp(0.0, 0.999)) as u8;
    let ig = (255.99 * g.clamp(0.0, 0.999)) as u8;
    let ib = (255.99 * b.clamp(0.0, 0.999)) as u8;
    img.put_pixel(x, y, Rgb([ir, ig, ib]));
}
