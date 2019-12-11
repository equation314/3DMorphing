use super::EPS;

use std::{cmp::Ordering, ops};

#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vertex {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn dot(self, rhs: Self) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn det(a: Self, b: Self, c: Self) -> f64 {
        a.dot(b * c)
    }

    pub fn len2(self) -> f64 {
        self.dot(self)
    }

    pub fn len(self) -> f64 {
        self.len2().sqrt()
    }

    pub fn unit(self) -> Self {
        let len = self.len();
        if len.abs() < EPS {
            self
        } else {
            self / len
        }
    }

    pub fn dist2(a: Self, b: Self) -> f64 {
        (a - b).len2()
    }

    pub fn dist(a: Self, b: Self) -> f64 {
        (a - b).len()
    }

    pub fn project_to_sphere(self, center: Self, radius: f64) -> Self {
        let dir = self - center;
        dir * (radius / dir.len())
    }

    pub fn bounding_box(verts: &Vec<Self>) -> (Self, Self) {
        let mut bbox = (Self::new(std::f64::MAX, std::f64::MAX, std::f64::MAX),
        Self::new(std::f64::MIN, std::f64::MIN, std::f64::MIN));
        for v in verts {
            if v.x < bbox.0.x {
                bbox.0.x = v.x
            }
            if v.y < bbox.0.y {
                bbox.0.y = v.y
            }
            if v.z < bbox.0.z {
                bbox.0.z = v.z
            }

            if v.x > bbox.1.x {
                bbox.1.x = v.x
            }
            if v.y > bbox.1.y {
                bbox.1.y = v.y
            }
            if v.z > bbox.1.z {
                bbox.1.z = v.z
            }
        }
        bbox
    }

    pub fn max(self) -> f64 {
        if self.x > self.y && self.x > self.z {
            self.x
        } else if self.y > self.z {
            self.y
        } else {
            self.z
        }
    }
}

impl Ord for Vertex {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.x, self.y, self.z)
            .partial_cmp(&(other.x, other.y, other.z))
            .unwrap_or(Ordering::Equal)
    }
}

impl PartialOrd for Vertex {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Vertex {
    fn eq(&self, other: &Self) -> bool {
        (self.x - other.x).abs() < EPS
            && (self.y - other.y).abs() < EPS
            && (self.z - other.z).abs() < EPS
    }
}

impl Eq for Vertex {}

impl ops::Add for Vertex {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl ops::Neg for Vertex {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y, -self.z)
    }
}

impl ops::Sub for Vertex {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl ops::Mul<f64> for Vertex {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl ops::Mul<Vertex> for Vertex {
    type Output = Self;

    fn mul(self, rhs: Vertex) -> Self::Output {
        Self::new(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }
}

impl ops::Div<f64> for Vertex {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        if rhs == 0.0 {
            panic!("Cannot divide `Vertex` by zero-valued!");
        }

        Self::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl ops::AddAssign for Vertex {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl ops::SubAssign for Vertex {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl ops::MulAssign<f64> for Vertex {
    fn mul_assign(&mut self, rhs: f64) {
        *self = *self * rhs;
    }
}

impl ops::DivAssign<f64> for Vertex {
    fn div_assign(&mut self, rhs: f64) {
        *self = *self / rhs;
    }
}
