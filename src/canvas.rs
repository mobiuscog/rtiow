use crate::vector3::Colour;

use macroquad::prelude::*;

pub struct Canvas {
    width: u32,
    height: u32,
    image: Image,
    texture: Texture2D,
}

impl Default for Canvas {
    fn default() -> Self {
        Self::new(screen_width() as u32, screen_height() as u32)
    }
}

impl Canvas {
    pub fn new(width: u32, height: u32) -> Canvas {
        let image = Image::gen_image_color(width as u16, height as u16, BLACK);
        let texture = Texture2D::from_image(&image);

        Canvas {
            width,
            height,
            image,
            texture,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn render(&self) {
        self.texture.update(&self.image);
        draw_texture_ex(
            self.texture,
            0.,
            0.,
            WHITE,
            DrawTextureParams {
                flip_y: true,
                ..Default::default()
            },
        );
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, colour: Colour, samples_per_pixel: u32) {
        self.image
            .set_pixel(x, y, colour.to_color(samples_per_pixel));
    }
}
