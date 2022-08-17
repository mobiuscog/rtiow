use crate::ray::Ray;

mod ray;
mod vector3;

use macroquad::prelude::*;

use vector3::{Colour, Point3, Vector3};

pub async fn run(aspect_ratio: f64) {
    let image_width = screen_width() as u32;
    let image_height = screen_height() as u32;

    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.;

    let origin: Point3 = Default::default();
    let horizontal = Vector3::new(viewport_width, 0., 0.);
    let vertical = Vector3::new(0., viewport_height, 0.);
    let lower_left_corner =
        origin - horizontal / 2. - vertical / 2. - Vector3::new(0., 0., focal_length);

    let mut image = Image::gen_image_color(image_width as u16, image_height as u16, BLACK);
    let texture = Texture2D::from_image(&image);

    loop {
        clear_background(WHITE);
        for y in (0..=image_height - 1).rev() {
            for x in 0..image_width {
                let u = x as f64 / (image_width - 1) as f64;
                let v = y as f64 / (image_height - 1) as f64;
                let r = Ray::new(
                    origin,
                    lower_left_corner + horizontal * u + vertical * v - origin,
                );
                let colour = r.colour();
                image.set_pixel(x, y, colour.to_color());
            }
        }

        texture.update(&image);

        draw_texture_ex(texture, 0., 0., WHITE, flip_texture_params());

        next_frame().await
    }
}

fn flip_texture_params() -> DrawTextureParams {
    DrawTextureParams {
        dest_size: None,
        source: None,
        rotation: 0.0,
        flip_x: false,
        flip_y: true,
        pivot: None,
    }
}
