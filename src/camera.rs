use crate::vector_3::{Vector3, Point3};
use crate::ray::Ray;

pub struct Camera {
    pub origin: Point3,
    pub horizontal: Vector3,
    pub vertical: Vector3,
    pub up: Point3,
    pub right: Point3,
    pub forward: Point3,
    pub focus_distance: f32,
    pub lens_radius: f32,
}

impl Camera {

    pub fn new(origin: Point3, target: Point3, up: Vector3, vertical_fov: f32, aspect_ratio: f32, aperture: f32, focus_distance: f32) -> Camera {
        let theta: f32 = f32::to_radians(vertical_fov);
        let h: f32 = f32::tan(theta/2.0);
        let viewport_height: f32 = 2.0 * h;
        let viewport_width: f32 = aspect_ratio * viewport_height;

        let forward: Vector3 = Vector3::unit_vector(&(origin - target));
        let right: Vector3 = Vector3::unit_vector(&Vector3::cross(&up, &forward));
        let up: Vector3 = Vector3::cross(&forward, &right);

        let horizontal: Vector3 = focus_distance * viewport_width * right;
        let vertical: Vector3 = focus_distance * viewport_height * up;
        let lens_radius: f32 = aperture / 2.0;
        Camera { origin, up, right, forward, horizontal, vertical, focus_distance, lens_radius }
    }

    pub fn get_ray(&self, s: f32, t: f32, rng: &mut xorshift::StdRng) -> Ray {

        let radius: Vector3 = self.lens_radius * Vector3::random_in_disk(rng);
        let offset: Vector3 = self.right * radius.x + self.up * radius.y;

        Ray{
            origin: self.origin + offset,
            direction: self.lower_left_corner() + s * self.horizontal + t * self.vertical - self.origin - offset
        }
    }

    fn lower_left_corner(&self) -> Vector3 {
        self.origin - self.horizontal/2.0 - self.vertical/2.0 - self.focus_distance * self.forward
    } 
}