use std::sync::{Arc, Mutex};
use std::thread;

use crate::camera::Camera;
use crate::canvas::Canvas;
use crate::scene::*;
use crate::vector3::{Colour, Point3, Vector3};

use ::rand::prelude::*;
use macroquad::prelude::*;
use num_cpus::get_physical;

pub async fn run(aspect_ratio: f64) {
    let canvas = Canvas::default();
    let canvas_width = canvas.width();
    let canvas_height = canvas.height();
    let canvas_ref = Arc::new(Mutex::new(canvas));
    let world_ref = Arc::new(build_cover());

    let look_from = Point3::new(13, 2, 3);
    let look_at = Point3::new(0, 0, 0);
    let up_vector = Vector3::new(0, 1, 0);
    let dist_to_focus = 10.;
    let aperture = 0.1;

    let camera = Camera::new(
        look_from,
        look_at,
        up_vector,
        20,
        aspect_ratio,
        aperture,
        dist_to_focus,
    );
    let camera_ref = Arc::new(camera);

    let cpu_cores = get_physical() as u32;
    let mut launched_threads = vec![];
    for thread_id in 0..cpu_cores {
        let canvas_local = canvas_ref.clone();
        let world_local = world_ref.clone();
        let camera_local = camera_ref.clone();

        let thread_handle = thread::spawn(move || {
            let mut rng = thread_rng();

            let samples_per_pixel = 20u32;
            let max_depth = 50u8;

            for y in (0..=canvas_height - thread_id - 1)
                .rev()
                .step_by(cpu_cores as usize)
            {
                for x in 0..canvas_width {
                    let mut pixel_colour = Colour::default();
                    for _ in 0..samples_per_pixel {
                        let u = (x as f64 + rng.gen::<f64>()) / (canvas_width - 1) as f64;
                        let v = (y as f64 + rng.gen::<f64>()) / (canvas_height - 1) as f64;
                        let ray = camera_local.get_ray(u, v);
                        pixel_colour += ray.colour(&world_local.as_ref(), max_depth);
                    }
                    canvas_local
                        .lock()
                        .unwrap()
                        .set_pixel(x, y, pixel_colour, samples_per_pixel);
                }
            }
        });
        launched_threads.push((thread_handle, true));
    }

    let font_size = 32;
    let rendering_text = "Tracing Rays...";
    let text_dimensions = measure_text(rendering_text, None, font_size, 1.0);
    let text_x = (screen_width() - text_dimensions.width) / 2.;
    let text_y = (screen_height() - text_dimensions.height) / 2.;

    let mut threads_running = true;
    loop {
        clear_background(WHITE);
        canvas_ref.lock().unwrap().render();
        if threads_running {
            draw_rectangle(
                text_x - 8.,
                text_y - text_dimensions.height / 2. - 16.,
                text_dimensions.width + 16.,
                text_dimensions.height + 16.,
                WHITE,
            );
            draw_text(rendering_text, text_x, text_y, font_size as f32, RED);
        }
        next_frame().await;
        if threads_running {
            let mut active_count = 0;
            for thread_handle in launched_threads.iter_mut() {
                if thread_handle.1 {
                    if thread_handle.0.is_finished() {
                        thread_handle.1 = false
                    } else {
                        active_count += 1;
                    }
                }
            }
            if active_count == 0 {
                threads_running = false;
            }
        }
    }
}
