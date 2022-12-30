use crate::{Point3, Ray, Vector3};
use num_traits::NumCast;

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vector3,
    vertical: Vector3,
}

impl Camera {
    pub fn new<T: NumCast>(
        look_from: Point3,
        look_at: Point3,
        up_vector: Vector3,
        v_fov: T,
        aspect_ratio: f64,
    ) -> Camera {
        let theta = v_fov.to_f64().unwrap().to_radians();
        let h = (theta / 2.).tan();

        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (look_from - look_at).unit_vector();
        let u = up_vector.cross(&w).unit_vector();
        let v = w.cross(&u);

        let origin: Point3 = look_from;
        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;
        let lower_left_corner = origin - horizontal / 2 - vertical / 2 - w;

        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin,
        )
    }
}
