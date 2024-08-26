use super::{
    super::body::body::Body,
    affine_contact::{ContactElem, ContactNode},
    boundary::Boundary,
};
use crate::sim::{sim::Simulation, utils::misc::dof_index};
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
                        for edge in Boundary::edges_extended() {
                            let node = glm::vec2(dof[ix], dof[iy]);
                            let node_dir = glm::vec2(dir[ix], dir[iy]);
                            let cpos = CcdPair { p: node, e: edge };
                            let cdir = CcdDir {
                                p: node_dir,
                                e: (glm::vec2(0f32, 0f32), glm::vec2(0f32, 0f32)),
                            };

                            let toi = accd.toi(&cpos, &cdir);

                            // compute toi for single pair
                            t = t.min(accd.toi(&cpos, &cdir));
                        }
                    }
                }
            }
        }

        let mut cnt = 0;
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
                                cnt += 1;
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
                                cnt += 1;
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
            }
        }
        println!("pair count: {}", cnt);
        // todo!()
        t
    }
}
