use crate::material::Material;
use crate::{Boxable, Hit, Hittable, Point3, Ray};
use num_traits::NumCast;
use std::sync::Arc;

pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Option<Arc<dyn Material>>,
}

impl Sphere {
    pub fn new<T: NumCast>(center: Point3, radius: T, material: Arc<dyn Material>) -> Self {
        Self {
            center,
            radius: radius.to_f64().unwrap(),
            material: Some(material),
        }
    }
}

impl Boxable for Sphere {}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut Hit) -> bool {
        let oc = ray.origin() - self.center;
        let a = ray.direction().length_squared();
        let half_b = ray.direction().dot(&oc);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        return if discriminant < 0. {
            false
        } else {
            let sqrtd = discriminant.sqrt();

            // Find the nearest root that lies in the acceptable range.
            let mut root = (-half_b - sqrtd) / a;
            if root < t_min || t_max < root {
                root = (-half_b + sqrtd) / a;
                if root < t_min || t_max < root {
                    return false;
                }
            }

            rec.t = root;
            rec.p = ray.at(rec.t);
            let outward_normal = (rec.p - self.center) / self.radius;
            rec.set_face_normal(&ray, &outward_normal);
            if let Some(material) = &self.material {
                rec.material = Some(material.clone())
            }
            true
        };
    }
}
