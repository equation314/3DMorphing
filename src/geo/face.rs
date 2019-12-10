use super::{Vertex, EPS};

pub type Face = Vec<usize>;

pub fn check_order(face: &Face, verts: &Vec<Vertex>, center: Vertex) -> bool {
    let n = face.len();
    if n < 3 {
        return false;
    }
    let face = face.iter().map(|id| verts[*id]).collect::<Vec<_>>();
    Vertex::det(face[0] - center, face[1] - center, face[2] - center) > EPS
}

pub fn adjust_order(face: &mut Face, verts: &Vec<Vertex>, center: Vertex) {
    if !check_order(face, verts, center) {
        face.reverse();
    }
}
