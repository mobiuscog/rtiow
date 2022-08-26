mod canvas;
mod ray;
mod sphere;
mod vector3;

#[macro_use]
extern crate auto_ops;

use macroquad::prelude::*;

use canvas::Canvas;
use ray::Ray;
use sphere::Sphere;
use vector3::{Colour, Point3, Vector3};

pub async fn run(aspect_ratio: f64) {
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.;

    let origin: Point3 = Default::default();
    let horizontal = Vector3::new(viewport_width, 0, 0);
    let vertical = Vector3::new(0, viewport_height, 0);
    let lower_left_corner =
        origin - horizontal / 2 - vertical / 2 - Vector3::new(0, 0, focal_length);

    let mut canvas = Canvas::new();

    let mut world: Vec<Box<dyn Hittable>> = vec![];
    world.push(Box::new(Sphere::new(Point3::new(0, 0, -1), 0.5)));
    world.push(Box::new(Sphere::new(Point3::new(0, -100.5, -1), 100)));
    // world.add(make_shared<sphere>(point3(0,-100.5,-1), 100));

    loop {
        clear_background(WHITE);
        for y in (0..=canvas.height() - 1).rev() {
            for x in 0..canvas.width() {
                let u = x as f64 / (canvas.width() - 1) as f64;
                let v = y as f64 / (canvas.height() - 1) as f64;
                let r = Ray::new(
                    origin,
                    lower_left_corner + horizontal * u + vertical * v - origin,
                );
                let colour = r.colour(&world);
                canvas.set_pixel(x, y, colour);
            }
        }

        canvas.render();

        next_frame().await
    }
}

#[derive(Clone, Default)]
pub struct Hit {
    p: Point3,
    normal: Vector3,
    t: f64,
    front_face: bool,
}

impl Hit {
    fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vector3) {
        self.front_face = r.direction().dot(outward_normal) < 0.;
        if self.front_face {
            self.normal = outward_normal.to_owned();
        } else {
            self.normal = -outward_normal.to_owned();
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut Hit) -> bool;
}

impl Hittable for Vec<Box<dyn Hittable>> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut Hit) -> bool {
        let mut temp_rec: Hit = Default::default();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in self.iter() {
            if object.hit(ray, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec.clone();
            }
        }

        return hit_anything;
    }
}
