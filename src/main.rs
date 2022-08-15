mod vec3;
mod ray;

use macroquad::prelude::*;
use vec3::{Colour, Point3, Vec3 as Vec3};

fn window_conf() -> Conf {
    Conf {
        window_title: "Raytracing In One Weekend".to_owned(),
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let w = screen_width() as usize;
    let h = screen_height() as usize;

    let mut image = Image::gen_image_color(w as u16, h as u16, BLACK);
    let texture = Texture2D::from_image(&image);

    
    loop {
        clear_background(WHITE);
        let w = image.width() as u32;
        let h = image.height() as u32;

        for y in 0..h {
            for x in 0..w {
               let colour: Colour = Vec3::new(
                    x as f64 / (w - 1) as f64,
                    y as f64 / (h - 1) as f64,
                    0.25);
                image.set_pixel(x, y, colour.to_color());
            }
        }

        texture.update(&image);

        draw_texture(texture, 0., 0., WHITE);

        next_frame().await
    }
}