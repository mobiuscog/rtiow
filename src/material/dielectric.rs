use crate::material::{
    Material,
    ScatterResult::{self, SCATTERED},
};
use crate::ray::Ray;
use crate::vector3::Colour;
use crate::Hit;

use num_traits::Pow;
use rand::{thread_rng, Rng};

#[derive(Default)]
pub struct Dielectric {
    index_of_refraction: f64,
}

impl Dielectric {
    pub fn new(index_of_refraction: f64) -> Self {
        Self {
            index_of_refraction,
        }
    }

    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        // Use Schlick's approximation for reflectance.
        let mut r0 = (1. - ref_idx) / (1. + ref_idx);
        r0 *= r0;
        r0 + (1. - r0) * (1. - cosine).pow(5.)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, rec: &Hit) -> ScatterResult {
        let refraction_ratio = match rec.front_face {
            true => 1. / self.index_of_refraction,
            false => self.index_of_refraction,
        };

        let unit_direction = ray.direction().unit_vector();

        let cos_theta = (-1. * unit_direction).dot(&rec.normal).min(1.);
        let sin_theta = (1. - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.
            || Dielectric::reflectance(cos_theta, refraction_ratio) > thread_rng().gen::<f64>();

        let direction = match cannot_refract {
            true => unit_direction.reflect(&rec.normal),
            false => unit_direction.refract(&rec.normal, refraction_ratio),
        };

        SCATTERED {
            attenuation: Colour::new(1., 1., 1.),
            scattered: Ray::new(rec.p, direction),
        }
    }
}
