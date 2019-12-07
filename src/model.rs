use std::fs::File;
use std::io::{self, prelude::*, BufReader, BufWriter};
use std::vec::Vec;

#[derive(Debug)]
pub struct Vertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

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

impl Vertex {
    fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

impl Face3 {
    fn new(a: i32, b: i32, c: i32) -> Self {
        Self { a, b, c }
    }
}

impl Model {
    pub fn load(filename: &str) -> io::Result<Model> {
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
        Ok(Model { verts, faces })
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
