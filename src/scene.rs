use std::sync::Arc;

use crate::material::{dielectric::Dielectric, lambertian::Lambertian, metal::Metal};
use crate::sphere::Sphere;
use crate::vector3::{Colour, Point3};
use crate::{Boxable, Hittable};

use rand::{prelude::StdRng, Rng, SeedableRng};

#[allow(dead_code)]
pub fn build_test() -> Vec<Box<dyn Hittable>> {
    let mut world: Vec<Box<dyn Hittable>> = vec![];

    let material_ground = Arc::new(Lambertian::new(Colour::new(0.8, 0.8, 0.)));
    let material_center = Arc::new(Lambertian::new(Colour::new(0.1, 0.2, 0.5)));
    let material_left = Arc::new(Dielectric::new(1.5));
    let material_right = Arc::new(Metal::new(Colour::new(0.8, 0.6, 0.2)));

    world.push(Sphere::new(Point3::new(0, -100.5, -1), 100, material_ground.clone()).to_boxed());
    world.push(Sphere::new(Point3::new(0, 0, -1), 0.5, material_center.clone()).to_boxed());
    world.push(Sphere::new(Point3::new(-1, 0, -1), 0.5, material_left.clone()).to_boxed());
    world.push(Sphere::new(Point3::new(-1, 0, -1), -0.45, material_left.clone()).to_boxed());
    world.push(Sphere::new(Point3::new(1, 0, -1), 0.5, material_right.clone()).to_boxed());

    world
}

pub fn build_cover() -> Vec<Box<dyn Hittable>> {
    let mut world: Vec<Box<dyn Hittable>> = vec![];

    let material_ground = Arc::new(Lambertian::new(Colour::new(0.5, 0.5, 0.5)));
    world.push(Sphere::new(Point3::new(0, -1000, 0), 1000, material_ground.clone()).to_boxed());

    let mut rng = StdRng::seed_from_u64(52);
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.gen::<f64>();
            let mut center = Point3::new(
                a as f64 + (0.9 * rng.gen::<f64>()),
                0.4,
                b as f64 + (0.9 * rng.gen::<f64>()),
            );

            if (center.subtract(4., 0.2, 0.)).length() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo =
                        Colour::random_from_rng(&mut rng) * Colour::random_from_rng(&mut rng);
                    let material = Arc::new(Lambertian::new(albedo));
                    world.push(Sphere::new(center, 0.2, material.clone()).to_boxed());
                } else if choose_mat < 0.95 {
                    let albedo = Colour::random_range_from_rng(&mut rng, 0.5, 1.);
                    let blur = rng.gen_range(0.0..0.5);
                    let material = Arc::new(Metal::new_blurred(albedo, blur));
                    world.push(Sphere::new(center, 0.2, material.clone()).to_boxed());
                } else {
                    let material = Arc::new(Dielectric::new(1.5));
                    world.push(Sphere::new(center, 0.2, material.clone()).to_boxed());
                }
            }
        }
    }

    let material1 = Arc::new(Dielectric::new(1.5));
    world.push(Sphere::new(Point3::new(0, 1, 0), 1.0, material1.clone()).to_boxed());

    let material2 = Arc::new(Lambertian::new(Colour::new(0.4, 0.2, 0.1)));
    world.push(Sphere::new(Point3::new(-4, 1, 0), 1.0, material2.clone()).to_boxed());

    let material3 = Arc::new(Metal::new(Colour::new(0.7, 0.6, 0.5)));
    world.push(Sphere::new(Point3::new(4, 1, 0), 1.0, material3.clone()).to_boxed());

    world
}
