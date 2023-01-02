extern crate rtiow;

use macroquad::prelude::*;

pub const ASPECT_RATIO: f64 = 3. / 2.;
const WINDOW_WIDTH: i32 = 1200;

fn window_conf() -> Conf {
    let window_height = (WINDOW_WIDTH as f64 / ASPECT_RATIO) as i32;
    Conf {
        window_title: "Raytracing in One Weekend".into(),
        window_resizable: false,
        window_width: WINDOW_WIDTH,
        window_height,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    rtiow::run(ASPECT_RATIO).await;
}
