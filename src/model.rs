use std::collections::HashSet;
use std::fs::File;
use std::io::{self, prelude::*, BufReader, BufWriter};
use std::vec::Vec;

use crate::geo::{Arc, ArcIntersectionResult, Triangle};
use crate::Vertex;

const SPHERE_RADIUS: f32 = 100.0;

#[derive(Debug, Clone)]
pub struct Face3 {
    a: usize,
    b: usize,
    c: usize,
}

#[derive(Debug, Clone)]
struct EdgeList(HashSet<(usize, usize)>);

impl std::ops::Deref for EdgeList {
    type Target = HashSet<(usize, usize)>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for EdgeList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug)]
pub struct Model {
    verts: Vec<Vertex>,
    faces: Vec<Face3>,
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
    faces: Vec<Face3>,
    vert_pairs: Vec<(Vertex, Vertex)>,
}

impl Face3 {
    pub fn new(a: usize, b: usize, c: usize) -> Self {
        Self { a, b, c }
    }
}

impl Model {
    pub fn nr_verts(&self) -> usize {
        self.verts.len()
    }

    pub fn nr_faces(&self) -> usize {
        self.faces.len()
    }

    pub fn new(verts: Vec<Vertex>, faces: Vec<Face3>) -> Self {
        Self { verts, faces }
    }

    pub fn load(filename: &str) -> io::Result<Self> {
        assert!(filename.ends_with(".obj"));

        let file = File::open(filename)?;
        let reader = BufReader::new(file);

        let mut verts = Vec::<Vertex>::new();
        let mut faces = Vec::<Face3>::new();
        for line in reader.lines() {
            let line = line?;
            let vals = line.split_whitespace().collect::<Vec<_>>();
            if vals.len() == 0 {
                continue;
            }
            match vals[0] {
                "v" => verts.push(Vertex::new(
                    vals[1].parse().unwrap(),
                    vals[2].parse().unwrap(),
                    vals[3].parse().unwrap(),
                )),
                "f" => faces.push(Face3::new(
                    vals[1].parse::<usize>().unwrap() - 1,
                    vals[2].parse::<usize>().unwrap() - 1,
                    vals[3].parse::<usize>().unwrap() - 1,
                )),
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
            writeln!(writer, "f {} {} {}", f.a + 1, f.b + 1, f.c + 1)?;
        }
        Ok(())
    }
}

impl EdgeList {
    fn new() -> Self {
        Self(HashSet::<(usize, usize)>::new())
    }

    fn add(&mut self, from: usize, to: usize) -> bool {
        if from == to {
            false
        } else if from < to {
            self.0.insert((from, to))
        } else {
            self.0.insert((to, from))
        }
    }
}

impl ProjectionModel {
    pub fn new(model: Model) -> Self {
        let mut center = Vertex::new(0.0, 0.0, 0.0);
        for v in &model.verts {
            center += *v;
        }
        center /= model.nr_verts() as f32;

        let mut sphere_verts = Vec::<Vertex>::new();
        for v in &model.verts {
            sphere_verts.push(v.project_to_sphere(center, SPHERE_RADIUS));
        }

        let mut edges = EdgeList::new();
        for f in &model.faces {
            edges.add(f.a, f.b);
            edges.add(f.b, f.c);
            edges.add(f.c, f.a);
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
            let tri = Triangle::new(self.verts[f.a], self.verts[f.b], self.verts[f.c]);
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
        let mut all_sphere_verts = Vec::<SphereVertex>::new();
        let mut all_faces = model1.faces.clone();
        let mut all_edges = EdgeList::new();

        // origin vertices of two models
        let n = model1.nr_verts();
        let m = model2.nr_verts();
        for i in 0..n {
            all_sphere_verts.push(SphereVertex {
                v: model1.verts[i],
                from: 1,
                index: i,
            });
        }
        for i in 0..m {
            all_sphere_verts.push(SphereVertex {
                v: model2.verts[i],
                from: 2,
                index: i,
            });
        }

        // calcuation intersection vertices, split & add edges
        for e in model1.edges.iter() {
            all_edges.add(e.0, e.1);
        }
        for e2 in model2.edges.iter() {
            let e2 = (e2.0 + n, e2.1 + n);
            let v1 = all_sphere_verts[e2.0].v;
            let v2 = all_sphere_verts[e2.1].v;
            let arc2 = Arc::new(v1, v2, e2.0, e2.1);
            let mut ints = vec![(0.0, e2.0), (1.0, e2.1)];
            for e1 in all_edges.clone().iter() {
                let u1 = all_sphere_verts[e1.0].v;
                let u2 = all_sphere_verts[e1.1].v;
                let arc1 = Arc::new(u1, u2, e1.0, e1.1);
                match Arc::intersect(&arc1, &arc2) {
                    ArcIntersectionResult::T1(index, ratio) => ints.push((ratio, index)),
                    ArcIntersectionResult::T2(index, ratio) => {
                        all_edges.remove(e1);
                        all_edges.add(e1.0, index);
                        all_edges.add(e1.1, index);
                        ints.push((ratio, index))
                    }
                    ArcIntersectionResult::X(v, ratio) => {
                        let id = all_sphere_verts.len();
                        all_sphere_verts.push(SphereVertex {
                            v,
                            from: 0,
                            index: 0,
                        });
                        all_edges.remove(e1);
                        all_edges.add(e1.0, id);
                        all_edges.add(e1.1, id);
                        ints.push((ratio, id))
                    }
                    _ => {}
                }
                // println!("{:?} {:?}",e1, e2);
                if ints.len() > 2 {
                    ints.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
                    for i in 0..ints.len() - 1 {
                        all_edges.add(ints[i].1, ints[i + 1].1);
                    }
                }
            }
        }

        // TODO: face adjustment
        for f in &model2.faces {
            all_faces.push(Face3::new(f.a + n, f.b + n, f.c + n))
        }

        // project back to the model
        let mut model_vert_pairs = Vec::<(Vertex, Vertex)>::new();
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

    pub fn interpolation(&self, ratio: f32) -> Model {
        let mut new_verts = Vec::<Vertex>::new();
        for (v1, v2) in &self.vert_pairs {
            new_verts.push(*v1 * (1.0 - ratio) + *v2 * ratio)
        }
        Model::new(new_verts, self.faces.clone())
    }
}
