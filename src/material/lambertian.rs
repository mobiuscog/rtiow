use crate::material::ScatterResult::SCATTERED;
use crate::material::{Material, ScatterResult};
use crate::ray::Ray;
use crate::vector3::{Colour, Vector3};
use crate::Hit;

#[derive(Default)]
pub struct Lambertian {
    albedo: Colour,
}

impl Lambertian {
    pub fn new(albedo: Colour) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, rec: &Hit) -> ScatterResult {
        let mut scatter_direction = rec.normal + Vector3::random_in_unit_sphere().unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        SCATTERED {
            attenuation: self.albedo,
            scattered: Ray::new(rec.p, scatter_direction),
        }
    }
}