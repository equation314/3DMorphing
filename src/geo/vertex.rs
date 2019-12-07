use std::ops;

#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vertex {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn len2(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn len(&self) -> f32 {
        self.len2().sqrt()
    }

    pub fn project_to_sphere(self, center: Self, radius: f32) -> Self {
        let dir = self - center;
        center + dir * (radius / dir.len())
    }
}

impl ops::Add for Vertex {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl ops::Sub for Vertex {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl ops::Mul<f32> for Vertex {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl ops::Div<f32> for Vertex {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
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

impl ops::MulAssign<f32> for Vertex {
    fn mul_assign(&mut self, rhs: f32) {
        *self = *self * rhs;
    }
}

impl ops::DivAssign<f32> for Vertex {
    fn div_assign(&mut self, rhs: f32) {
        *self = *self / rhs;
    }
}
