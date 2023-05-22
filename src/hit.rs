use std::rc::Rc;
use crate::vector_3::{Vector3, Point3};
use crate::ray::Ray;
use crate::material::Material;

//Position, Normal, Distance, Outward
pub enum HitResult {
    Hit(Point3, Vector3, f32, Rc<dyn Material>, bool),
    None,
}

pub trait Surface {
    fn hit(&self, ray: &Ray, min: f32, max: f32) -> HitResult;
}

pub struct HitList  {
    pub objects: Vec<Box<dyn Surface>>
}

impl Surface for HitList {
    fn hit(&self, ray: &crate::ray::Ray, min: f32, max: f32) -> HitResult {
        let mut hit_record: HitResult = HitResult::None;
        let mut max: f32 = max;

        for object in self.objects.iter() {
            if let HitResult::Hit(point, normal, t, material, outward) = object.hit(ray, min, max) {
                max = t;
                hit_record = HitResult::Hit(point, normal, t, material, outward);
            }
        }
        hit_record
    }
}

impl HitList {
    pub fn add(&mut self, surface: impl Surface + 'static) -> () {
        self.objects.push(Box::new(surface))
    }
}