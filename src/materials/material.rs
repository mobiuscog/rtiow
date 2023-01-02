use crate::prelude::*;

pub enum ScatterResult {
    Absorbed {
        attenuation: Option<Colour>,
        scattered: Option<Ray>,
    },
    Scattered {
        attenuation: Colour,
        scattered: Ray,
    },
}

pub trait Material: Sync + Send {
    fn scatter(&self, ray: &Ray, rec: &Hit) -> ScatterResult;
}
