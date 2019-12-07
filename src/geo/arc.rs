use super::Vertex;

#[derive(Debug)]
pub struct Arc {
    pub a: Vertex,
    pub b: Vertex,
    pub a_id: usize,
    pub b_id: usize,
}

#[derive(Debug)]
pub enum ArcIntersectionResult {
    T1(usize, f32), // `T` shape 1, A's endpoint on B, return A's endpoint id
    T2(usize, f32), // `T` shape 2, B's endpoint on A, return B's endpoint id
    L(usize, f32),  // `L` shape, A's endpoint on B's endpoint, return A's endpoint id
    X(Vertex, f32), // `X` shape, return the intersectoon coordinates
}

impl Arc {
    pub fn new(a: Vertex, b: Vertex, a_id: usize, b_id: usize) -> Self {
        Arc { a, b, a_id, b_id }
    }

    pub fn intersect(a: &Arc, b: &Arc) -> ArcIntersectionResult {
        ArcIntersectionResult::T1(a.a_id, 0.0)
    }
}
