extern crate image;

use image::{ImageBuffer, RgbImage};
use ray_tracing::{
    camera::Camera,
    hittable::HittableList,
    material::{LambertMaterial, MetalMaterial},
    ray_trace,
    sphere::Sphere,
    vec_three::Vec3,
    IMAGE_ASPECT_RATIO, IMAGE_HEIGHT, IMAGE_WIDTH,
};

fn main() {
    let mut img: RgbImage = ImageBuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    let origin = Vec3::zero();
    let camera = Camera::new(IMAGE_ASPECT_RATIO, 2.0, 1.0, &origin);

    let material_ground = LambertMaterial {
        color: Vec3::new(0.8, 0.8, 0.0),
    };
    let material_center = LambertMaterial {
        color: Vec3::new(0.7, 0.3, 0.3),
    };
    let material_left = MetalMaterial {
        color: Vec3::new(0.8, 0.8, 0.8),
    };
    let material_right = MetalMaterial {
        color: Vec3::new(0.8, 0.6, 0.2),
    };

    let mut world = HittableList::new();
    world.add_object(Box::new(Sphere::new(
        (0.0, -100.5, -1.0),
        100.0,
        Box::new(material_ground),
    )));
    world.add_object(Box::new(Sphere::new(
        (0.0, 0.0, -1.0),
        0.5,
        Box::new(material_center),
    )));
    world.add_object(Box::new(Sphere::new(
        (-1.0, 0.0, -1.0),
        0.5,
        Box::new(material_left),
    )));
    world.add_object(Box::new(Sphere::new(
        (1.0, 0.0, -1.0),
        0.5,
        Box::new(material_right),
    )));

    ray_trace(&camera, &world, &mut img);

    img.save("output/ray_traced_image.png").unwrap();
}
