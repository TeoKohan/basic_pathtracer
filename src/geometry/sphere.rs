use crate::vector_3::Vector3;
use crate::vector_3::Point3;
use std::rc::Rc;
use crate::hit::{Surface, HitResult};
use crate::material::Material;


pub struct Sphere {
    pub center: Point3,
    pub radius: f32,
    pub material: Rc<dyn Material>
}

enum QuadraticSolution {
    None,
    Result(f32, f32)
}

fn solve_quadratic(a : f32, b : f32, c : f32) -> QuadraticSolution {

    let discriminant : f32 = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        QuadraticSolution::None
    } else {
        //CURRENTLY RETURNING FIRST VALUE
        QuadraticSolution::Result((-b - discriminant.sqrt())/(2.0 * a), (-b + discriminant.sqrt())/(2.0 * a))
    }
}

impl Surface for Sphere {

    fn hit(&self, ray : &crate::ray::Ray, min : f32, max : f32) -> HitResult {

        let o : Vector3 = ray.origin - self.center;
        let a : f32 = ray.direction.length_squared();
        let b : f32 = 2.0 * Vector3::dot(&o, &ray.direction);
        let c : f32 = o.length_squared() - self.radius * self.radius;
        
        match solve_quadratic(a, b, c) {
            QuadraticSolution::None => HitResult::None,
            QuadraticSolution::Result(one, two) => {
                if min < one && one < max {
                    let p: Point3 = ray.at(one);
                    let n: Vector3 = (p - self.center) / self.radius;
                    let outward: bool = Vector3::dot(&ray.direction, &n) < 0.0;
                    let n: Vector3 = if outward {n} else {-n};
                    HitResult::Hit(p, n, one, self.material.clone(), outward)
                } else if min < two && two < max {
                    let p: Point3 = ray.at(two);
                    let n: Vector3 = (p - self.center) / self.radius;
                    let outward: bool = Vector3::dot(&ray.direction, &n) < 0.0;
                    let n: Vector3 = if outward {n} else {-n};
                    HitResult::Hit(p, n, two, self.material.clone(), outward)
                } else {
                    HitResult::None
                }
            }
        }
    }
}