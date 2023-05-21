use std::ops;

#[derive(Debug, Copy, Clone)]
pub struct Vector3 {
    pub x : f32,
    pub y : f32,
    pub z : f32,
}

impl Vector3 {
    pub const ZERO : Point3 = V3!(0.0, 0.0, 0.0);
    pub const X : Point3 = V3!(1.0, 0.0, 0.0);
    pub const Y : Point3 = V3!(0.0, 1.0, 0.0);
    pub const Z : Point3 = V3!(0.0, 0.0, 1.0);
    pub const ONE : Point3 = V3!(1.0, 1.0, 1.0);
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
    type Output = f32;
    fn mul(self, rhs: Vector3) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
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
        *self * *self
    }

    pub fn length(&self) -> f32 {
        f32::sqrt(Vector3::length_squared(self))
    }

    pub fn unit_vector(&self) -> Vector3 {
        V3!(self.x, self.y, self.z) / Vector3::length(self)
    }

    pub fn dot(v : &Vector3, w : &Vector3) -> f32 {
        v.x * w.x + v.y * w.y + v.z * w.z
    }
}

