use super::{Vertex, EPS};

#[derive(Debug)]
pub struct Triangle {
    a: Vertex,
    b: Vertex,
    c: Vertex,
}

impl Triangle {
    pub fn new(a: Vertex, b: Vertex, c: Vertex) -> Self {
        Triangle { a, b, c }
    }

    pub fn norm(&self) -> Vertex {
        (self.b - self.a) * (self.c - self.a)
    }

    pub fn contains(&self, v: Vertex) -> bool {
        let a = self.a - v;
        let b = self.b - v;
        let c = self.c - v;
        let area = self.norm().len();
        let sum = (a * b).len() + (b * c).len() + (c * a).len();
        return (area - sum).abs() < EPS;
    }

    pub fn intersect(&self, a: Vertex, b: Vertex) -> Option<Vertex> {
        let norm = self.norm();
        let div = (a - b).dot(norm);
        if div.abs() < EPS {
            return None;
        }
        let t = (a - self.a).dot(norm) / div;
        let v = a + (b - a) * t;

        if t > EPS && self.contains(v) {
            Some(v)
        } else {
            None
        }
    }
}
