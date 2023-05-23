use std::{ops, f32::consts::PI};
use xorshift::Rng;

#[derive(Debug, Copy, Clone)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    pub const ZERO: Point3 = V3!(0.0, 0.0, 0.0);
    pub const X: Point3 = V3!(1.0, 0.0, 0.0);
    pub const Y: Point3 = V3!(0.0, 1.0, 0.0);
    pub const Z: Point3 = V3!(0.0, 0.0, 1.0);
    pub const ONE: Point3 = V3!(1.0, 1.0, 1.0);
}

pub type Point3 = Vector3;

impl ops::AddAssign for Vector3 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl ops::Add<Vector3> for Vector3 {
    type Output = Vector3;

    fn add(self, rhs: Vector3) -> Vector3 {
        V3!(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl ops::Neg for Vector3 {
    type Output = Vector3;

    fn neg(self) -> Vector3 {
        V3!(-self.x, -self.y, -self.z)
    }
}

impl ops::Sub<Vector3> for Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: Vector3) -> Vector3 {
        V3!(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl ops::Mul<Vector3> for Vector3 {
    type Output = Vector3;
    fn mul(self, rhs: Vector3) -> Vector3 {
        V3!(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl ops::Mul<f32> for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: f32) -> Vector3 {
        V3!(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl ops::Mul<Vector3> for f32 {
    type Output = Vector3;
    fn mul(self, rhs: Vector3) -> Vector3 {
        V3!(self * rhs.x, self * rhs.y, self * rhs.z)
    }
}

impl ops::Div<f32> for Vector3 {
    type Output = Vector3;

    fn div(self, rhs: f32) -> Vector3 {
        V3!(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl Vector3 {
    pub fn length_squared(&self) -> f32 {
        Vector3::dot(self, self)
    }

    pub fn length(&self) -> f32 {
        f32::sqrt(Vector3::length_squared(self))
    }

    pub fn unit_vector(&self) -> Vector3 {
        V3!(self.x, self.y, self.z) / Vector3::length(self)
    }

    pub fn dot(v: &Vector3, w: &Vector3) -> f32 {
        v.x * w.x + v.y * w.y + v.z * w.z
    }

    pub fn random(rng: &mut xorshift::StdRng) -> Vector3{
        V3!(rng.next_f32(), rng.next_f32(), rng.next_f32())
    }

    pub fn random_in_sphere(rng: &mut xorshift::StdRng) -> Vector3 {
        let theta: f32 = rng.next_f32() * 2.0 * PI;
        let phi: f32   = rng.next_f32() * PI;
        let rho: f32   = rng.next_f32();
        V3!(rho * phi.sin() * theta.cos(), rho * phi.sin() * theta.sin(), rho * phi.cos())
    }

    pub fn random_unit_vector(rng: &mut xorshift::StdRng) -> Vector3 {
        let theta: f32 = rng.next_f32() * 2.0 * PI;
        let phi: f32   = rng.next_f32() * PI;
        V3!(phi.sin() * theta.cos(), phi.sin() * theta.sin(), phi.cos())
    }

    pub fn reflect(v: &Vector3, n: &Vector3) -> Vector3 {
        *v - 2.0 * Vector3::dot(v, n) * *n
    }

    pub fn refract(uv: &Vector3, n: &Vector3, etai_over_etat: f32) -> Vector3 {
        let cos_theta: f32 = f32::min(Vector3::dot(&-*uv, n), 1.0);
        let r_out_perp: Vector3 =  etai_over_etat * (*uv + cos_theta * *n);
        let r_out_parallel: Vector3 = -f32::sqrt((1.0 - r_out_perp.length_squared()).abs()) * *n;
        return r_out_perp + r_out_parallel;
    }
}