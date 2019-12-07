use super::Vertex;

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

    pub fn intersect(&self, a: Vertex, b: Vertex) -> Option<Vertex> {
        Some(a)
    }
}
