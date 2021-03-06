use std::fs::File;
use std::io::{self, prelude::*, BufReader, BufWriter};
use std::vec::Vec;
use std::{cmp::Ordering, ops::Deref};

use crate::geo::{adjust_order, Arc, ArcIntersectionResult, Face, Triangle, EPS};
use crate::graph::{Edge, EdgeList, Graph, RcGraphEdge};
use crate::Config;
use crate::Vertex;

const SPHERE_RADIUS: f64 = 100.0;
const MODEL_SIZE: f64 = 1.0;

#[derive(Debug)]
pub struct Model {
    verts: Vec<Vertex>,
    faces: Vec<Face>,
}

#[derive(Debug)]
pub struct ProjectionModel {
    model: Model,
    edges: EdgeList,

    center: Vertex,
    sphere_verts: Vec<Vertex>,
}

impl Deref for ProjectionModel {
    type Target = Model;

    fn deref(&self) -> &Self::Target {
        &self.model
    }
}

#[derive(Debug)]
pub struct MergedModel {
    faces: Vec<Face>,
    vert_pairs: Vec<(Vertex, Vertex)>,
}

impl Model {
    pub fn nr_verts(&self) -> usize {
        self.verts.len()
    }

    pub fn nr_faces(&self) -> usize {
        self.faces.len()
    }

    pub fn new(verts: Vec<Vertex>, faces: Vec<Face>) -> Self {
        Self { verts, faces }
    }

    pub fn center(&self) -> Vertex {
        let mut center = Vertex::new(0.0, 0.0, 0.0);
        for v in &self.verts {
            center += *v;
        }
        center / self.nr_verts() as f64
    }

    pub fn load(filename: &str) -> io::Result<Self> {
        assert!(filename.ends_with(".obj"));

        let file = File::open(filename)?;
        let reader = BufReader::new(file);

        let mut verts = Vec::new();
        let mut faces = Vec::new();
        for line in reader.lines() {
            let line = line?;
            let vals = line.split_whitespace().collect::<Vec<_>>();
            if vals.len() == 0 {
                continue;
            }
            match vals[0] {
                "v" => {
                    assert!(vals.len() == 4);
                    verts.push(Vertex::new(
                        vals[1].parse().unwrap(),
                        vals[2].parse().unwrap(),
                        vals[3].parse().unwrap(),
                    ))
                }
                "f" => {
                    assert!(vals.len() == 4);
                    faces.push(vec![
                        vals[1].parse::<usize>().unwrap() - 1,
                        vals[2].parse::<usize>().unwrap() - 1,
                        vals[3].parse::<usize>().unwrap() - 1,
                    ])
                }
                _ => {}
            }
        }
        Ok(Self::new(verts, faces))
    }

    pub fn save(&self, filename: &str) -> io::Result<()> {
        assert!(filename.ends_with(".obj"));

        let file = File::create(filename)?;
        let mut writer = BufWriter::new(file);

        for v in &self.verts {
            writeln!(writer, "v {} {} {}", v.x, v.y, v.z)?;
        }
        for f in &self.faces {
            let mut line = "f".to_string();
            for id in f {
                line += &format!(" {}", id + 1);
            }
            writeln!(writer, "{}", line)?;
        }
        Ok(())
    }
}

impl ProjectionModel {
    pub fn new(model: Model) -> Self {
        let center = model.center();

        let mut sphere_verts = Vec::new();
        for v in &model.verts {
            sphere_verts.push(v.project_to_sphere(center, SPHERE_RADIUS));
        }

        let mut edges = EdgeList::new();
        for f in &model.faces {
            edges.add(f[0], f[1]);
            edges.add(f[1], f[2]);
            edges.add(f[2], f[0]);
        }

        Self {
            model,
            edges,
            center,
            sphere_verts,
        }
    }

    fn project_from_sphere(&self, v: Vertex) -> Vertex {
        for f in &self.faces {
            let tri = Triangle::new(self.verts[f[0]], self.verts[f[1]], self.verts[f[2]]);
            if let Some(int) = tri.intersect(self.center, self.center + v) {
                return int;
            }
        }
        panic!(format!("No intersect found of {:?}!", v))
    }
}

struct SphereVertex {
    v: Vertex,
    from: usize,
    index: usize,
}

impl MergedModel {
    pub fn save(&self, filename: &str) -> io::Result<()> {
        assert!(filename.ends_with(".obj"));

        let file = File::create(filename)?;
        let mut writer = BufWriter::new(file);

        for v in &self.vert_pairs {
            writeln!(writer, "v {} {} {}", v.0.x, v.0.y, v.0.z)?;
        }
        for v in &self.vert_pairs {
            writeln!(writer, "u {} {} {}", v.1.x, v.1.y, v.1.z)?;
        }
        for f in &self.faces {
            let mut line = "f".to_string();
            for id in f {
                line += &format!(" {}", id + 1);
            }
            writeln!(writer, "{}", line)?;
        }
        Ok(())
    }

    pub fn merge(model1: ProjectionModel, model2: ProjectionModel, config: Config) -> Self {
        let mut all_sphere_verts = Vec::new();
        let mut all_edges = EdgeList::new();

        // origin sphere vertices of two models
        let n = model1.nr_verts();
        let m = model2.nr_verts();
        for i in 0..n {
            all_sphere_verts.push(SphereVertex {
                v: model1.sphere_verts[i],
                from: 1,
                index: i,
            });
        }
        for i in 0..m {
            all_sphere_verts.push(SphereVertex {
                v: model2.sphere_verts[i],
                from: 2,
                index: i,
            });
        }
        println!("SIZE {:?} {:?}", all_sphere_verts.len(), all_edges.len());

        // calcuation new vertices from intersection, split & add edges
        for e in model1.edges.iter() {
            all_edges.add(e.from, e.to);
        }
        for e2 in model2.edges.iter() {
            let e2 = Edge::new(e2.from + n, e2.to + n);
            let v1 = all_sphere_verts[e2.from].v;
            let v2 = all_sphere_verts[e2.to].v;
            let arc2 = Arc::new(v1, v2, e2.from, e2.to);
            let mut ints = vec![(0.0, e2.from), (1.0, e2.to)];

            let mut donot_add = false;
            for e1 in &mut all_edges.clone().iter() {
                let u1 = all_sphere_verts[e1.from].v;
                let u2 = all_sphere_verts[e1.to].v;
                let arc1 = Arc::new(u1, u2, e1.from, e1.to);

                match Arc::intersect(&arc1, &arc2) {
                    ArcIntersectionResult::T1(index, k) => ints.push((k, index)),
                    ArcIntersectionResult::T2(index, _k) => {
                        all_edges.remove(e1);
                        all_edges.add(e1.from, index);
                        all_edges.add(e1.to, index);
                    }
                    ArcIntersectionResult::X(v, k) => {
                        let id = all_sphere_verts.len();
                        all_sphere_verts.push(SphereVertex {
                            v,
                            from: 0,
                            index: 0,
                        });
                        all_edges.remove(e1);
                        all_edges.add(e1.from, id);
                        all_edges.add(e1.to, id);
                        ints.push((k, id))
                    }
                    ArcIntersectionResult::I((id1, k1), (id2, k2)) => {
                        all_edges.remove(e1);
                        if k1 > 0.0 {
                            ints.push((k1, id1))
                        } else if k1 < 0.0 {
                            all_edges.add(id1, ints[0].1);
                        } else {
                            // assert!(id1 == e2.from);
                        }

                        if k2 < 1.0 {
                            ints.push((k2, id2))
                        } else if k2 > 1.0 {
                            all_edges.add(id2, ints[1].1);
                        } else {
                            // assert!(id2 == e2.to);
                        }
                    }
                    ArcIntersectionResult::L(id1, id2) => {
                        if id2 == e2.from {
                            // assert!(ints[0].1 == id1);
                            ints[0].1 = id1
                        } else if id2 == e2.to {
                            // assert!(ints[1].1 == id1);
                            ints[1].1 = id1
                        }
                    }
                    ArcIntersectionResult::S => {
                        donot_add = true;
                        break;
                    }
                    ArcIntersectionResult::N => {}
                }
            }
            if donot_add {
                continue;
            }

            ints.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
            for i in 0..ints.len() - 1 {
                all_edges.add(ints[i].1, ints[i + 1].1);
            }
        }
        println!("SIZE {:?} {:?}", all_sphere_verts.len(), all_edges.len());

        // project back to the origin model
        let mut model_vert_pairs = Vec::new();
        for v in &all_sphere_verts {
            let mut p = if config.sphere_only {
                (v.v, v.v)
            } else {
                match v.from {
                    1 => (model1.verts[v.index], model2.project_from_sphere(v.v)),
                    2 => (model1.project_from_sphere(v.v), model2.verts[v.index]),
                    _ => (
                        model1.project_from_sphere(v.v),
                        model2.project_from_sphere(v.v),
                    ),
                }
            };
            p.0 -= model1.center;
            p.1 -= model2.center;
            model_vert_pairs.push(p);
        }

        // scale models to the same scale
        let bbox1 = Vertex::bounding_box(&model_vert_pairs.iter().map(|p| p.0).collect());
        let bbox2 = Vertex::bounding_box(&model_vert_pairs.iter().map(|p| p.1).collect());
        let mut scale1 = (bbox1.1 - bbox1.0).max();
        let mut scale2 = (bbox2.1 - bbox2.0).max();
        println!("Bounding box 1: {:?}", bbox1);
        println!("Bounding box 2: {:?}", bbox2);
        println!("Scale 1: {:?}", scale1);
        println!("Scale 2: {:?}", scale2);
        if !config.scale {
            let r = if scale1 > scale2 { scale1 } else { scale2 };
            scale1 = r;
            scale2 = r;
        }
        for p in &mut model_vert_pairs {
            p.0 *= MODEL_SIZE / scale1;
            p.1 *= MODEL_SIZE / scale2;
        }

        let all_sphere_verts = all_sphere_verts.iter().map(|v| v.v).collect::<Vec<_>>();
        let all_faces = if config.edge_only {
            // show all edges only, without faces
            for p in model_vert_pairs.clone().iter() {
                model_vert_pairs.push(*p)
            }
            let n = all_sphere_verts.len();
            all_edges
                .iter()
                .map(|e| vec![e.from, e.to, e.to + n])
                .collect()
        } else {
            // face tracing
            Self::resolve_faces(&all_sphere_verts, &all_edges)
        };

        // triangulize & unique
        let mut triangle_faces = Vec::new();
        let mut set = std::collections::BTreeSet::<Vec<usize>>::new();
        for f in all_faces {
            if f.len() > 3 {
                for i in 1..f.len() - 1 {
                    let mut tri = vec![f[0], f[i], f[i + 1]];
                    tri.sort();
                    if f[0] == f[i] || f[0] == f[i + 1] {
                        continue;
                    }
                    if set.insert(tri.clone()) {
                        adjust_order(&mut tri, &all_sphere_verts, Vertex::new(0.0, 0.0, 0.0));
                        triangle_faces.push(tri);
                    }
                }
            } else {
                let mut tri = f;
                tri.sort();
                if set.insert(tri.clone()) {
                    adjust_order(&mut tri, &all_sphere_verts, Vertex::new(0.0, 0.0, 0.0));
                    triangle_faces.push(tri);
                }
            }
        }

        MergedModel {
            vert_pairs: model_vert_pairs,
            faces: triangle_faces,
        }
    }

    pub fn interpolation(&self, ratio: f64) -> Model {
        let mut new_verts = Vec::new();
        for (v1, v2) in &self.vert_pairs {
            new_verts.push(*v1 + (*v2 - *v1) * ratio);
        }

        let mut model = Model::new(new_verts, self.faces.clone());
        let center = model.center();
        for mut f in &mut model.faces {
            adjust_order(&mut f, &model.verts, center);
        }
        model
    }

    fn resolve_faces(verts: &Vec<Vertex>, edges: &EdgeList) -> Vec<Face> {
        let n = verts.len();
        let mut graph = Graph::new(verts);
        for e in edges.iter() {
            graph.add_pair(e.from, e.to);
        }

        // get next edge
        for i in 0..n {
            let v = verts[i];
            let v_len2 = v.len2();
            let m = graph.neighbors_count(i);
            if m < 1 {
                continue;
            }
            let first = verts[graph.neighbors(i).next().unwrap().borrow().to];
            let first_dir = (first - v * (v.dot(first) / v_len2)).unit();
            let mut adj_edges = graph
                .neighbors(i)
                .map(|e| {
                    let p = verts[e.borrow().to];
                    let dir = (p - v * (v.dot(p) / v_len2)).unit();
                    let norm = first_dir * dir;
                    let cos = first_dir.dot(dir);
                    let mut angle = if (cos - 1.0).abs() < EPS {
                        0.0
                    } else if (cos + 1.0).abs() < EPS {
                        std::f64::consts::PI
                    } else {
                        cos.acos()
                    };
                    if v.dot(norm) < -EPS {
                        angle = -angle;
                    }
                    (angle, e)
                })
                .collect::<Vec<(f64, &RcGraphEdge)>>();
            adj_edges.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(Ordering::Equal));
            for j in 0..m {
                let k = if j == m - 1 { 0 } else { j + 1 };
                adj_edges[j].1.borrow_mut().next = std::rc::Rc::downgrade(&adj_edges[k].1);
            }
        }

        // get faces
        let mut faces = Vec::<Face>::new();
        for i in 0..n {
            for e in graph.neighbors(i) {
                let mut e = e.clone();
                let mut one_face = Vec::new();
                while !e.borrow().visited {
                    let p = e.borrow().to;
                    one_face.push(p);
                    e.borrow_mut().visited = true;
                    let o = e.borrow().oppo.upgrade().expect("No opposite edge!");
                    let n = o.borrow().next.upgrade().expect("No next edge");
                    e = n;
                }
                if one_face.len() > 2 {
                    faces.push(one_face);
                }
            }
        }
        faces
    }
}
