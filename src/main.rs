use std::ops::{Add, Mul, Sub};

const IMAGE_HEIGHT: i64 = 256;
const IMAGE_WIDTH: i64 = 256;

// TODO: move to a lib
#[derive(Debug, Copy, Clone, PartialEq)]
struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }
    fn len(self) -> f64 {
        f64::sqrt(self.x * self.x + self.y * self.y + self.z * self.z)
    }

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    fn selfadd(self) -> Vec3 {
        let clone = self.clone();
        self.add(clone)
    }

    fn mult(self, t: f64) -> Vec3 {
        Vec3 {
            x: self.x * t,
            y: self.y * t,
            z: self.z * t,
        }
    }

    fn negate(self) -> Vec3 {
        self.mult(-1.0)
    }

    fn div(self, t: f64) -> Vec3 {
        Vec3 {
            x: self.x / t,
            y: self.y / t,
            z: self.z / t,
        }
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

// type Point3 = Vec3; // TODO
type Color = Vec3;

fn main() {
    // Render

    // colors are in ascii
    println!("P3");

    // columns, rows
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);

    // max color
    println!("255");

    // RGB triplets
    for j in (0..IMAGE_HEIGHT).rev() {
        eprintln!("Lines remaining: {j}...");
        for i in 0..IMAGE_WIDTH {
            let r: f64 = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let g: f64 = j as f64 / (IMAGE_WIDTH - 1) as f64;
            let b = 0.25;
            write_color(Color::new(r, g, b))
        }
    }
    eprintln!("Done.");
}

fn write_color(color: Color) {
    let ir = (255.999 * color.x) as i64;
    let ig = (255.999 * color.y) as i64;
    let ib = (255.999 * color.z) as i64;

    println!("{ir} {ig} {ib}");
}
