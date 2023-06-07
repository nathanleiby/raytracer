use std::ops::{Add, Sub};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn len(self) -> f64 {
        f64::sqrt(self.x * self.x + self.y * self.y + self.z * self.z)
    }

    pub fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    pub fn selfadd(self) -> Vec3 {
        let clone = self.clone();
        self.add(clone)
    }

    pub fn mul(self, t: f64) -> Vec3 {
        Vec3 {
            x: self.x * t,
            y: self.y * t,
            z: self.z * t,
        }
    }

    pub fn div(self, t: f64) -> Vec3 {
        self.mul(1.0 / t)
    }

    pub fn negate(self) -> Vec3 {
        self.mul(-1.0)
    }

    pub fn dot(self, other: Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn unit_vector(self) -> Vec3 {
        self.div(self.len())
    }
}

// Operator Overloading via Traits
impl Add for Vec3 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

// impl Mul for Vec3 {
//     type Output = Self;
//     fn mul(self, t: f64) -> Self {
//         Self {
//             x: self.x * t,
//             y: self.y * t,
//             z: self.z * t,
//         }
//     }
// }

pub type Point3 = Vec3;
pub type Color = Vec3;
