use macroquad::prelude::Color;
use num_traits::NumCast;

#[derive(Clone, Copy, Default)]
pub struct Vector3 {
    x: f64,
    y: f64,
    z: f64,
}

pub type Point3 = Vector3;
pub type Colour = Vector3;

impl Vector3 {
    pub fn new<T: NumCast, U: NumCast, V: NumCast>(x: T, y: U, z: V) -> Self {
        Self {
            x: x.to_f64().unwrap(),
            y: y.to_f64().unwrap(),
            z: z.to_f64().unwrap(),
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

    pub fn cross(&self, other: Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn unit_vector(&self) -> Self {
        *self / self.length()
    }

    pub fn to_color(&self, samples_per_pixel: u32) -> Color {
        let scale = 1. / samples_per_pixel as f64;
        Color::from_rgba(
            (255.999 * self.x * scale) as u8,
            (255.999 * self.y * scale) as u8,
            (255.999 * self.z * scale) as u8,
            255,
        )
    }
}

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
