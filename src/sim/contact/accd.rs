use super::{
    super::body::body::Body,
    affine_contact::{ContactElem, ContactNode},
    boundary::{self, Boundary},
};
use crate::{
    pause,
    sim::{
        sim::Simulation,
        utils::{affine_utils::interop::InteropCol, misc::dof_index, types::Vec6},
    },
};
use faer::{dbgf, Col};

#[derive(Clone, Debug)]
pub struct CcdPair {
    pub p: glm::Vec2,
    pub e: (glm::Vec2, glm::Vec2),
}

impl CcdPair {
    pub fn distance(&self) -> f32 {
        let p = self.p;
        let (a, b) = (&self.e.0, &self.e.1);

        let ab = b - a;
        let ap = p - a;
        let bp = p - b;

        // Check if the point is closest to the start of the edge
        if ab.dot(&ap) <= 0.0 {
            return ap.norm();
        }

        // Check if the point is closest to the end of the edge
        if ab.dot(&bp) >= 0.0 {
            return bp.norm();
        }

        // The point is closest to the edge itself
        let ap_proj_on_ab = ap.dot(&ab) / ab.norm_squared();
        let proj_point = a + ap_proj_on_ab * ab;
        (p - proj_point).norm()
    }
}

#[derive(Debug)]
pub struct CcdDir {
    pub p: glm::Vec2,
    pub e: (glm::Vec2, glm::Vec2),
}

/// Erases **dof-index** info, keep only position info
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
        cpos: &CcdPair, // x
        cdir: &CcdDir,  // p
    ) -> f32 {
        let pbar = (cdir.p + cdir.e.0 + cdir.e.1) / 3f32;

        let mut x = cpos.clone();
        let p = CcdDir {
            p: cdir.p - pbar,
            e: (cdir.e.0 - pbar, cdir.e.1 - pbar),
        };

        let l_p = p.p.magnitude() + p.e.0.magnitude().max(p.e.1.magnitude());
        if l_p == 0f32 {
            return 1f32;
        }

        // let dsqr = pair.distance().powi(2);
        let d = x.distance();
        let g = self.s * d;

        let mut t = 0f32;
        let mut t_l = (1f32 - self.s) * d / l_p;

        for _ in 0..self.max_iter {
            x.p += t_l * p.p;
            x.e.0 += t_l * p.e.0;
            x.e.1 += t_l * p.e.1;

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
    /// - given current `dof` and line search `dir`.
    /// Compute Ccd pair (not contact pair)
    pub fn toi(&self, sim: &Simulation, dof: &Col<f32>, dir: &Col<f32>) -> f32 {
        let mut t = 1f32;
        let accd = Accd {
            s: self.s,
            t_c: self.t_c,
            max_iter: self.max_iter,
        };

        // 1. bodies and boundaries
        // - springsbody
        // - affinebody
        for body in &sim.bodies {
            match body {
                Body::Affine(ab, offset) => {
                    // unique for each body
                    let q = Vec6::from_dof(dof, *offset);
                    let qdir = Vec6::from_dof(dir, *offset);
                    for i in 0..ab.nvert {
                        // unique for each vertex
                        let p = ab.pos(&q, i);
                        let pdir = ab.pos_delta(&q, &qdir, i);

                        for edge in Boundary::edges_extended() {
                            let cpos = CcdPair { p, e: edge };
                            let cdir = CcdDir {
                                p: pdir,
                                e: (glm::Vec2::zeros(), glm::Vec2::zeros()),
                            };

                            t = t.min(accd.toi(&cpos, &cdir));
                        }
                    }
                }
                Body::Soft() => todo!(),
                Body::Springs(spbody, offset) => {
                    for inode in 0..spbody.ndof / 2 {
                        let (ix, iy) = dof_index(inode, *offset);
                        let node = glm::vec2(dof[ix], dof[iy]);
                        let node_dir = glm::vec2(dir[ix], dir[iy]);

                        for edge in Boundary::edges_extended() {
                            let cpos = CcdPair { p: node, e: edge };
                            let cdir = CcdDir {
                                p: node_dir,
                                e: (glm::vec2(0f32, 0f32), glm::vec2(0f32, 0f32)),
                            };

                            // let toi = accd.toi(&cpos, &cdir);

                            // compute toi for single pair
                            t = t.min(accd.toi(&cpos, &cdir));
                        }
                    }
                }
            }
        }

        // 2. inter-body
        for i in 0..sim.bodies.len() {
            for j in i + 1..sim.bodies.len() {
                let b1 = &sim.bodies[i];
                let b2 = &sim.bodies[j];

                // springbody inter-body
                if let Body::Springs(spb1, off1) = b1 {
                    if let Body::Springs(spb2, off2) = b2 {
                        // p1 collide with e2
                        for ip in 0..spb1.ndof / 2 {
                            for c in &spb2.constraints {
                                let (ipx, ipy) = dof_index(ip, *off1);
                                let (ie1x, ie1y) = dof_index(c.i1, *off2);
                                let (ie2x, ie2y) = dof_index(c.i2, *off2);
                                // pairs and dirs
                                let cpos = CcdPair {
                                    p: glm::vec2(dof[ipx], dof[ipy]),
                                    e: (
                                        glm::vec2(dof[ie1x], dof[ie1y]),
                                        glm::vec2(dof[ie2x], dof[ie2y]),
                                    ),
                                };
                                let cdir = CcdDir {
                                    p: glm::vec2(dir[ipx], dir[ipy]),
                                    e: (
                                        glm::vec2(dir[ie1x], dir[ie1y]),
                                        glm::vec2(dir[ie2x], dir[ie2y]),
                                    ),
                                };
                                t = t.min(accd.toi(&cpos, &cdir));
                            }
                        }

                        // p2 collide with e1
                        for ip in 0..spb2.ndof / 2 {
                            for c in &spb1.constraints {
                                let (ipx, ipy) = dof_index(ip, *off2);
                                let (ie1x, ie1y) = dof_index(c.i1, *off1);
                                let (ie2x, ie2y) = dof_index(c.i2, *off1);
                                // pairs and dirs
                                let cpos = CcdPair {
                                    p: glm::vec2(dof[ipx], dof[ipy]),
                                    e: (
                                        glm::vec2(dof[ie1x], dof[ie1y]),
                                        glm::vec2(dof[ie2x], dof[ie2y]),
                                    ),
                                };
                                let cdir = CcdDir {
                                    p: glm::vec2(dir[ipx], dir[ipy]),
                                    e: (
                                        glm::vec2(dir[ie1x], dir[ie1y]),
                                        glm::vec2(dir[ie2x], dir[ie2y]),
                                    ),
                                };
                                t = t.min(accd.toi(&cpos, &cdir));
                            }
                        }
                    }
                }

                // affinebody inter-body
                if let Body::Affine(ab1, off1) = b1 {
                    if let Body::Affine(ab2, off2) = b2 {
                        // extract q1 and q2
                        let q1 = Vec6::from_dof(dof, *off1);
                        let dq1 = Vec6::from_dof(dir, *off1);
                        let q2 = Vec6::from_dof(dof, *off2);
                        let dq2 = Vec6::from_dof(dir, *off2);

                        // p1 - e2
                        for ip in 0..ab1.nvert {
                            let p = ab1.pos(&q1, ip);
                            let dp = ab1.pos_delta(&q1, &dq1, ip);
                            for (e, de) in ab2.edges_and_deltas(&q2, &dq2) {
                                let cpos = CcdPair { p, e };
                                let cdir = CcdDir { p: dp, e: de };

                                t = t.min(accd.toi(&cpos, &cdir));
                                // debug
                                // let toi = accd.toi(&cpos, &cdir);
                                // if toi < 1.0 {
                                //     println!("collision detected!");
                                //     println!("p1-e2 collided on toi = {toi}");
                                //     dbg!(&cpos);
                                //     dbg!(&cdir);
                                //     pause();
                                // }
                            }
                        }

                        // p2 - e1
                        for ip in 0..ab2.nvert {
                            let p = ab2.pos(&q2, ip);
                            let dp = ab2.pos_delta(&q2, &dq2, ip);
                            for (e, de) in ab1.edges_and_deltas(&q1, &dq1) {
                                let cpos = CcdPair { p, e };
                                let cdir = CcdDir { p: dp, e: de };

                                t = t.min(accd.toi(&cpos, &cdir));
                                // debug
                                // let toi = accd.toi(&cpos, &cdir);
                                // if toi < 1.0 {
                                //     println!("collision detected!");
                                //     println!("p2-e1 collided on toi = {toi}");
                                //     dbg!(&cpos);
                                //     dbg!(&cdir);
                                //     pause();
                                // }
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
