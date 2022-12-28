use crate::{Colour, Hit, Hittable, Point3, Vector3};

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

    pub fn colour(&self, world: &dyn Hittable, depth: u8) -> Colour {
        if depth <= 0 {
            return Colour::default();
        }

        let mut rec: Hit = Default::default();

        if world.hit(self, 0.0001, f64::INFINITY, &mut rec) {
            let target = rec.p + rec.normal + Vector3::random_in_unit_sphere().unit_vector();
            return 0.5 * Ray::new(rec.p, target - rec.p).colour(world, depth - 1);
        }

        let unit_direction = self.direction().unit_vector();
        let t = 0.5 * (unit_direction.y() + 1.0);
        (1. - t) * Colour::new(1, 1, 1) + t * Colour::new(0.5, 0.7, 1)
    }
}
