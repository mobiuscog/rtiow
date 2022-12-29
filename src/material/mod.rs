use crate::{Colour, Hit, Ray};

pub mod lambertian;
pub mod metal;

pub enum ScatterResult {
    ABSORBED {
        attenuation: Option<Colour>,
        scattered: Option<Ray>,
    },
    SCATTERED {
        attenuation: Colour,
        scattered: Ray,
    },
}

pub trait Material: Sync + Send {
    fn scatter(&self, ray: &Ray, rec: &Hit) -> ScatterResult;
}
