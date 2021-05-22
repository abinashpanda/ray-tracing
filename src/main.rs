extern crate image;

use image::{ImageBuffer, RgbImage};
use ray_tracing::{
    camera::Camera, hittable::HittableList, ray_trace, sphere::Sphere, vec_three::Vec3,
    IMAGE_ASPECT_RATIO, IMAGE_HEIGHT, IMAGE_WIDTH,
};

fn main() {
    let mut img: RgbImage = ImageBuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    let origin = Vec3::new(0.0, 0.0, 0.0);
    let camera = Camera::new(IMAGE_ASPECT_RATIO, 2.0, 1.0, &origin);

    let mut world = HittableList::new();
    world.add_object(Box::new(Sphere::new((0.0, 0.0, -1.0), 0.5)));
    world.add_object(Box::new(Sphere::new((0.0, -100.5, -1.0), 100.0)));

    ray_trace(&camera, &world, &mut img);

    img.save("output/ray_traced_image.png").unwrap();
}
