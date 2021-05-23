extern crate image;

use std::time::Instant;

use image::{ImageBuffer, RgbImage};
use ray_tracing::{
    camera::Camera,
    hittable::HittableList,
    material::{DielectricMaterial, LambertMaterial, MetalMaterial},
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
        color: Vec3::new(0.4, 0.4, 0.4),
    };
    let material_left = LambertMaterial {
        color: Vec3::new(0.1, 0.1, 0.8),
    };
    let material_center = DielectricMaterial {
        color: Vec3::new(0.6, 0.0, 0.2),
        refraction_index: 1.1,
    };
    let material_right = MetalMaterial {
        color: Vec3::new(0.8, 0.6, 0.2),
        fuzz: 0.0,
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
        (-0.75, 0.0, -1.5),
        // (-1.0, 0.0, -1.0),
        0.5,
        Box::new(material_left),
    )));
    world.add_object(Box::new(Sphere::new(
        (0.75, 0.0, -1.5),
        // (1.0, 0.0, -1.0),
        0.5,
        Box::new(material_right),
    )));

    let now = Instant::now();
    ray_trace(&camera, &world, &mut img);
    println!("ray tracing took {:.2?}", now.elapsed());

    img.save("output/ray_traced_image.png").unwrap();
}
