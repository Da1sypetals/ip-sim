use super::super::body::body::Body;
use crate::sim::{
    sim::{Boundary, Simulation},
    utils::misc::dof_index,
};
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
    max_iter: u32,
}

impl Accd {
    pub fn new(s: f32, max_iter: u32) -> Self {
        Self {
            s,
            t_c: 1f32,
            max_iter,
        }
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

        for _ in 0..self.max_iter {
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
    pub max_iter: u32,
}

impl AccdMassive {
    pub fn new(s: f32, max_iter: u32) -> Self {
        Self {
            s,
            t_c: 1f32,
            max_iter,
        }
    }

    /// Compute the `toi` for the whole simulation
    /// - given current `dof` and line search `dir`
    /// Currently only springbody and boundaries
    pub fn toi(&self, sim: &Simulation, dof: &Col<f32>, dir: &Col<f32>) -> f32 {
        let mut t = 1f32;
        let accd = Accd {
            s: self.s,
            t_c: self.t_c,
            max_iter: self.max_iter,
        };

        // 1. springbody and boundaries
        for body in &sim.bodies {
            match body {
                Body::Affine() => todo!(),
                Body::Soft() => todo!(),
                Body::Springs(spbody, offset) => {
                    for inode in 0..spbody.ndof / 2 {
                        let (ix, iy) = dof_index(inode, *offset);
                        for edge in Boundary::edges() {
                            let node = glm::vec2(dof[ix], dof[iy]);
                            let node_dir = glm::vec2(dir[ix], dir[iy]);
                            let index = ContactIndex {
                                p: Some((inode * 2, inode * 2 + 1)),
                                e: None,
                            };
                            let cpair = ContactPair {
                                point: node,
                                edge,
                                index,
                            };
                            let cdir = ContactPairDir {
                                point: node_dir,
                                edge: (glm::vec2(0f32, 0f32), glm::vec2(0f32, 0f32)),
                                index,
                            };

                            // compute toi for single pair
                            t = t.min(accd.toi(&cpair, &cdir));
                        }
                    }
                }
            }
        }

        // 2. springbody inter-body
        for i in 0..sim.bodies.len() {
            for j in i + 1..sim.bodies.len() {
                let b1 = &sim.bodies[i];
                let b2 = &sim.bodies[j];
                if let Body::Springs(spb1, off1) = b1 {
                    if let Body::Springs(spb2, off2) = b2 {
                        // p1 collide with e2
                        for ip in 0..spb1.ndof / 2 {
                            for c in &spb2.constraints {
                                let (ipx, ipy) = dof_index(ip, *off1);
                                let (ie1x, ie1y) = dof_index(c.i1, *off2);
                                let (ie2x, ie2y) = dof_index(c.i2, *off2);
                                // pairs and dirs
                                let cpair = ContactPair {
                                    point: glm::vec2(dof[ipx], dof[ipy]),
                                    edge: (
                                        glm::vec2(dof[ie1x], dof[ie1y]),
                                        glm::vec2(dof[ie2x], dof[ie2y]),
                                    ),
                                    index: ContactIndex {
                                        p: Some((ipx, ipy)),
                                        e: Some(((ie1x, ie1y), (ie2x, ie2y))),
                                    },
                                };
                                let cdir = ContactPairDir {
                                    point: glm::vec2(dir[ipx], dir[ipy]),
                                    edge: (
                                        glm::vec2(dir[ie1x], dir[ie1y]),
                                        glm::vec2(dir[ie2x], dir[ie2y]),
                                    ),
                                    index: cpair.index,
                                };
                                t = t.min(accd.toi(&cpair, &cdir));
                            }
                        }

                        // p2 collide with e1
                        for ip in 0..spb2.ndof / 2 {
                            for c in &spb1.constraints {
                                let (ipx, ipy) = dof_index(ip, *off2);
                                let (ie1x, ie1y) = dof_index(c.i1, *off1);
                                let (ie2x, ie2y) = dof_index(c.i2, *off1);
                                // pairs and dirs
                                let cpair = ContactPair {
                                    point: glm::vec2(dof[ipx], dof[ipy]),
                                    edge: (
                                        glm::vec2(dof[ie1x], dof[ie1y]),
                                        glm::vec2(dof[ie2x], dof[ie2y]),
                                    ),
                                    index: ContactIndex {
                                        p: Some((ipx, ipy)),
                                        e: Some(((ie1x, ie1y), (ie2x, ie2y))),
                                    },
                                };
                                let cdir = ContactPairDir {
                                    point: glm::vec2(dir[ipx], dir[ipy]),
                                    edge: (
                                        glm::vec2(dir[ie1x], dir[ie1y]),
                                        glm::vec2(dir[ie2x], dir[ie2y]),
                                    ),
                                    index: cpair.index,
                                };
                                t = t.min(accd.toi(&cpair, &cdir));
                            }
                        }
                    }
                }
            }
        }

        // todo!()
        t
    }
}
