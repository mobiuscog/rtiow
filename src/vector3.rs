use macroquad::prelude::Color;

#[derive(Clone, Copy, Debug, Default)]
pub struct Vector3 {
    x: f64,
    y: f64,
    z: f64,
}

pub type Point3 = Vector3;
pub type Colour = Vector3;

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
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

    pub fn dot(&self, other: Self) -> f64 {
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

    pub fn to_color(&self) -> Color {
        Color::from_rgba(
            (255.999 * self.x) as u8,
            (255.999 * self.y) as u8,
            (255.999 * self.z) as u8,
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
