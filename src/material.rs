use crate::hit;
use crate::colour;
use crate::ray;

pub mod lambertian;
pub mod metallic;
pub mod dielectric;

pub enum Scatter {
    None,
    Scatter(colour::Colour, ray::Ray)
}

pub trait Material {
    fn scatter(&self, ray : &ray::Ray, hit_result: &hit::HitResult, rng: &mut xorshift::StdRng) -> Scatter;
}