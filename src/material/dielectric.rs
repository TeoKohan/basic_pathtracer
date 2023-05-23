use xorshift::Rng;

use crate::material;
use crate::vector_3::Vector3;
use crate::ray::Ray;
use crate::hit::HitResult;

pub struct Dielectric {
    pub refraction_index: f32
}

fn reflectance(cos: f32, refraction_index: f32) -> f32 {
    let r0: f32 = (1.0 - refraction_index) / (1.0 + refraction_index);
    let r0: f32 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cos).powi(5)
}

impl material::Material for Dielectric {

    fn scatter(&self, ray: &Ray, hit_result: &HitResult, rng: &mut xorshift::StdRng) -> material::Scatter {
        match hit_result {
            HitResult::Hit(position, normal, _, _, outward) => {
                let refraction_ratio: f32 = if *outward {1.0 / self.refraction_index} else {self.refraction_index};
                let unit_direction: Vector3 = Vector3::unit_vector(&ray.direction);

                let cos_theta: f32 = f32::min(Vector3::dot(&-unit_direction, normal), 1.0);
                let sin_theta: f32 = f32::sqrt(1.0 - cos_theta * cos_theta);
    
                let cannot_refract: bool = refraction_ratio * sin_theta > 1.0;
                let direction: Vector3 = if cannot_refract || reflectance(cos_theta, refraction_ratio) > rng.next_f32()
                                            {Vector3::reflect(&unit_direction, normal)} 
                                        else 
                                            {Vector3::refract(&unit_direction, normal, refraction_ratio)};
                material::Scatter::Scatter(Vector3::ONE, Ray { origin: *position, direction: direction })
            }
            HitResult::None => {
                material::Scatter::None
            }
        }
    }
}