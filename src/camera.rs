use crate::{Point3, Ray, Vector3};

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vector3,
    vertical: Vector3,
}

impl Camera {
    pub fn new(aspect_ratio: f64) -> Camera {
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.;

        let origin: Point3 = Default::default();
        let horizontal = Vector3::new(viewport_width, 0, 0);
        let vertical = Vector3::new(0, viewport_height, 0);
        let lower_left_corner =
            origin - horizontal / 2 - vertical / 2 - Vector3::new(0, 0, focal_length);

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
