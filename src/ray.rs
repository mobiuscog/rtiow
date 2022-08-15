use crate::{Vec3, Point3, Colour};

#[derive(Clone, Copy, Debug, Default)]
pub struct Ray {
    origin: Point3,
    direction: Vec3
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    pub fn origin(&self) -> Point3 {
        self.origin
    }

    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + self.direction * t
    }

    pub fn colour(&self) -> Colour {
        let unit_direction = self.direction().unit_vector();
        let t = 0.5 * (unit_direction.y() + 1.);
        Colour::new(1., 1., 1.) * (1. - t) + Colour::new(0.5, 0.7, 1.) * t
    }
}