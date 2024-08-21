use std::ops::Bound;

use super::super::body::body::Body;
use crate::sim::sim::{Boundary, Simulation};
use faer::Col;

use super::contact::{ContactIndex, ContactPair};
pub struct ContactPairDir {
    pub edge: (glm::Vec2, glm::Vec2),
    pub point: glm::Vec2,

    pub index: ContactIndex,
}

pub struct Accd {
    pub s: f32,
    pub t_c: f32,
}

impl Accd {
    pub fn new(s: f32) -> Self {
        Self { s, t_c: 1f32 }
    }

    /// See: https://ipc-sim.github.io/C-IPC/
    /// - currently `xi` is not supported
    pub fn toi(
        &self,
        pair: &ContactPair,   // x
        dir: &ContactPairDir, // p
    ) -> f32 {
        let pbar = (dir.point + dir.edge.0 + dir.edge.1) / 3f32;

        let mut x = pair.clone();
        let p = ContactPairDir {
            point: dir.point - pbar,
            edge: (dir.edge.0 - pbar, dir.edge.1 - pbar),
            index: pair.index,
        };

        let l_p = p.point.magnitude() + p.edge.0.magnitude().max(p.edge.1.magnitude());
        if l_p == 0f32 {
            return 1f32;
        }

        // let dsqr = pair.distance().powi(2);
        let d = x.distance();
        let g = self.s * d;

        let mut t = 0f32;
        let mut t_l = (1f32 - self.s) * d / l_p;

        loop {
            x.point += t_l * p.point;
            x.edge.0 += t_l * p.edge.0;
            x.edge.1 += t_l * p.edge.1;

            let d = x.distance();
            if t > 0f32 && d < g {
                return t;
            }

            t += t_l;
            if t > self.t_c {
                return self.t_c;
            }
            t_l = 0.9f32 * d / l_p;
        }

        t
    }
}

pub struct AccdMassive {
    pub s: f32,
    pub t_c: f32,
}

impl AccdMassive {
    pub fn new(s: f32) -> Self {
        Self { s, t_c: 1f32 }
    }

    /// Compute the `toi` for the whole simulation
    /// - given current `dof` and line search `dir`
    /// Currently only springbody and boundaries
    pub fn toi(&self, sim: &Simulation, dof: &Col<f32>, dir: &Col<f32>) -> f32 {
        let mut t = 1f32;
        let accd = Accd {
            s: self.s,
            t_c: self.t_c,
        };

        for body in &sim.bodies {
            match body {
                Body::Affine() => todo!(),
                Body::Soft() => todo!(),
                Body::Springs(spbody, offset) => {
                    for inode in 0..spbody.ndof / 2 {
                        for edge in Boundary::edges() {
                            let node = glm::vec2(dof[inode * 2], dof[inode * 2 + 1]);
                            let node_dir = glm::vec2(dir[inode * 2], dir[inode * 2 + 1]);
                            let index = ContactIndex {
                                p: Some((inode * 2, inode * 2 + 1)),
                                e: None,
                            };
                            let pair = ContactPair {
                                point: node,
                                edge,
                                index,
                            };
                            let dir = ContactPairDir {
                                point: node_dir,
                                edge: (glm::vec2(0f32, 0f32), glm::vec2(0f32, 0f32)),
                                index,
                            };

                            // compute toi for single pair
                            t = t.min(accd.toi(&pair, &dir));
                        }
                    }
                }
            }
        }

        todo!()
    }
}
