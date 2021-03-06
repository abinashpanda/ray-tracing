extern crate rand;

use rand::Rng;
use std::ops;

const S_MIN: f32 = 1e-8;

#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl ops::Sub<f32> for Vec3 {
    type Output = Vec3;

    fn sub(self, value: f32) -> Self::Output {
        Vec3 {
            x: self.x - value,
            y: self.y - value,
            z: self.z - value,
        }
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, vec2: Vec3) -> Self::Output {
        Vec3 {
            x: self.x - vec2.x,
            y: self.y - vec2.y,
            z: self.z - vec2.z,
        }
    }
}

impl ops::Add<f32> for Vec3 {
    type Output = Vec3;

    fn add(self, value: f32) -> Self::Output {
        Vec3 {
            x: self.x + value,
            y: self.y + value,
            z: self.z + value,
        }
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, vec2: Vec3) -> Self::Output {
        Vec3 {
            x: self.x + vec2.x,
            y: self.y + vec2.y,
            z: self.z + vec2.z,
        }
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, value: f32) -> Self::Output {
        Vec3 {
            x: self.x * value,
            y: self.y * value,
            z: self.z * value,
        }
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, vec2: Vec3) -> Self::Output {
        Vec3 {
            x: self.x * vec2.x,
            y: self.y * vec2.y,
            z: self.z * vec2.z,
        }
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, value: f32) -> Self::Output {
        Vec3 {
            x: self.x / value,
            y: self.y / value,
            z: self.z / value,
        }
    }
}

impl Vec3 {
    pub fn identity() -> Vec3 {
        Vec3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        }
    }

    pub fn zero() -> Vec3 {
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn from(vec: &Vec3) -> Vec3 {
        Vec3 {
            x: vec.x,
            y: vec.y,
            z: vec.z,
        }
    }

    pub fn length_squared(self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn unit_vector(self) -> Vec3 {
        let unit_v = Vec3::from(&self);
        let length = self.length();
        unit_v / length
    }

    pub fn get(&self, i: u8) -> f32 {
        match i {
            0 => self.x,
            1 => self.y,
            2 => self.z,
            _ => panic!("Vec3 doesn't have {} index", i),
        }
    }

    pub fn dot(vec1: &Vec3, vec2: &Vec3) -> f32 {
        vec1.x * vec2.x + vec1.y * vec2.y + vec1.z * vec2.z
    }

    pub fn cross(vec1: &Vec3, vec2: &Vec3) -> Vec3 {
        Vec3 {
            x: vec1.y * vec2.z - vec1.z * vec2.y,
            y: vec1.z * vec2.x - vec1.x * vec2.z,
            z: vec1.x * vec2.y - vec1.y * vec2.x,
        }
    }

    pub fn random_vec3(min: f32, max: f32) -> Vec3 {
        let mut rng = rand::thread_rng();
        Vec3::new(
            rng.gen_range(min..=max),
            rng.gen_range(min..=max),
            rng.gen_range(min..=max),
        )
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        let mut point: Vec3;
        loop {
            point = Vec3::random_vec3(-1.0, 1.0);
            if point.length_squared() < 1.0 {
                break;
            }
        }
        point
    }

    pub fn random_unit_vector() -> Vec3 {
        Vec3::random_in_unit_sphere().unit_vector()
    }

    pub fn random_in_unit_disk() -> Vec3 {
        let mut rng = rand::thread_rng();
        loop {
            let p = Vec3 {
                x: rng.gen_range(-1.0..=1.0),
                y: rng.gen_range(-1.0..=1.0),
                z: 0.0,
            };
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    pub fn near_zero(&self) -> bool {
        self.x.abs() < S_MIN && self.y.abs() < S_MIN && self.z.abs() < S_MIN
    }

    pub fn reflect(vec1: &Vec3, normal: &Vec3) -> Vec3 {
        *vec1 - *normal * 2.0 * Vec3::dot(vec1, normal)
    }

    pub fn refract(unit_vector: &Vec3, normal: &Vec3, refraction_ratio: f32) -> Vec3 {
        let uv = *unit_vector;
        let n = *normal;
        let cos_theta = (Vec3::dot(unit_vector, normal) * -1.0).min(1.0);
        let r_perp = (uv + n * cos_theta) * refraction_ratio;
        let r_parallel = n * (-(1.0 - r_perp.length_squared()).abs().sqrt());
        r_perp + r_parallel
    }
}
