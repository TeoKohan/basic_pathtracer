use crate::vector_3;

#[derive(Copy, Clone)]
pub struct Ray {
    pub origin    : vector_3::Point3 ,
    pub direction : vector_3::Vector3,
}

impl Ray {
    pub fn at(&self, t: f32) -> vector_3::Point3 {
        return self.origin + t * self.direction;
    }
}