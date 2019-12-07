use std::fs::File;
use std::io::{self, prelude::*, BufReader, BufWriter};
use std::vec::Vec;

use crate::geo::Triangle;
use crate::Vertex;

const SPHERE_RADIUS: f32 = 100.0;

#[derive(Debug, Clone)]
pub struct Face3 {
    pub a: usize,
    pub b: usize,
    pub c: usize,
}

#[derive(Debug)]
pub struct Model {
    pub verts: Vec<Vertex>,
    pub faces: Vec<Face3>,
}

#[derive(Debug)]
pub struct ProjectionModel {
    pub model: Model,

    pub center: Vertex,
    pub sphere_verts: Vec<Vertex>,
}

impl std::ops::Deref for ProjectionModel {
    type Target = Model;

    fn deref(&self) -> &Model {
        &self.model
    }
}

#[derive(Debug)]
pub struct MergedModel {
    pub faces: Vec<Face3>,
    pub vert_pairs: Vec<(Vertex, Vertex)>,
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

        Self {
            model,
            center,
            sphere_verts,
        }
    }

    pub fn project_from_sphere(&self, v: Vertex) -> Vertex {
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
    pub fn new(model1: ProjectionModel, model2: ProjectionModel) -> Self {
        let mut all_sphere_verts = Vec::<SphereVertex>::new();
        let mut all_faces = model1.faces.clone();

        for i in 0..model1.nr_verts() {
            all_sphere_verts.push(SphereVertex {
                v: model1.verts[i],
                from: 1,
                index: i,
            });
        }
        for i in 0..model2.nr_verts() {
            all_sphere_verts.push(SphereVertex {
                v: model2.verts[i],
                from: 2,
                index: i,
            });
        }

        let n = model1.nr_verts();
        for f in &model2.faces {
            all_faces.push(Face3::new(f.a + n, f.b + n, f.c + n))
        }

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
