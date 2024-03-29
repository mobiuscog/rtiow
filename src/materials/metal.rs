use crate::prelude::*;

#[derive(Default)]
pub struct Metal {
    albedo: Colour,
    blur: f64,
}

impl Metal {
    pub fn new(albedo: Colour) -> Self {
        Self { albedo, blur: 0. }
    }

    #[allow(dead_code)]
    pub fn new_blurred(albedo: Colour, blur: f64) -> Self {
        let blurred = blur.clamp(0., 1.);
        Self {
            albedo,
            blur: blurred,
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, rec: &Hit) -> ScatterResult {
        let reflection_direction = ray.direction().unit_vector().reflect(&rec.normal);
        match reflection_direction.dot(&rec.normal) > 0. {
            true => Scattered {
                attenuation: self.albedo,
                scattered: Ray::new(
                    rec.p,
                    reflection_direction + (Vector3::random_in_unit_sphere() * self.blur),
                ),
            },
            false => Absorbed {
                attenuation: None,
                scattered: None,
            },
        }
    }
}
