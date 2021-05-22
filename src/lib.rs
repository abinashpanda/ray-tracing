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
const RED_COLOR: Vec3 = Vec3 {
    x: 1.0,
    y: 0.0,
    z: 0.0,
};

pub fn ray_color(ray: &Ray) -> Vec3 {
    let sphere_position = Vec3::from((0.0, 0.0, -1.0));
    let sphere_radius = 0.5;

    match hit_sphere(&sphere_position, sphere_radius, ray) {
        true => RED_COLOR,
        false => {
            let unit_direction = ray.direction.unit_vector();

            // normalize the t to be a value between 0 and 1
            let t = 0.5 * (unit_direction.y + 1.0);

            // interpolate from sky_blue_color to white using the y
            WHITE_COLOR * (1.0 - t) + SKY_BLUE_COLOR * t
        }
    }
}

fn hit_sphere(center: &Vec3, radius: f64, ray: &Ray) -> bool {
    // check for solution of this equation
    // t^2*(rd.rd) + 2t(rd.(ro - c)) + (ro - c).(ro - c) - r^2 = 0
    // where rd is the direction of ray, ro is the origin of ray
    // c is the center and r is the radius of sphere
    let origin_to_center = ray.origin - *center;
    let a = Vec3::dot(&ray.direction, &ray.direction);
    let b = 2.0 * Vec3::dot(&ray.direction, &origin_to_center);
    let c = Vec3::dot(&origin_to_center, &origin_to_center) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    discriminant > 0.0
}
