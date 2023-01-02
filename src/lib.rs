mod camera;
mod canvas;
mod hit;
mod materials;
mod ray;
mod renderer;
mod scene;
mod sphere;
mod vector3;

mod prelude {
    pub use crate::materials::{
        dielectric::Dielectric,
        lambertian::Lambertian,
        material::{
            Material,
            ScatterResult::{self, Absorbed, Scattered},
        },
        metal::Metal,
    };
    pub use crate::ray::Ray;
    pub use crate::sphere::Sphere;
    pub use crate::vector3::{Colour, Point3, Vector3};
    pub use crate::{
        hit::{Hit, Hittable},
        Boxable,
    };
}

pub use crate::renderer::run;

#[macro_use]
extern crate auto_ops;

pub trait Boxable {
    fn to_box(self) -> Box<Self>
    where
        Self: Sized,
    {
        Box::new(self)
    }
}
