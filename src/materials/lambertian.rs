use crate::prelude::*;

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
