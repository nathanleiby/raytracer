use std::sync::Arc;

use rt::material::{Dialectric, Lambertian, Material, Metal};
use rt::util::{random_bounded, random_double};
use rt::vec3::{Color, Point3};
use rt::{HitList, Sphere};

pub fn simple_scene() -> HitList {
    // World
    let mut world = HitList::new();
    // ground
    world.add(Box::new(Sphere {
        center: Point3::new(0.0, -100.5, -1.0),
        radius: 100.0,
        mat_ptr: Arc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0))),
    }));
    // center
    world.add(Box::new(Sphere {
        center: Point3::new(0.0, 0.0, -1.0),
        radius: 0.5,
        mat_ptr: Arc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5))),
    }));
    // left (glass)
    world.add(Box::new(Sphere {
        center: Point3::new(-1.0, 0.0, -1.0),
        radius: 0.5,
        mat_ptr: Arc::new(Dialectric::new(1.5)),
    }));
    world.add(Box::new(Sphere {
        center: Point3::new(-1.0, 0.0, -1.0),
        radius: -0.4, // TODO: wat is a negative radius
        mat_ptr: Arc::new(Dialectric::new(1.5)),
    }));
    // right (metal)
    world.add(Box::new(Sphere {
        center: Point3::new(1.0, 0.0, -1.0),
        radius: 0.5,
        mat_ptr: Arc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.05)),
    }));

    // let radius1 = (PI / 4.0).cos();
    // world.add(Box::new(Sphere {
    //     center: Point3::new(-radius1, 0.0, -1.0),
    //     radius: radius1,
    //     mat_ptr: Arc::new(Lambertian::new(Color::new(0.0, 0.0, 1.0))),
    // }));

    // world.add(Box::new(Sphere {
    //     center: Point3::new(radius1, 0.0, -1.0),
    //     radius: radius1,
    //     mat_ptr: Arc::new(Lambertian::new(Color::new(1.0, 0.0, 0.0))),
    // }));

    world
}

pub fn random_scene() -> HitList {
    let mut world = HitList::new();

    let ground_material: Arc<dyn Material> = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Box::new(Sphere {
        center: Point3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        mat_ptr: Arc::clone(&ground_material),
    }));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_double();
            let center = Point3::new(
                a as f64 + 0.9 * random_double(),
                0.2,
                b as f64 + 0.9 * random_double(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Arc<dyn Material>;

                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::new_random() * Color::new_random();
                    sphere_material = Arc::new(Lambertian::new(albedo));
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::new_random_bounded(0.5, 1.0);
                    let fuzz = random_bounded(0.0, 0.5);
                    sphere_material = Arc::new(Metal::new(albedo, fuzz));
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    // glass
                    sphere_material = Arc::new(Dialectric::new(1.5));
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material1 = Arc::new(Dialectric::new(1.5));
    let center1 = Point3::new(0.0, 1.0, 0.0);
    world.add(Box::new(Sphere::new(center1, 1.0, material1)));

    let material2 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    let center2 = Point3::new(-4.0, 1.0, 0.0);
    world.add(Box::new(Sphere::new(center2, 1.0, material2)));

    let material3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    let center3 = Point3::new(4.0, 1.0, 0.0);
    world.add(Box::new(Sphere::new(center3, 1.0, material3)));

    world
}
