use std::ops::{Add, Sub};

// Constants
const INF: f64 = f64::INFINITY;
const PI: f64 = 3.14159265358979323846264338327950288f64;

// Utility
fn degrees_to_radians(degrees: f64) -> f64 {
    degrees / 180.0 * PI
}

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

    pub fn color(self, world: &mut impl Hittable) -> Color {
        match world.hit(self, 0.0, INF) {
            Some(hit_record) => (hit_record.normal + Color::new(1.0, 1.0, 1.0)).mul(0.5),
            None => {
                let unit_direction = self.dir.unit_vector();
                let t = 0.5 * (unit_direction.y + 1.0);
                Color::new(1.0, 1.0, 1.0).mul(1.0 - t) + Color::new(0.5, 0.7, 1.0).mul(t)
            }
        }
    }
}

#[derive(Copy, Clone)]
pub struct HitRecord {
    p: Point3,
    normal: Vec3,
    t: f64,
    front_face: bool,
}

impl HitRecord {
    fn with_face_normal(self: Self, r: Ray, outward_normal: Vec3) -> HitRecord {
        let front_face = dot(r.dir, outward_normal) < 0.0;
        let normal = if self.front_face {
            outward_normal
        } else {
            outward_normal
        };

        HitRecord {
            p: self.p,
            normal,
            t: self.t,
            front_face,
        }
    }
}

pub trait Hittable {
    fn hit(&mut self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
}

impl Hittable for Sphere {
    fn hit(&mut self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.orig - self.center;
        let a = dot(ray.dir, ray.dir);
        let half_b = dot(oc, ray.dir);
        let c = dot(oc, oc) - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = f64::sqrt(discriminant);

        // try first root.. does it fall in time range?
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            // try 2nd root
            root = (-half_b + sqrtd) / a;
            if (root < t_min || t_max < root) {
                return None;
            }
        }

        let t = root;
        let p = ray.at(t);
        let hr = HitRecord {
            t,
            p,
            normal: (p - self.center).div(self.radius),
            front_face: false,
        };
        let outward_normal = (p - self.center).div(self.radius);

        Some(HitRecord::with_face_normal(hr, ray, outward_normal))
    }
}

pub struct HitList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HitList {
    pub fn new() -> HitList {
        HitList {
            objects: Vec::new(),
        }
    }
    pub fn clear(mut self: Self) {
        self.objects.clear();
    }
    pub fn add(self: &mut Self, obj: Box<dyn Hittable>) {
        self.objects.push(obj);
    }
}

impl Hittable for HitList {
    fn hit(&mut self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest_so_far = None;
        for obj in self.objects.as_mut_slice() {
            match obj.hit(ray, t_min, t_max) {
                None => (),
                Some(current) => match closest_so_far {
                    None => closest_so_far = Some(current),
                    Some(prev) => {
                        if current.t < prev.t {
                            closest_so_far = Some(current)
                        }
                    }
                },
            }
        }

        closest_so_far
    }
}
