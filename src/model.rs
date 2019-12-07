use std::fs::File;
use std::io::{self, prelude::*, BufReader, BufWriter};
use std::vec::Vec;

use crate::Vertex;

const SPHERE_RADIUS: f32 = 100.0;

#[derive(Debug)]
pub struct Face3 {
    pub a: i32,
    pub b: i32,
    pub c: i32,
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

impl Face3 {
    pub fn new(a: i32, b: i32, c: i32) -> Self {
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
                    vals[1].parse().unwrap(),
                    vals[2].parse().unwrap(),
                    vals[3].parse().unwrap(),
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
            writeln!(writer, "f {} {} {}", f.a, f.b, f.c)?;
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
            sphere_verts.push(v.project_to_sphere(&center, SPHERE_RADIUS));
        }

        Self {
            model,
            center,
            sphere_verts,
        }
    }
}
