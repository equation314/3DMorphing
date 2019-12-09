use super::{Vertex, EPS};

#[derive(Debug)]
pub struct Arc {
    pub a: Vertex,
    pub b: Vertex,
    pub a_id: usize,
    pub b_id: usize,
}

#[derive(Debug)]
pub enum ArcIntersectionResult {
    I((usize, f64), (usize, f64)), // `I` shape, two arcs are co-planar
    T1(usize, f64),                // `T` shape 1, A's endpoint on B, return A's endpoint id
    T2(usize, f64),                // `T` shape 2, B's endpoint on A, return B's endpoint id
    L(usize, usize), // `L` shape, A's endpoint on B's endpoint, return two endpoint ids
    X(Vertex, f64),  // `X` shape, return the intersectoon coordinates
    N,               // no intersection
    S,               // Two arcs are same
}

use ArcIntersectionResult::*;

impl Arc {
    pub fn new(a: Vertex, b: Vertex, a_id: usize, b_id: usize) -> Self {
        Arc { a, b, a_id, b_id }
    }

    fn planar_contains(&self, v: Vertex) -> bool {
        let ab = self.a * self.b;
        let ba = -ab;
        (self.a * v).dot(ab) > EPS && (self.b * v).dot(ba) > EPS
    }

    fn contains(&self, v: Vertex) -> Option<f64> {
        if v.dot(self.a * self.b).abs() < EPS && self.planar_contains(v) {
            let a_unit = self.a.unit();
            let b_unit = self.b.unit();
            Some(v.unit().dot(a_unit).acos() / b_unit.dot(a_unit).acos())
        } else {
            None
        }
    }

    pub fn intersect(a: &Arc, b: &Arc) -> ArcIntersectionResult {
        let ab = a.a * a.b;
        let ba = -ab;
        let cd = b.a * b.b;
        let dc = -cd;

        if (ab * cd).len() < EPS {
            if a.a.dot(cd).abs() > EPS {
                return N;
            }

            let mut res = Vec::new();
            for (v, id) in vec![(a.a, a.a_id), (a.b, a.b_id)] {
                if v == b.a {
                    res.push((id, 0.0))
                } else if v == b.b {
                    res.push((id, 1.0))
                } else if let Some(k) = b.contains(v) {
                    res.push((id, k));
                } else if Vertex::dist2(v, b.a) < Vertex::dist2(v, b.b) {
                    res.push((id, -1.0));
                } else {
                    res.push((id, 2.0));
                }
            }

            if res.len() != 2 {
                return N;
            }
            if res[0].1 > res[1].1 {
                res.swap(0, 1);
            }
            if (res[0].1 < 0.0 && res[1].1 < 0.0) || (res[0].1 > 1.0 && res[1].1 > 1.0) {
                return N;
            }
            if (res[0].1 < 0.0 && a.planar_contains(b.a))
                || (res[1].1 > 1.0 && !a.planar_contains(b.b))
            {
                return N;
            }
            if res[0].1 == 0.0 && res[1].1 == 1.0 {
                return S;
            }
            if res[0].1 < 0.0 && res[1].1 == 0.0 {
                return L(res[1].0, b.a_id);
            }
            if res[0].1 == 1.0 && res[1].1 > 1.0 {
                return L(res[0].0, b.b_id);
            }
            return I(res[0], res[1]);
        }

        if a.a == b.a {
            return L(a.a_id, b.a_id);
        } else if a.a == b.b {
            return L(a.a_id, b.b_id);
        } else if a.b == b.a {
            return L(a.b_id, b.a_id);
        } else if a.b == b.b {
            return L(a.b_id, b.b_id);
        }

        if let Some(k) = b.contains(a.a) {
            return T1(a.a_id, k);
        } else if let Some(k) = b.contains(a.b) {
            return T1(a.b_id, k);
        } else if let Some(k) = a.contains(b.a) {
            return T2(b.a_id, k);
        } else if let Some(k) = a.contains(b.a) {
            return T2(b.b_id, k);
        }

        if b.a.dot(ab) * b.b.dot(ba) > EPS && a.a.dot(cd) * a.b.dot(dc) > EPS {
            let div = (a.a - a.b).dot(cd);
            if div.abs() < EPS {
                return N;
            }
            let t = a.a.dot(cd) / div;
            let v_unit = (a.a + (a.b - a.a) * t).unit();

            let c_len = b.a.len();
            let c_unit = b.a / c_len;
            let d_unit = b.b.unit();
            let v = v_unit * c_len;

            if b.planar_contains(v) {
                let k = v_unit.dot(c_unit).acos() / d_unit.dot(c_unit).acos();
                return X(v, k);
            } else {
                return N;
            }
        }

        N
    }
}
