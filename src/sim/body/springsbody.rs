use super::body::Ip;
use crate::sim::utils::hess::Hess;
use crate::RunConfig;
use faer::{unzipped, zipped, Col};
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

#[derive(Clone)]
pub struct Constraint {
    pub i1: usize,
    pub i2: usize,
    pub k: f32,
    pub l0: f32,
}

impl Constraint {
    pub fn new(i1: usize, i2: usize, k: f32, l0: f32) -> Self {
        Constraint { i1, i2, k, l0 }
    }
}

pub struct SpringsBody {
    // all data required to compute the next configuration in optimization
    pub ndof: usize,

    // update
    pub x: Col<f32>,
    pub xhat: Col<f32>,
    pub xprev: Col<f32>,
    pub v: Col<f32>,

    // constants
    pub constraints: Vec<Constraint>,
}

impl SpringsBody {
    pub fn new(n: usize) -> Self {
        let ndof = n * 2;
        SpringsBody {
            ndof,
            x: Col::zeros(ndof),
            xhat: Col::zeros(ndof),
            xprev: Col::zeros(ndof),
            v: Col::zeros(ndof),
            constraints: Vec::new(),
        }
    }

    pub fn from_file_with_v0(path: &str) -> io::Result<Self> {
        let file = File::open(Path::new(path))?;
        let reader = BufReader::new(file);
        let mut lines = reader.lines();

        let mut nodes = Vec::new();
        let mut velocities = Vec::new();
        let mut constraints = Vec::new();
        let mut k_value = 0.0; // Default k value

        while let Some(line) = lines.next() {
            let line = line?;
            let line = line.trim();

            if line.is_empty() {
                continue; // Skip empty lines
            }

            if line.starts_with("!k") {
                k_value = line.split_whitespace().nth(1).unwrap().parse().unwrap();
            } else if line.starts_with("!node") {
                while let Some(node_line) = lines.next() {
                    let node_line = node_line?;
                    let node_line = node_line.trim();
                    if node_line.is_empty() || node_line.starts_with("!v0") {
                        break;
                    }
                    let coords: Vec<f32> = node_line
                        .split_whitespace()
                        .map(|s| s.parse().unwrap())
                        .collect();
                    nodes.push((coords[0], coords[1]));
                }
            } else if line.starts_with("!v0") {
                while let Some(v0_line) = lines.next() {
                    let v0_line = v0_line?;
                    let v0_line = v0_line.trim();
                    if v0_line.is_empty() || v0_line.starts_with("!spring") {
                        break;
                    }
                    let vels: Vec<f32> = v0_line
                        .split_whitespace()
                        .map(|s| s.parse().unwrap())
                        .collect();
                    velocities.push((vels[0], vels[1]));
                }
            } else if line.starts_with("!spring") {
                while let Some(spring_line) = lines.next() {
                    let spring_line = spring_line?;
                    let spring_line = spring_line.trim();
                    if spring_line.is_empty() || spring_line.starts_with("!end") {
                        break;
                    }
                    let indices: Vec<usize> = spring_line
                        .split_whitespace()
                        .map(|s| s.parse().unwrap())
                        .collect();
                    let i1 = indices[0];
                    let i2 = indices[1];
                    let (x1, y1) = nodes[i1];
                    let (x2, y2) = nodes[i2];
                    let l0 = ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt();
                    let constraint = Constraint {
                        i1,
                        i2,
                        k: k_value,
                        l0,
                    };
                    constraints.push(constraint);
                }
            } else if line.starts_with("!end") {
                break;
            }
        }

        let n = nodes.len();
        let mut body = SpringsBody::new(n);

        for (i, &(x, y)) in nodes.iter().enumerate() {
            body.x[(i * 2)] = x;
            body.x[(i * 2) + 1] = y;
        }

        for (i, &(vx, vy)) in velocities.iter().enumerate() {
            body.v[(i * 2)] = vx;
            body.v[(i * 2) + 1] = vy;
        }

        body.constraints = constraints;

        Ok(body)
    }

    pub fn from_file(path: &str) -> io::Result<Self> {
        let file = File::open(Path::new(path))?;
        let reader = BufReader::new(file);
        let mut lines = reader.lines();

        let mut nodes = Vec::new();
        let mut constraints = Vec::new();
        let mut k_value = 0.0; // Default k value

        while let Some(line) = lines.next() {
            let line = line?;
            let line = line.trim();

            if line.is_empty() {
                continue; // Skip empty lines
            }

            if line.starts_with("!k") {
                k_value = line.split_whitespace().nth(1).unwrap().parse().unwrap();
            } else if line.starts_with("!node") {
                while let Some(node_line) = lines.next() {
                    let node_line = node_line?;
                    let node_line = node_line.trim();
                    if node_line.is_empty() || node_line.starts_with("!spring") {
                        break;
                    }
                    let coords: Vec<f32> = node_line
                        .split_whitespace()
                        .map(|s| s.parse().unwrap())
                        .collect();
                    nodes.push((coords[0], coords[1]));
                }
            } else if line.starts_with("!spring") {
                while let Some(spring_line) = lines.next() {
                    let spring_line = spring_line?;
                    let spring_line = spring_line.trim();
                    if spring_line.is_empty() || spring_line.starts_with("!end") {
                        break;
                    }
                    let indices: Vec<usize> = spring_line
                        .split_whitespace()
                        .map(|s| s.parse().unwrap())
                        .collect();
                    let i1 = indices[0];
                    let i2 = indices[1];
                    let (x1, y1) = nodes[i1];
                    let (x2, y2) = nodes[i2];
                    let l0 = ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt();
                    let constraint = Constraint {
                        i1,
                        i2,
                        k: k_value,
                        l0,
                    };
                    constraints.push(constraint);
                }
            } else if line.starts_with("!end") {
                break;
            }
        }

        let n = nodes.len();
        let mut body = SpringsBody::new(n);

        for (i, &(x, y)) in nodes.iter().enumerate() {
            body.x[(i * 2)] = x;
            body.x[(i * 2) + 1] = y;
        }

        body.constraints = constraints;

        Ok(body)
    }
}

// Ip should be a tree, where each Ip instantiate its sub-ip.
pub struct SpringsBodyIp {
    pub run_cfg: RunConfig,
    pub inertial: SpringInertialIp,
    pub spring: SpringIp,
}

impl SpringsBodyIp {
    /// Preferred clone:
    /// `SpringsBodyIp::new(rc.clone())`
    pub fn new(rc: &RunConfig) -> Self {
        SpringsBodyIp {
            run_cfg: rc.clone(),
            inertial: SpringInertialIp {
                run_cfg: rc.clone(),
            },
            spring: SpringIp {
                run_cfg: rc.clone(),
            },
        }
    }
}

impl Ip for SpringsBodyIp {
    type BodyT = SpringsBody;

    fn prepare(&self, cfg: &mut Self::BodyT) {
        self.inertial.prepare(cfg);
        self.spring.prepare(cfg);
    }

    fn value(&self, cfg: &Self::BodyT, dof: &Col<f32>) -> f32 {
        self.inertial.value(cfg, dof) + self.spring.value(cfg, dof)
    }

    fn grad(&self, cfg: &Self::BodyT, dof: &Col<f32>, result: &mut Col<f32>) {
        self.inertial.grad(cfg, dof, result);
        self.spring.grad(cfg, dof, result);
    }

    fn hess(&self, cfg: &Self::BodyT, dof: &Col<f32>, result: &mut Hess) {
        self.inertial.hess(cfg, dof, result);
        self.spring.hess(cfg, dof, result);
    }
}

pub struct SpringIp {
    pub run_cfg: RunConfig,
}

pub struct SpringInertialIp {
    pub run_cfg: RunConfig,
}

impl Ip for SpringIp {
    type BodyT = SpringsBody;

    fn prepare(&self, body: &mut Self::BodyT) {}

    fn value(&self, body: &Self::BodyT, dof: &Col<f32>) -> f32 {
        let mut res = 0f32;
        for c in &body.constraints {
            let x1 = glm::vec2(dof[c.i1 * 2], dof[c.i1 * 2 + 1]);
            let x2 = glm::vec2(dof[c.i2 * 2], dof[c.i2 * 2 + 1]);
            let len = (x1 - x2).magnitude();

            res += c.k * 0.5f32 * (len - c.l0) * (len - c.l0);
        }
        self.run_cfg.dt * self.run_cfg.dt * res
    }

    fn grad(&self, body: &Self::BodyT, dof: &Col<f32>, result: &mut Col<f32>) {
        for c in &body.constraints {
            let x1 = glm::vec2(dof[c.i1 * 2], dof[c.i1 * 2 + 1]);
            let x2 = glm::vec2(dof[c.i2 * 2], dof[c.i2 * 2 + 1]);
            let x12 = x1 - x2;
            let len = x12.magnitude();
            let mut g = c.k * (len - c.l0) * x12 / len;
            // potential * dt^2
            g *= self.run_cfg.dt * self.run_cfg.dt;

            // modify result
            // 1
            result[c.i1 * 2] += g.x;
            result[c.i1 * 2 + 1] += g.y;
            // 2
            result[c.i2 * 2] += -g.x;
            result[c.i2 * 2 + 1] += -g.y;
        }
    }

    fn hess(&self, body: &Self::BodyT, dof: &Col<f32>, result: &mut Hess) {
        for c in &body.constraints {
            // grad
            let i1x = c.i1 * 2;
            let i1y = c.i1 * 2 + 1;
            let i2x = c.i2 * 2;
            let i2y = c.i2 * 2 + 1;

            let x1 = glm::vec2(dof[i1x], dof[i1y]);
            let x2 = glm::vec2(dof[i2x], dof[i2x]);

            let x12 = x1 - x2;
            let sqlen = x12.norm_squared();
            let xxt = x12 * x12.transpose();

            // hess
            let mut he = c.k / sqlen * xxt
                + c.k * (1f32 - c.l0 / x12.norm()) * (glm::identity() - xxt / sqlen);
            he *= self.run_cfg.dt * self.run_cfg.dt;

            // add all elements
            {
                // 1,1
                result.add_elem(i1x, i1x, he[(0, 0)]);
                result.add_elem(i1x, i1y, he[(0, 1)]);
                result.add_elem(i1y, i1x, he[(1, 0)]);
                result.add_elem(i1y, i1y, he[(1, 1)]);

                // 1,2
                result.add_elem(i1x, i2x, -he[(0, 0)]);
                result.add_elem(i1x, i2y, -he[(0, 1)]);
                result.add_elem(i1y, i2x, -he[(1, 0)]);
                result.add_elem(i1y, i2y, -he[(1, 1)]);

                // 2,1
                result.add_elem(i2x, i1x, -he[(0, 0)]);
                result.add_elem(i2x, i1y, -he[(0, 1)]);
                result.add_elem(i2y, i1x, -he[(1, 0)]);
                result.add_elem(i2y, i1y, -he[(1, 1)]);

                // 2,2
                result.add_elem(i2x, i2x, he[(0, 0)]);
                result.add_elem(i2x, i2y, he[(0, 1)]);
                result.add_elem(i2y, i2x, he[(1, 0)]);
                result.add_elem(i2y, i2y, he[(1, 1)]);
            }

            ()
        }
    }
}

impl Ip for SpringInertialIp {
    type BodyT = SpringsBody;

    fn prepare(&self, body: &mut Self::BodyT) {
        // compute xhat
        let mut g = Col::<f32>::zeros(body.ndof);
        let nele = body.ndof / 2;
        for i in 0..nele {
            g[i * 2] = 0f32;
            g[i * 2 + 1] = self.run_cfg.g;
        }
        zipped!(&mut body.xhat, &body.x, &body.v, &g).for_each(|unzipped!(mut xhat, x, v, g)| {
            *xhat = *x + self.run_cfg.dt * *v + self.run_cfg.dt * self.run_cfg.dt * *g;
        });
    }

    fn value(&self, body: &Self::BodyT, dof: &Col<f32>) -> f32 {
        0.5f32 * (dof - &body.xhat).squared_norm_l2()
    }

    fn grad(&self, body: &Self::BodyT, dof: &Col<f32>, result: &mut Col<f32>) {
        zipped!(result, dof, &body.xhat).for_each(|unzipped!(mut res, x, xhat)| {
            *res = *res + *x - *xhat;
        });
    }

    fn hess(&self, body: &Self::BodyT, dof: &Col<f32>, result: &mut Hess) {
        for i in 0..body.ndof {
            result.add_elem(i, i, 1f32)
        }
    }
}
