use std::sync::{Arc, Mutex};
use std::thread;

mod camera;
mod canvas;
mod material;
mod ray;
mod sphere;
mod vector3;

use crate::camera::Camera;
use crate::canvas::Canvas;
use crate::material::{dielectric::Dielectric, lambertian::Lambertian, metal::Metal};
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vector3::{Colour, Point3, Vector3};

#[macro_use]
extern crate auto_ops;

use ::rand::prelude::*;
use macroquad::prelude::*;
use num_cpus::get_physical;

pub trait Boxable {
    fn to_boxed(self) -> Box<Self>
    where
        Self: Sized,
    {
        Box::new(self)
    }
}

pub async fn run(aspect_ratio: f64) {
    let canvas = Canvas::new();
    let canvas_width = canvas.width();
    let canvas_height = canvas.height();
    let canvas_ref = Arc::new(Mutex::new(canvas));

    let mut world: Vec<Box<dyn Hittable>> = vec![];

    let material_ground = Arc::new(Lambertian::new(Colour::new(0.8, 0.8, 0.)));
    let material_center = Arc::new(Lambertian::new(Colour::new(0.1, 0.2, 0.5)));
    let material_left = Arc::new(Dielectric::new(1.5));
    let material_right = Arc::new(Metal::new(Colour::new(0.8, 0.6, 0.2)));

    world.push(Sphere::new(Point3::new(0, -100.5, -1), 100, material_ground.clone()).to_boxed());
    world.push(Sphere::new(Point3::new(0, 0, -1), 0.5, material_center.clone()).to_boxed());
    world.push(Sphere::new(Point3::new(-1, 0, -1), 0.5, material_left.clone()).to_boxed());
    world.push(Sphere::new(Point3::new(-1, 0, -1), -0.4, material_left.clone()).to_boxed());
    world.push(Sphere::new(Point3::new(1, 0, -1), 0.5, material_right.clone()).to_boxed());

    let world_ref = Arc::new(world);

    let camera = Camera::new(aspect_ratio);
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
            let max_depth = 10u8;

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

#[derive(Clone)]
pub struct Hit {
    p: Point3,
    normal: Vector3,
    material: Option<Arc<dyn material::Material>>,
    t: f64,
    front_face: bool,
}

impl Default for Hit {
    fn default() -> Self {
        Self {
            p: Point3::default(),
            normal: Vector3::default(),
            material: None,
            t: 0.,
            front_face: true,
        }
    }
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

pub trait Hittable: Send + Sync {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut Hit) -> bool;
}

impl Hittable for &Vec<Box<dyn Hittable>> {
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
