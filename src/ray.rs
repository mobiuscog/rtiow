use crate::{Colour, Point3, Vector3};

#[derive(Clone, Copy, Debug, Default)]
pub struct Ray {
    origin: Point3,
    direction: Vector3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vector3) -> Self {
        Self { origin, direction }
    }

    pub fn origin(&self) -> Point3 {
        self.origin
    }

    pub fn direction(&self) -> Vector3 {
        self.direction
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + self.direction * t
    }

    pub fn colour(&self) -> Colour {
        if self.hit_sphere(&Point3::new(0., 0., -1.), 0.5) {
            return Colour::new(1., 0., 0.);
        }
        let unit_direction = self.direction().unit_vector();
        let t = (unit_direction.y() + 1.) * 0.5;
        (1. - t) * Colour::new(1., 1., 1.) + t * Colour::new(0.5, 0.7, 1.)
    }

    fn hit_sphere(self, center: &Point3, radius: f64) -> bool {
        let oc = self.origin() - center;
        let a = self.direction().dot(self.direction());
        let b = 2. * oc.dot(self.direction());
        let c = oc.dot(oc) - radius * radius;
        let discriminant = b * b - 4. * a * c;
        discriminant > 0.
    }
}
