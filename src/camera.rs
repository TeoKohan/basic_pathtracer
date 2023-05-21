use crate::vector_3::{Vector3, Point3};
use crate::ray::Ray;

pub struct Camera {
    pub origin: Point3,
    pub horizontal: Vector3,
    pub vertical: Vector3,
    pub focal_length: f32,
}

impl Camera {
    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray{origin: self.origin, direction: self.lower_left_corner() + u * self.horizontal + v * self.vertical - self.origin}
    }

    fn lower_left_corner(&self) -> Vector3 {
        self.origin - self.horizontal / 2.0 - self.vertical / 2.0 - self.focal_length * Vector3::Z
    } 
}