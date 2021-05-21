use std::ops;
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
    pub fn from(tup: (f64, f64, f64)) -> Vec3 {
        Vec3 {
            x: tup.0,
            y: tup.1,
            z: tup.2,
        }
    }

    pub fn from_vec(vec: &Vec3) -> Vec3 {
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
        let unit_v = Vec3::from_vec(&self);
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
}
