use macroquad::prelude::*;

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

                let r = x as f64 / (w - 1) as f64;
                let g = y as f64 / (h - 1) as f64;
                let b = 0.25;

                let ir = (255.999 * r) as u8;
                let ig = (255.999 * g) as u8;
                let ib = (255.999 * b) as u8;
                image.set_pixel(x, y, Color::from_rgba(ir, ig, ib, 255));
            }
        }

        texture.update(&image);

        draw_texture(texture, 0., 0., WHITE);

        next_frame().await
    }
}