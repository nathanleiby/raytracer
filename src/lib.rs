use std::{
    cmp::Ordering,
    ops::{Add, Sub},
    rc::Rc,
};

use rand::{random, Rng};

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

    pub fn new_random() -> Vec3 {
        let mut rng = rand::thread_rng();

        Vec3 {
            x: rng.gen::<f64>(),
            y: rng.gen::<f64>(),
            z: rng.gen::<f64>(),
        }
    }

    pub fn new_random_bounded(min: f64, max: f64) -> Vec3 {
        let mut rng = rand::thread_rng();

        Vec3 {
            x: rng.gen_range(min..max),
            y: rng.gen_range(min..max),
            z: rng.gen_range(min..max),
        }
    }

    pub fn new_random_in_unit_sphere() -> Vec3 {
        loop {
            let v = Self::new_random();
            if v.dot(v) < 1.0 {
                return v;
            }
        }
    }

    pub fn new_random_in_hemisphere(normal: Vec3) -> Vec3 {
        let v = Self::new_random_in_unit_sphere();
        if dot(v, normal) > 0.0 {
            return v;
        }
        v.mul(-1.0)
    }

    pub fn new_random_unit_vector() -> Vec3 {
        Self::new_random_in_unit_sphere().unit_vector()
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

    pub fn mulvec3(self, v: Vec3) -> Vec3 {
        Vec3 {
            x: self.x * v.x,
            y: self.y * v.y,
            z: self.z * v.z,
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

    pub fn near_zero(self) -> bool {
        let s = 1e-8;
        f64::abs(self.x) < s && f64::abs(self.y) < s && f64::abs(self.z) < s
    }
}

pub fn dot(u: Vec3, v: Vec3) -> f64 {
    u.x * v.x + u.y * v.y + u.z * v.z
}
pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v.sub(n.mul(2.0 * dot(v, n)))
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

#[derive(Debug)]
pub struct Ray {
    orig: Point3,
    dir: Vec3,
}

impl Ray {
    pub fn new(orig: Point3, dir: Vec3) -> Ray {
        Ray { orig, dir }
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.orig + self.dir.mul(t)
    }

    pub fn color(self, world: &mut impl Hittable, depth: i32) -> Color {
        // If we've exceeded the ray bounce limit, no more light is gathered.
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        let unit_direction = self.dir.unit_vector();
        match world.hit(&self, 0.001, INF) {
            Some(rec) => {
                let out = rec.mat_ptr.scatter(&self, &rec);
                match out.scattered {
                    Some(scattered) => out
                        .attenuation
                        .unwrap()
                        .mulvec3(scattered.color(world, depth - 1)),
                    None => Color::new(0.0, 0.0, 0.0),
                }
            }
            None => {
                let t = 0.5 * (unit_direction.y + 1.0);
                Color::new(1.0, 1.0, 1.0).mul(1.0 - t) + Color::new(0.5, 0.7, 1.0).mul(t)
            }
        }
    }
}

pub struct HitRecord {
    p: Point3,
    normal: Vec3,
    t: f64,
    front_face: bool,
    mat_ptr: Rc<dyn Material>,
}

impl HitRecord {
    fn with_face_normal(self: Self, r: &Ray, outward_normal: Vec3) -> HitRecord {
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
            mat_ptr: Rc::clone(&self.mat_ptr),
        }
    }
}

pub struct ScatterResult {
    pub scattered: Option<Ray>,
    pub attenuation: Option<Color>,
}

pub trait Material {
    fn scatter(&self, r: &Ray, rec: &HitRecord) -> ScatterResult;
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r: &Ray, rec: &HitRecord) -> ScatterResult {
        let random_scatter_direction = rec.normal + Vec3::new_random_unit_vector();
        let scatter_direction = if random_scatter_direction.near_zero() {
            rec.normal
        } else {
            random_scatter_direction
        };

        ScatterResult {
            scattered: Some(Ray::new(rec.p, scatter_direction)),
            attenuation: Some(self.albedo),
        }
    }
}

pub struct Metal {
    albedo: Color,
}

impl Metal {
    pub fn new(albedo: Color) -> Metal {
        Metal { albedo }
    }
}

impl Material for Metal {
    fn scatter(&self, r: &Ray, rec: &HitRecord) -> ScatterResult {
        let reflected = reflect(r.dir.unit_vector(), rec.normal);

        let scattered = Ray::new(rec.p, reflected);
        ScatterResult {
            scattered: if dot(scattered.dir, rec.normal) > 0.0 {
                Some(scattered)
            } else {
                None
            },
            attenuation: Some(self.albedo),
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    pub mat_ptr: Rc<dyn Material>,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
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
            if root < t_min || t_max < root {
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
            mat_ptr: Rc::clone(&self.mat_ptr),
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
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        // let mut closest_so_far = None;
        let hits = self.objects.iter().map(|obj| obj.hit(ray, t_min, t_max));
        let closest = hits.into_iter().min_by(|x, y| match (x, y) {
            (None, None) => Ordering::Greater,
            (Some(_x), None) => Ordering::Less,
            (None, Some(_y)) => Ordering::Greater,
            (Some(x), Some(y)) => x.t.total_cmp(&y.t),
        });

        closest.unwrap()
    }
}

#[derive(Copy, Clone)]
pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(aspect_ratio: f64) -> Camera {
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let origin = Point3::new(0.0, 0.0, 0.0);
        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport_height, 0.0);
        let lower_left_corner =
            origin - horizontal.div(2.0) - vertical.div(2.0) - Vec3::new(0.0, 0.0, focal_length);

        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    pub fn get_ray(self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + self.horizontal.mul(u) + self.vertical.mul(v) - self.origin,
        )
    }
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x > max {
        return max;
    } else if x < min {
        return min;
    }
    x
}
