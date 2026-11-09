use std::ops::{Add, Sub, Mul, Div, Neg};
// 3D vector struct and operations
#[derive(Debug, Clone, Copy)]
pub struct Vec3{
    x: f64,
    y: f64,
    z: f64,
}
impl Vec3{
    pub fn new(x: f64, y: f64, z: f64) -> Self{
        Vec3{x, y, z}
    }
    pub fn normalize(&self) -> Self{
        let len = (self.x*self.x + self.y*self.y + self.z*self.z).sqrt();
        if(len == 0.0){
            return Vec3{x: 0.0, y: 0.0, z: 0.0};
        }
        Vec3{x: self.x/len, y: self.y/len, z: self.z/len}
    }

    pub fn cross(&self, other: &Vec3) -> Self{
        Vec3{
            x: self.y*other.z - self.z*other.y,
            y: self.z*other.x - self.x*other.z,
            z: self.x*other.y - self.y*other.x,
        }
    }
    pub fn dot(&self, other: &Vec3) -> f64{
        self.x*other.x + self.y*other.y + self.z*other.z
    }
    pub fn length(&self) -> f64{
        (self.x*self.x + self.y*self.y + self.z*self.z).sqrt()
    }
    pub fn distance(&self, other: &Vec3) -> f64{
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        (dx*dx + dy*dy + dz*dz).sqrt()
    }
}

pub fn vec3(x: f64, y: f64, z: f64) -> Vec3{
    Vec3::new(x, y, z)
}

impl Add for Vec3{
    type Output = Vec3;
    fn add(self, other: Vec3) -> Vec3{
        Vec3{x: self.x + other.x, y: self.y + other.y, z: self.z + other.z}
    }
}
impl Sub for Vec3{
    type Output = Vec3;
    fn sub(self, other: Vec3) -> Vec3{
        Vec3{x: self.x - other.x, y: self.y - other.y, z: self.z - other.z}
    }
}
impl Mul<f64> for Vec3{
    type Output = Vec3;
    fn mul(self, scalar: f64) -> Vec3{
        Vec3{x: self.x * scalar, y: self.y * scalar, z: self.z * scalar}
    }
}
impl Div<f64> for Vec3{
    type Output = Vec3;
    fn div(self, scalar: f64) -> Vec3{
        Vec3{x: self.x / scalar, y: self.y / scalar, z: self.z / scalar}
    }
}
impl Neg for Vec3{
    type Output = Vec3;
    fn neg(self) -> Vec3{
        Vec3{x: -self.x, y: -self.y, z: -self.z}
    }
}