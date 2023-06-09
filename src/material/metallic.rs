use crate::colour::Colour;
use crate::material;
use crate::vector_3::Vector3;
use crate::ray::Ray;
use crate::hit::HitResult;

use super::Scatter;

pub struct Metallic {
    pub albedo: Colour,
    pub fuzziness: f32
}

impl material::Material for Metallic {

    fn scatter(&self, ray: &Ray, hit_result: &HitResult, rng: &mut xorshift::StdRng) -> material::Scatter {
        match hit_result {
            HitResult::Hit(position, normal, _, _, _) => {
                let reflected: Vector3 = Vector3::reflect(&ray.direction.unit_vector(), normal);
                Scatter::Scatter(self.albedo, Ray{ origin: *position, direction: reflected + self.fuzziness * Vector3::random_in_sphere(rng)})
            }
            HitResult::None => material::Scatter::None
        }
    }
}