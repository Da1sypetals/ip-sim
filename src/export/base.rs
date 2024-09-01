use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{self, Read, Write};

use crate::sim::body::{body::Body, springsbody::SpringsBody};
use crate::sim::sim::Simulation;

pub trait Export {
    fn export_nodes(&self) -> Vec<(f32, f32)>;
    fn export_edges(&self) -> Vec<((f32, f32), (f32, f32))>;
}

impl Export for Simulation {
    fn export_nodes(&self) -> Vec<(f32, f32)> {
        let mut nodes = Vec::new();
        for body in &self.bodies {
            nodes.extend(body.export_nodes());
        }

        nodes
    }

    fn export_edges(&self) -> Vec<((f32, f32), (f32, f32))> {
        let mut edges = Vec::new();
        for body in &self.bodies {
            edges.extend(body.export_edges());
        }

        edges
    }
}

impl Export for Body {
    fn export_nodes(&self) -> Vec<(f32, f32)> {
        match self {
            Body::Affine(ab, _) => ab.export_nodes(),
            Body::Soft() => todo!(),
            Body::Springs(spbody, _) => spbody.export_nodes(),
        }
    }
    fn export_edges(&self) -> Vec<((f32, f32), (f32, f32))> {
        match self {
            Body::Affine(ab, _) => ab.export_edges(),
            Body::Soft() => todo!(),
            Body::Springs(spbody, _) => spbody.export_edges(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct VisFrame {
    pub nodes: Vec<(f32, f32)>,
    pub edges: Vec<((f32, f32), (f32, f32))>,
}

#[derive(Serialize, Deserialize)]
pub struct VisFrameVec {
    pub frames: Vec<VisFrame>,
}

impl VisFrameVec {
    pub fn save(&self, file_path: &str) -> io::Result<()> {
        let json_data =
            serde_json::to_string(self).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        let mut file = File::create(file_path)?;
        file.write_all(json_data.as_bytes())?;
        Ok(())
    }

    pub fn load(file_path: &str) -> io::Result<Self> {
        let mut file = File::open(file_path)?;
        let mut json_data = String::new();
        file.read_to_string(&mut json_data)?;
        let vis_frame_vec = serde_json::from_str(&json_data)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        Ok(vis_frame_vec)
    }
}

/// Serialization utils for Simulation
impl Simulation {
    pub fn export(&mut self) {
        let frame = VisFrame {
            nodes: self.export_nodes(),
            edges: self.export_edges(),
        };
        self.vis_frames.frames.push(frame);
    }

    pub fn save(&self, path: &str) -> io::Result<()> {
        self.vis_frames.save(path)?;
        Ok(())
    }
}
