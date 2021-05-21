use crate::vec_three::Vec3;
use image::{Rgb, RgbImage};

pub fn write_color(img: &mut RgbImage, x: u32, y: u32, color: &Vec3) -> () {
    let ir = (255.99 * color.x) as u8;
    let ig = (255.99 * color.y) as u8;
    let ib = (255.99 * color.z) as u8;
    img.put_pixel(x, y, Rgb([ir, ig, ib]));
}
