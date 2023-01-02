use std::sync::Arc;

use crate::prelude::*;

#[derive(Clone)]
pub struct Hit {
    pub p: Point3,
    pub normal: Vector3,
    pub material: Option<Arc<dyn Material>>,
    pub t: f64,
    pub front_face: bool,
}

impl Default for Hit {
    fn default() -> Self {
        Self {
            p: Point3::default(),
            normal: Vector3::default(),
            material: None,
            t: 0.,
            front_face: true,
        }
    }
}

impl Hit {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vector3) {
        self.front_face = r.direction().dot(outward_normal) < 0.;
        if self.front_face {
            self.normal = outward_normal.to_owned();
        } else {
            self.normal = -outward_normal.to_owned();
        }
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut Hit) -> bool;
}

impl Hittable for &Vec<Box<dyn Hittable>> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut Hit) -> bool {
        let mut temp_rec: Hit = Default::default();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in self.iter() {
            if object.hit(ray, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec.clone();
            }
        }

        hit_anything
    }
}
