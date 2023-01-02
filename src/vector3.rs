use ::rand::prelude::*;
use macroquad::prelude::Color;
use num_traits::NumCast;

#[derive(Clone, Copy, Debug)]
pub struct Vector3 {
    x: f64,
    y: f64,
    z: f64,
}

pub type Point3 = Vector3;
pub type Colour = Vector3;

impl Default for Vector3 {
    fn default() -> Self {
        Self {
            x: 0.,
            y: 0.,
            z: 0.,
        }
    }
}

impl Vector3 {
    pub fn new<T: NumCast, U: NumCast, V: NumCast>(x: T, y: U, z: V) -> Self {
        Self {
            x: x.to_f64().unwrap_or_default(),
            y: y.to_f64().unwrap_or_default(),
            z: z.to_f64().unwrap_or_default(),
        }
    }

    pub fn x(&self) -> f64 {
        self.x
    }
    pub fn y(&self) -> f64 {
        self.y
    }
    pub fn z(&self) -> f64 {
        self.z
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn dot(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn unit_vector(&self) -> Self {
        *self / self.length()
    }

    pub fn to_color(self, samples_per_pixel: u32) -> Color {
        let scale = 1. / samples_per_pixel as f64;
        Color::from_rgba(
            (255.999 * (self.x * scale).sqrt()) as u8,
            (255.999 * (self.y * scale).sqrt()) as u8,
            (255.999 * (self.z * scale).sqrt()) as u8,
            255,
        )
    }

    pub fn subtract(&mut self, x: f64, y: f64, z: f64) -> &mut Self {
        self.x -= x;
        self.y -= y;
        self.z -= z;
        self
    }

    pub fn random() -> Self {
        let mut rng = thread_rng();
        Self {
            x: rng.gen::<f64>(),
            y: rng.gen::<f64>(),
            z: rng.gen::<f64>(),
        }
    }

    pub fn random_from_rng(rng: &mut StdRng) -> Self {
        Self {
            x: rng.gen::<f64>(),
            y: rng.gen::<f64>(),
            z: rng.gen::<f64>(),
        }
    }

    pub fn random_range(min: f64, max: f64) -> Self {
        let mut rng = thread_rng();
        Self {
            x: rng.gen_range(min..max),
            y: rng.gen_range(min..max),
            z: rng.gen_range(min..max),
        }
    }

    pub fn random_range_from_rng(rng: &mut StdRng, min: f64, max: f64) -> Self {
        Self {
            x: rng.gen_range(min..max),
            y: rng.gen_range(min..max),
            z: rng.gen_range(min..max),
        }
    }

    pub fn random_in_unit_sphere() -> Self {
        loop {
            let p = Self::random_range(-1., 1.);
            if p.length_squared() >= 1. {
                continue;
            }
            return p;
        }
    }

    pub fn random_in_unit_disk() -> Self {
        loop {
            let mut rng = thread_rng();
            let p = Vector3::new(rng.gen_range(-1. ..1.), rng.gen_range(-1. ..1.), 0);
            if p.length_squared() >= 1. {
                continue;
            }
            return p;
        }
    }

    pub fn near_zero(&self) -> bool {
        // Return true if the vector is close to zero in all dimensions.
        const S: f64 = 1e-8;
        (self.x.abs() < S) && (self.y.abs() < S) && (self.z.abs() < S)
    }

    pub fn reflect(&self, normal: &Vector3) -> Self {
        self - (2. * self.dot(normal)) * normal
    }

    pub fn refract(&self, normal: &Vector3, etai_over_etat: f64) -> Self {
        let cos_theta = (-1. * self).dot(normal).min(1.);
        let r_out_perp = etai_over_etat * (self + cos_theta * normal);
        let r_out_parallel = -((1.0 - r_out_perp.length_squared()).abs().sqrt()) * normal;
        r_out_perp + r_out_parallel
    }
}

// Below are the operator definitions

impl_op_ex!(+|lhs: &Vector3, rhs: &Vector3| -> Vector3 {
    Vector3 {
        x: lhs.x + rhs.x,
        y: lhs.y + rhs.y,
        z: lhs.z + rhs.z,
    }
});

impl_op_ex !( += |lhs: &mut Vector3, rhs: &Vector3| {
        lhs.x += rhs.x;
        lhs.y += rhs.y;
        lhs.z += rhs.z;
});

impl_op_ex!(-|lhs: &Vector3, rhs: &Vector3| -> Vector3 {
    Vector3 {
        x: lhs.x - rhs.x,
        y: lhs.y - rhs.y,
        z: lhs.z - rhs.z,
    }
});

impl_op_ex!( -= |lhs: &mut Vector3, rhs: &Vector3| {
        lhs.x -= rhs.x;
        lhs.y -= rhs.y;
        lhs.z -= rhs.z;
});

impl_op_ex!(*|lhs: &Vector3, rhs: &Vector3| -> Vector3 {
    Vector3 {
        x: lhs.x * rhs.x,
        y: lhs.y * rhs.y,
        z: lhs.z * rhs.z,
    }
});

impl_op_ex!(*=|lhs: &mut Vector3, rhs: &Vector3| {
        lhs.x *= rhs.x;
        lhs.y *= rhs.y;
        lhs.z *= rhs.z;
});

impl_op_ex_commutative!(*|lhs: &Vector3, rhs: f64| -> Vector3 {
    Vector3 {
        x: lhs.x * rhs,
        y: lhs.y * rhs,
        z: lhs.z * rhs,
    }
});

impl_op_ex!(*=|lhs: &mut Vector3, rhs: f64| {
        lhs.x *= rhs;
        lhs.y *= rhs;
        lhs.z *= rhs;
});

impl_op_ex_commutative!(*|lhs: &Vector3, rhs: u32| -> Vector3 {
    Vector3 {
        x: lhs.x * rhs as f64,
        y: lhs.y * rhs as f64,
        z: lhs.z * rhs as f64,
    }
});

impl_op_ex_commutative!(/|lhs: &Vector3, rhs: f64| -> Vector3 {
    Vector3 {
        x: lhs.x / rhs,
        y: lhs.y / rhs,
        z: lhs.z / rhs,
    }
});

impl_op_ex!(/=|lhs: &mut Vector3, rhs: f64| {
        lhs.x /= rhs;
        lhs.y /= rhs;
        lhs.z /= rhs;
});

impl_op_ex_commutative!(/|lhs: &Vector3, rhs: u32| -> Vector3 {
    Vector3 {
        x: lhs.x / rhs as f64,
        y: lhs.y / rhs as f64,
        z: lhs.z / rhs as f64,
    }
});

impl std::ops::Neg for Vector3 {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}
