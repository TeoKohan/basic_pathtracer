use xorshift::Rng;

use crate::colour::Colour;
use crate::material;
use crate::vector_3::Vector3;
use crate::ray::Ray;
use crate::hit::HitResult;

pub struct Lambertian {
    pub albedo: Colour
}

impl material::Material for Lambertian {

    fn scatter(&self, ray: &Ray, hit_result: &HitResult, rng: &mut xorshift::StdRng) -> material::Scatter {
        match hit_result {
            HitResult::Hit(position, normal, _, _, _) => {
                let mut scatter_direction: Vector3 = *normal + Vector3::random_unit_vector(rng);
                if scatter_direction.length_squared() < 0.001 {
                    scatter_direction = *normal;
                }
                
                material::Scatter::Scatter(self.albedo, Ray { origin: *position, direction: scatter_direction })
            }
            HitResult::None => {
                return material::Scatter::None;
            }
        }
    }
}