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
        dot(self, other)
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

pub fn dot(u: Vec3, v: Vec3) -> f64 {
    u.x * v.x + u.y * v.y + u.z * v.z
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

pub type Point3 = Vec3;
pub type Color = Vec3;

const RED_COLOR: Color = Color {
    x: 1.0,
    y: 0.0,
    z: 0.0,
};

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    orig: Point3,
    dir: Vec3,
}

impl Ray {
    pub fn new(orig: Point3, dir: Vec3) -> Ray {
        Ray { orig, dir }
    }

    pub fn at(self, t: f64) -> Point3 {
        self.orig + self.dir.mul(t)
    }

    pub fn color(self) -> Color {
        if hit_sphere(Point3::new(0.0, 0.0, -1.0), 0.5, self.clone()) {
            return RED_COLOR;
        }

        let unit_direction = self.dir.unit_vector();
        let t = 0.5 * (unit_direction.y + 1.0);
        Color::new(1.0, 1.0, 1.0).mul(1.0 - t) + Color::new(0.5, 0.7, 1.0).mul(t)
    }
}

fn hit_sphere(center: Point3, radius: f64, ray: Ray) -> bool {
    let oc = ray.orig - center;
    let a = dot(ray.dir, ray.dir);
    // let b = 2.0 * dot(oc, ray.dir);
    let half_b = dot(oc, ray.dir);
    let c = dot(oc, oc) - radius * radius;
    // let discriminant = b * b - 4.0 * a * c;
    let discriminant = half_b * half_b - a * c;
    discriminant > 0.0
}
