use rt::{Color, HitList, Point3, Ray, Sphere, Vec3};

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_height: i64 = 256;
    let image_width: i64 = (image_height as f64 * aspect_ratio) as i64;

    // World
    let mut world = HitList::new();
    world.add(Box::new(Sphere {
        center: Point3::new(0.0, 0.0, -1.0),
        radius: 0.5,
    }));
    world.add(Box::new(Sphere {
        center: Point3::new(0.0, -100.5, -1.0),
        radius: 100.0,
    }));

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
            let ray = Ray::new(
                origin,
                lower_left_corner + horizontal.mul(u) + vertical.mul(v) - origin,
            );
            write_color(ray.color(&mut world));
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
