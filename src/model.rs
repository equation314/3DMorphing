use std::fs::File;
use std::io::{self, prelude::*, BufReader, BufWriter};
use std::vec::Vec;

use crate::geo::{Arc, ArcIntersectionResult, Triangle};
use crate::graph::{Edge, EdgeList, Face, GraphEdgeList};
use crate::Vertex;

const SPHERE_RADIUS: f64 = 100.0;

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

impl std::ops::Deref for ProjectionModel {
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
        let mut center = Vertex::new(0.0, 0.0, 0.0);
        for v in &model.verts {
            center += *v;
        }
        center /= model.nr_verts() as f64;

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
            if let Some(int) = tri.intersect(self.center, v) {
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
    pub fn merge(model1: ProjectionModel, model2: ProjectionModel) -> Self {
        let mut all_sphere_verts = Vec::new();
        let mut all_faces = model1.faces.clone();
        let mut all_edges = EdgeList::new();

        // origin vertices of two models
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

        // calcuation intersection vertices, split & add edges
        for e in model1.edges.iter() {
            all_edges.add(e.from, e.to);
        }
        for e2 in model2.edges.iter() {
            let e2 = Edge::new(e2.from + n, e2.to + n);
            let v1 = all_sphere_verts[e2.from].v;
            let v2 = all_sphere_verts[e2.to].v;
            let arc2 = Arc::new(v1, v2, e2.from, e2.to);
            let mut ints = vec![(0.0, e2.from), (1.0, e2.to)];

            for e1 in &mut all_edges.clone().iter() {
                let u1 = all_sphere_verts[e1.from].v;
                let u2 = all_sphere_verts[e1.to].v;
                let arc1 = Arc::new(u1, u2, e1.from, e1.to);

                match Arc::intersect(&arc1, &arc2) {
                    ArcIntersectionResult::T1(index, k) => ints.push((k, index)),
                    ArcIntersectionResult::T2(index, k) => {
                        all_edges.remove(e1);
                        all_edges.add(e1.from, index);
                        all_edges.add(e1.to, index);
                        ints.push((k, index))
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
                    ArcIntersectionResult::L(id1, id2) => {
                        if id2 == e2.from {
                            ints[0].1 = id1
                        } else if id2 == e2.to {
                            ints[1].1 = id1
                        }
                    }
                    ArcIntersectionResult::N => {}
                }
                if ints.len() > 2 {
                    ints.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
                    for i in 0..ints.len() - 1 {
                        all_edges.add(ints[i].1, ints[i + 1].1);
                    }
                }
            }
        }
        println!("SIZE {:?} {:?}", all_sphere_verts.len(), all_edges.len());

        // TODO: face tracing
        for f in &model2.faces {
            all_faces.push(vec![f[0] + n, f[1] + n, f[2] + n])
        }

        // project back to the model
        let mut model_vert_pairs = Vec::new();
        for v in all_sphere_verts {
            match v.from {
                1 => {
                    model_vert_pairs.push((model1.verts[v.index], model2.project_from_sphere(v.v)))
                }
                2 => {
                    model_vert_pairs.push((model1.project_from_sphere(v.v), model2.verts[v.index]))
                }
                _ => model_vert_pairs.push((
                    model1.project_from_sphere(v.v),
                    model2.project_from_sphere(v.v),
                )),
            }
        }

        MergedModel {
            vert_pairs: model_vert_pairs,
            faces: all_faces,
        }
    }

    pub fn interpolation(&self, ratio: f64) -> Model {
        let mut new_verts = Vec::new();
        for (v1, v2) in &self.vert_pairs {
            new_verts.push(*v1 + (*v2 - *v1) * ratio);
        }
        Model::new(new_verts, self.faces.clone())
    }
}
