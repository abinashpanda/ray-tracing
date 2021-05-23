extern crate rand;

use rand::Rng;
use std::ops;

const S_MIN: f64 = 1e-8;

#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl ops::Sub<f64> for Vec3 {
    type Output = Vec3;

    fn sub(self, value: f64) -> Self::Output {
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

impl ops::Add<f64> for Vec3 {
    type Output = Vec3;

    fn add(self, value: f64) -> Self::Output {
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

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, value: f64) -> Self::Output {
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

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, value: f64) -> Self::Output {
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

    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn from(vec: &Vec3) -> Vec3 {
        Vec3 {
            x: vec.x,
            y: vec.y,
            z: vec.z,
        }
    }

    pub fn length_squared(self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn unit_vector(self) -> Vec3 {
        let unit_v = Vec3::from(&self);
        let length = self.length();
        unit_v / length
    }

    pub fn dot(vec1: &Vec3, vec2: &Vec3) -> f64 {
        vec1.x * vec2.x + vec1.y * vec2.y + vec1.z * vec2.z
    }

    pub fn cross(vec1: &Vec3, vec2: &Vec3) -> Vec3 {
        Vec3 {
            x: vec1.y * vec2.z - vec1.z * vec2.y,
            y: vec1.z * vec2.x - vec1.x * vec2.z,
            z: vec1.x * vec2.y - vec1.y * vec2.x,
        }
    }

    pub fn random_vec3(min: f64, max: f64) -> Vec3 {
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

    pub fn near_zero(&self) -> bool {
        self.x.abs() < S_MIN && self.y.abs() < S_MIN && self.z.abs() < S_MIN
    }

    pub fn reflect(vec1: &Vec3, normal: &Vec3) -> Vec3 {
        *vec1 - *normal * 2.0 * Vec3::dot(vec1, normal)
    }
}
