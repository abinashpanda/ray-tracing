extern crate image;

use std::fs::create_dir;
use std::path::Path;
use std::time::Instant;

use image::{ImageBuffer, RgbImage};
use rand::Rng;
use ray_tracing::{
    camera::Camera,
    hittable::HittableList,
    material::{DielectricMaterial, LambertMaterial, MetalMaterial},
    ray_trace,
    sphere::Sphere,
    vec_three::Vec3,
    IMAGE_ASPECT_RATIO, IMAGE_HEIGHT, IMAGE_WIDTH,
};

fn make_scene() -> HittableList {
    let mut rng = rand::thread_rng();

    let mut world = HittableList::new();

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f64 = rng.gen();
            let center = Vec3::new(
                a as f64 + 0.9 * rng.gen::<f64>(),
                0.2,
                b as f64 + 0.9 * rng.gen::<f64>(),
            );
            let scene_center = Vec3::new(4.0, 0.2, 0.0);

            if (center - scene_center).length() > 0.9 {
                if choose_mat < 0.8 {
                    let color = Vec3::random_vec3(0.0, 1.0) * Vec3::random_vec3(0.0, 1.0);
                    let material = LambertMaterial { color };
                    let sphere =
                        Sphere::new((center.x, center.y, center.z), 0.2, Box::new(material));
                    world.add_object(Box::new(sphere));
                }
            } else if choose_mat < 0.95 {
                let color = Vec3::random_vec3(0.5, 1.0);
                let fuzz: f64 = rng.gen_range(0.0..=0.5);
                let material = MetalMaterial { color, fuzz };
                let sphere = Sphere::new((center.x, center.y, center.z), 0.2, Box::new(material));
                world.add_object(Box::new(sphere));
            } else {
                let material = DielectricMaterial {
                    color: Vec3::identity(),
                    refraction_index: 1.3,
                };
                let sphere = Sphere::new((center.x, center.y, center.z), 0.2, Box::new(material));
                world.add_object(Box::new(sphere));
            }
        }
    }

    let material_1 = DielectricMaterial {
        color: Vec3::identity(),
        refraction_index: 1.3,
    };
    let sphere_1 = Sphere::new((0.0, 1.0, 0.0), 1.0, Box::new(material_1));
    world.add_object(Box::new(sphere_1));

    let material_2 = LambertMaterial {
        color: Vec3::new(0.4, 0.2, 0.1),
    };
    let sphere_2 = Sphere::new((-4.0, 1.0, 0.0), 1.0, Box::new(material_2));
    world.add_object(Box::new(sphere_2));

    let material_3 = MetalMaterial {
        color: Vec3::new(0.7, 0.6, 0.5),
        fuzz: 0.0,
    };
    let sphere_3 = Sphere::new((4.0, 1.0, 0.0), 1.0, Box::new(material_3));
    world.add_object(Box::new(sphere_3));

    let material_ground = LambertMaterial {
        color: Vec3::new(0.5, 0.5, 0.5),
    };
    let ground = Sphere::new((0.0, -1000.0, 0.0), 1000.0, Box::new(material_ground));
    world.add_object(Box::new(ground));

    world
}

fn main() {
    let mut img: RgbImage = ImageBuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    let look_from = Vec3::new(13.0, 2.0, 4.0);
    let look_at = Vec3::new(0.0, 0.0, 0.0);
    let camera = Camera::new(
        IMAGE_ASPECT_RATIO,
        20.0,
        &look_from,
        &look_at,
        &Vec3::new(0.0, 1.0, 0.0),
        0.1,
        10.0,
    );

    let world = make_scene();

    let now = Instant::now();
    ray_trace(&camera, &world, &mut img);
    println!("ray tracing took {:.2?}", now.elapsed());

    if !Path::new("output").exists() {
        create_dir("output").unwrap();
    }

    img.save("output/ray_traced_image.png").unwrap();
}
