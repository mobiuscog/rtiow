mod canvas;
mod ray;
mod vector3;

#[macro_use]
extern crate auto_ops;

use macroquad::prelude::*;

use canvas::Canvas;
use ray::Ray;
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
                let colour = r.colour();
                canvas.set_pixel(x, y, colour);
            }
        }

        canvas.render();

        next_frame().await
    }
}
