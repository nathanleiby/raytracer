use rt::{Color, Point3, Vec3};

pub struct Ray {
    orig: Point3,
    dir: Vec3,
}

impl Ray {
    pub fn new(orig: Point3, dir: Vec3) -> Ray {
        Ray { orig, dir }
    }

    pub fn at(self, t: f64) -> Point3 {
        self.orig.add(self.dir.mul(t))
    }

    pub fn color(self) -> Color {
        let unit_direction = self.dir.unit_vector();
        let t = 0.5 * (unit_direction.y + 1.0);
        Color::new(1.0, 1.0, 1.0).mul(1.0 - t) + Color::new(0.5, 0.7, 1.0).mul(t)
    }
}

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_height: i64 = 256;
    let image_width: i64 = (image_height as f64 * aspect_ratio) as i64;

    // Camera

    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal.div(2.0) - vertical.div(2.0) - Vec3::new(0.0, 0.0, focal_length);

    // Render

    // colors are in ascii
    println!("P3");

    // columns, rows
    println!("{} {}", image_width, image_height);

    // max color
    println!("255");

    // RGB triplets
    for j in (0..image_height).rev() {
        eprintln!("Lines remaining: {j}...");
        for i in 0..image_width {
            let u = i as f64 / (image_width as f64 - 1.0); // how horizontal? (0 to 1)
            let v = j as f64 / (image_height as f64 - 1.0); // how vertical? (0 to 1)
            let ray = Ray::new(origin, horizontal.mul(u) + vertical.mul(v) - origin);
            write_color(ray.color());
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
