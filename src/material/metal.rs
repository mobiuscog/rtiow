use crate::material::ScatterResult::{ABSORBED, SCATTERED};
use crate::material::{Material, ScatterResult};
use crate::ray::Ray;
use crate::vector3::Colour;
use crate::Hit;

#[derive(Default)]
pub struct Metal {
    albedo: Colour,
}

impl Metal {
    pub fn new(albedo: Colour) -> Self {
        Self { albedo }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, rec: &Hit) -> ScatterResult {
        let reflection_direction = ray.direction().unit_vector().reflect(&rec.normal);
        match reflection_direction.dot(&rec.normal) > 0. {
            true => SCATTERED {
                attenuation: self.albedo,
                scattered: Ray::new(rec.p, reflection_direction),
            },
            false => ABSORBED {
                attenuation: None,
                scattered: None,
            },
        }
    }
}
