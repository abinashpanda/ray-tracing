pub mod image;
pub mod ray;
pub mod vec_three;

use ray::Ray;
use vec_three::Vec3;

const WHITE_COLOR: Vec3 = Vec3 {
    x: 1.0,
    y: 1.0,
    z: 1.0,
};
const SKY_BLUE_COLOR: Vec3 = Vec3 {
    x: 0.5,
    y: 0.7,
    z: 1.0,
};

pub fn ray_color(ray: &Ray) -> Vec3 {
    let sphere_position = Vec3::from((0.0, 0.0, -1.0));
    let sphere_radius = 0.5;

    let t = hit_sphere(&sphere_position, sphere_radius, ray);
    if t > 0.0 {
        let ray_position = ray.at(t);
        let normal = ray_position - sphere_position;
        let unit_normal = normal.unit_vector();
        return (unit_normal + 1.0) * 0.5;
    }

    sky_color(&ray)
}

fn sky_color(ray: &Ray) -> Vec3 {
    let unit_direction = ray.direction.unit_vector();
    // normalize the t to be a value between 0 and 1
    let t = 0.5 * (unit_direction.y + 1.0);
    // interpolate from sky_blue_color to white using the y
    WHITE_COLOR * (1.0 - t) + SKY_BLUE_COLOR * t
}

fn hit_sphere(center: &Vec3, radius: f64, ray: &Ray) -> f64 {
    // check for solution of this equation
    // t^2*(rd.rd) + 2t(rd.(ro - c)) + (ro - c).(ro - c) - r^2 = 0
    // where rd is the direction of ray, ro is the origin of ray
    // c is the center and r is the radius of sphere
    let origin_to_center = ray.origin - *center;
    let a = Vec3::dot(&ray.direction, &ray.direction);
    let half_b = Vec3::dot(&ray.direction, &origin_to_center);
    let c = Vec3::dot(&origin_to_center, &origin_to_center) - radius * radius;
    let discriminant = half_b * half_b - a * c;
    match discriminant > 0.0 {
        true => (-half_b - discriminant.sqrt()) / a,
        false => -1.0,
    }
}
