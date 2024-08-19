use super::body::Ip;
use crate::sim::utils::hess::Hess;
use crate::RunConfig;
use faer::{unzipped, zipped, Col};
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

pub struct StaticBody {
    // all data required to compute the next configuration in optimization
    pub ndof: usize,

    // update
    pub x: Col<f32>,
    pub edges: Vec<(usize, usize)>,
}

impl StaticBody {
    pub fn new(n: usize) -> Self {
        let ndof = n * 2;
        Self {
            ndof,
            x: Col::zeros(ndof),
            edges: Vec::new(),
        }
    }
    pub fn from_file(path: &str) -> io::Result<Self> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let mut points = Vec::new();
        let mut edges = Vec::new();
        let mut section = None;

        for line in reader.lines() {
            let line = line?;
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            if line.starts_with("!") {
                match line {
                    "!points" => section = Some("points"),
                    "!edges" => section = Some("edges"),
                    "!end" => break,
                    _ => {
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            "Unknown section",
                        ))
                    }
                }
                continue;
            }

            match section {
                Some("points") => {
                    let coords: Vec<f32> = line
                        .split_whitespace()
                        .map(|s| {
                            s.parse().map_err(|_| {
                                io::Error::new(io::ErrorKind::InvalidData, "Invalid point format")
                            })
                        })
                        .collect::<Result<Vec<f32>, _>>()?;
                    if coords.len() != 2 {
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            "Each point must have exactly 2 coordinates",
                        ));
                    }
                    points.push((coords[0], coords[1]));
                }
                Some("edges") => {
                    let indices: Vec<usize> = line
                        .split_whitespace()
                        .map(|s| {
                            s.parse().map_err(|_| {
                                io::Error::new(io::ErrorKind::InvalidData, "Invalid edge format")
                            })
                        })
                        .collect::<Result<Vec<usize>, _>>()?;
                    if indices.len() != 2 {
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            "Each edge must have exactly 2 indices",
                        ));
                    }
                    edges.push((indices[0], indices[1]));
                }
                _ => {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "Unexpected data outside of sections",
                    ))
                }
            }
        }

        if points.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "No points found",
            ));
        }

        let ndof = points.len() * 2;
        let mut x = Col::zeros(ndof);
        for (i, &(px, py)) in points.iter().enumerate() {
            x[(i * 2)] = px;
            x[(i * 2 + 1)] = py;
        }

        Ok(Self { ndof, x, edges })
    }
}
