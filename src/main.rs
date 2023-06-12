use std::{env, f64::consts::PI, rc::Rc};

use rand::Rng;
use rt::{
    Camera, Color, Dialectric, HitList, Lambertian, Metal, Point3, Sphere, Vec3, COLOR_BLACK,
};

const DEBUG_ONE_RAY: bool = false;

fn main() {
    let mut max_depth: i32 = 50;
    let mut samples_per_pixel = 100.0;
    if env::var("FAST_MODE").is_ok() {
        max_depth = 3;
        samples_per_pixel = 40.0;
    }
    if env::var("DETAIL_MODE").is_ok() {
        max_depth = 100;
        samples_per_pixel = 200.0;
    }

    let mut rng = rand::thread_rng();

    // World
    let mut world = HitList::new();
    // ground
    world.add(Box::new(Sphere {
        center: Point3::new(0.0, -100.5, -1.0),
        radius: 100.0,
        mat_ptr: Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0))),
    }));
    // center
    world.add(Box::new(Sphere {
        center: Point3::new(0.0, 0.0, -1.0),
        radius: 0.5,
        mat_ptr: Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5))),
    }));
    // left (glass)
    world.add(Box::new(Sphere {
        center: Point3::new(-1.0, 0.0, -1.0),
        radius: 0.5,
        mat_ptr: Rc::new(Dialectric::new(1.5)),
    }));
    world.add(Box::new(Sphere {
        center: Point3::new(-1.0, 0.0, -1.0),
        radius: -0.4, // TODO: wat is a negative radius
        mat_ptr: Rc::new(Dialectric::new(1.5)),
    }));
    // right (metal)
    world.add(Box::new(Sphere {
        center: Point3::new(1.0, 0.0, -1.0),
        radius: 0.5,
        mat_ptr: Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0)),
    }));

    // let radius1 = (PI / 4.0).cos();
    // world.add(Box::new(Sphere {
    //     center: Point3::new(-radius1, 0.0, -1.0),
    //     radius: radius1,
    //     mat_ptr: Rc::new(Lambertian::new(Color::new(0.0, 0.0, 1.0))),
    // }));

    // world.add(Box::new(Sphere {
    //     center: Point3::new(radius1, 0.0, -1.0),
    //     radius: radius1,
    //     mat_ptr: Rc::new(Lambertian::new(Color::new(1.0, 0.0, 0.0))),
    // }));

    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_height: i64 = 256;
    let image_width: i64 = (image_height as f64 * aspect_ratio) as i64;

    let max_depth = max_depth;

    // Camera
    let camera = Camera::new(
        Point3::new(-2.0, 2.0, 1.0),
        Point3::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, 1.0, 0.0),
        90.0,
        aspect_ratio,
    );

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
            // if i == 256 && j == 128 {
            if !DEBUG_ONE_RAY || (i == 1 && j == 128) {
                // eprintln!("running.. i={i}, j={j}");
                let mut pixel_color = COLOR_BLACK;
                for sample_idx in 0..samples_per_pixel as i64 {
                    // eprintln!("\n**sample** .. sample_idx={sample_idx}");
                    let u = (i as f64 + rng.gen::<f64>()) / (image_width as f64 - 1.0); // how horizontal? (0 to 1)
                    let v = (j as f64 + rng.gen::<f64>()) / (image_height as f64 - 1.0); // how vertical? (0 to 1)
                    let ray = camera.get_ray(u, v);
                    pixel_color = pixel_color + ray.color(&mut world, max_depth);
                    // eprintln!("{:?}", pixel_color);
                }
                write_color(pixel_color, samples_per_pixel);
            }
        }
    }
    eprintln!("Done.");
}

fn write_color(color: Color, samples_per_pixel: f64) {
    let mut r = color.x();
    let mut g = color.y();
    let mut b = color.z();

    // Divide the color by the number of samples and gamma-correct for gamma=2.0.
    let scale = 1.0 / samples_per_pixel;
    r = f64::sqrt(scale * r);
    g = f64::sqrt(scale * g);
    b = f64::sqrt(scale * b);

    let ir = (256.0 * rt::clamp(r, 0.0, 0.999)) as i64;
    let ig = (256.0 * rt::clamp(g, 0.0, 0.999)) as i64;
    let ib = (256.0 * rt::clamp(b, 0.0, 0.999)) as i64;

    println!("{ir} {ig} {ib}");
}
