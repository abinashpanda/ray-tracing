extern crate image;

use std::fs::create_dir;
use std::path::Path;
use std::time::Instant;

use image::{ImageBuffer, RgbImage};
use rand::Rng;
use ray_tracing::{
    camera::Camera, geometry::Geometry, material::Material, ray_trace, scene::Scene,
    vec_three::Vec3, IMAGE_ASPECT_RATIO, IMAGE_HEIGHT, IMAGE_WIDTH,
};

fn make_scene() -> Scene {
    let mut rng = rand::thread_rng();

    let mut world = Scene::new();

    for a in -5..5 {
        for b in -5..5 {
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
                    let material = Material::Lambert { color };
                    let sphere = Geometry::Sphere {
                        center,
                        radius: 0.2,
                        material,
                    };
                    world.add_object(sphere);
                }
            } else if choose_mat < 0.95 {
                let color = Vec3::random_vec3(0.5, 1.0);
                let fuzz: f64 = rng.gen_range(0.0..=0.5);
                let material = Material::Metal { color, fuzz };
                let sphere = Geometry::Sphere {
                    center,
                    radius: 0.2,
                    material,
                };
                world.add_object(sphere);
            } else {
                let material = Material::Dielectric {
                    color: Vec3::identity(),
                    refraction_index: 1.3,
                };
                let sphere = Geometry::Sphere {
                    center,
                    radius: 0.2,
                    material,
                };
                world.add_object(sphere);
            }
        }
    }

    let material_1 = Material::Dielectric {
        color: Vec3::identity(),
        refraction_index: 1.3,
    };
    let sphere_1 = Geometry::Sphere {
        center: Vec3::new(0.0, 1.0, 0.0),
        radius: 1.0,
        material: material_1,
    };
    world.add_object(sphere_1);

    let material_2 = Material::Lambert {
        color: Vec3::new(0.4, 0.2, 0.1),
    };
    let sphere_2 = Geometry::Sphere {
        center: Vec3::new(-4.0, 1.0, 0.0),
        radius: 1.0,
        material: material_2,
    };
    world.add_object(sphere_2);

    let material_3 = Material::Metal {
        color: Vec3::new(0.7, 0.6, 0.5),
        fuzz: 0.0,
    };
    let sphere_3 = Geometry::Sphere {
        center: Vec3::new(4.0, 1.0, 0.0),
        radius: 1.0,
        material: material_3,
    };
    world.add_object(sphere_3);

    let material_ground = Material::Lambert {
        color: Vec3::new(0.5, 0.5, 0.5),
    };
    let ground = Geometry::Sphere {
        center: Vec3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: material_ground,
    };
    world.add_object(ground);

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
