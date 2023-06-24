use std::{cmp::Ordering, sync::Arc};

pub mod material;
pub mod util;
pub mod vec3;

use material::Material;
use util::degrees_to_radians;
use vec3::{dot, Color, Point3, Vec3, COLOR_BLACK, COLOR_WHITE};

// Constants
const INF: f64 = f64::INFINITY;

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
        self.orig + self.dir * t
    }

    pub fn color(self, world: &impl Hittable, depth: i32) -> Color {
        // If we've exceeded the ray bounce limit, no more light is gathered.
        if depth <= 0 {
            // eprintln!("depth <= 0");
            return COLOR_BLACK;
        }

        match world.hit(&self, 0.001, INF) {
            Some(rec) => {
                // eprintln!("hit? yes");
                let out = rec.mat_ptr.scatter(&self, &rec);
                match out {
                    Some(out) => {
                        // eprintln!("attenuation: {:?}", out.attenuation);
                        // eprintln!("scattered:{:?}", out.scattered);
                        out.attenuation * out.scattered.color(world, depth - 1)
                    }
                    None => COLOR_BLACK,
                }
            }
            None => {
                // eprintln!("hit? no");
                let unit_direction = self.dir.unit_vector();
                let t = 0.5 * (unit_direction.y() + 1.0);
                COLOR_WHITE * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
            }
        }
    }
}

pub struct HitRecord {
    p: Point3,
    normal: Vec3,
    t: f64,
    front_face: bool,
    mat_ptr: Arc<dyn Material>,
}

impl HitRecord {
    fn with_face_normal(self: Self, r: &Ray, outward_normal: Vec3) -> HitRecord {
        let (normal, front_face) = if dot(r.dir, outward_normal) > 0.0 {
            (-outward_normal, false)
        } else {
            (outward_normal, true)
        };

        HitRecord {
            p: self.p,
            normal,
            t: self.t,
            front_face,
            mat_ptr: Arc::clone(&self.mat_ptr),
        }
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    pub mat_ptr: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, mat_ptr: Arc<dyn Material>) -> Self {
        Sphere {
            center,
            radius,
            mat_ptr,
        }
    }
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
            normal: (p - self.center) / self.radius,
            front_face: false,
            mat_ptr: Arc::clone(&self.mat_ptr),
        };
        let outward_normal = (p - self.center) / self.radius;

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

// #[derive(Copy, Clone)]
pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f64,
}

impl Camera {
    pub fn new(
        lookfrom: Point3,
        lookat: Point3,
        vup: Vec3, // view up
        vfov: f64, // vertical field-of-view (degrees)
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Camera {
        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (lookfrom - lookat).unit_vector();
        let u = vup.cross(w).unit_vector();
        let v = w.cross(u);

        let origin = lookfrom;
        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        let lower_left_corner = origin - (horizontal / 2.0) - (vertical / 2.0) - focus_dist * w;

        let lens_radius = aperture / 2.0;

        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            w,
            lens_radius,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * Vec3::new_random_in_unit_disk();
        let offset = self.u * rd.x() + self.v * rd.y();
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + self.horizontal * s + self.vertical * t - self.origin - offset,
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
