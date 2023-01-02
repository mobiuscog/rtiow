use crate::prelude::*;

use num_traits::NumCast;

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vector3,
    vertical: Vector3,
    u: Vector3,
    v: Vector3,
    lens_radius: f64,
}

impl Camera {
    pub fn new<T: NumCast>(
        look_from: Point3,
        look_at: Point3,
        up_vector: Vector3,
        v_fov: T,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Camera {
        let theta = v_fov.to_f64().unwrap_or_default().to_radians();
        let h = (theta / 2.).tan();

        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (look_from - look_at).unit_vector();
        let u = up_vector.cross(&w).unit_vector();
        let v = w.cross(&u);

        let origin: Point3 = look_from;
        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        let lower_left_corner = origin - horizontal / 2 - vertical / 2 - focus_dist * w;
        let lens_radius = aperture / 2.;

        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            lens_radius,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * Vector3::random_in_unit_disk();
        let offset = self.u * rd.x() + self.v * rd.y();

        Ray::new(
            self.origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
        )
    }
}
