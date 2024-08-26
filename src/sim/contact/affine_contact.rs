use faer::Col;
use std::ops::{Index, IndexMut};

use crate::{
    sim::{
        contact::generated::dist_node::{d_grad_e0_node, d_grad_e1_node},
        utils::{hess::Hess, misc::hess_spd_proj},
    },
    RunConfig,
};

use super::{accd::CcdPair, generated::dist_node::d_grad_p_node};

pub enum ContactNodeGrad {
    Static(),
    /// node position
    Node {
        p: glm::Vec2,
        index: (usize, usize),
    },
    /// a, t and initial pos
    Affine {
        t: glm::Vec2,
        a: glm::Mat2,
        /// (t1, t2, a11, a12, a21, a22)
        index: (usize, usize, usize, usize, usize, usize),
        // x0: glm::Vec2, // this does not have grad
    },
}

impl ContactNodeGrad {
    pub fn ndof(&self) -> usize {
        match self {
            ContactNodeGrad::Static() => 0,
            ContactNodeGrad::Node { p, index } => 2,
            ContactNodeGrad::Affine { t, a, index } => 6,
        }
    }
    pub fn scaled_by(&self, scalar: f32) -> ContactNodeGrad {
        match self {
            ContactNodeGrad::Static() => ContactNodeGrad::Static(),
            ContactNodeGrad::Node { p, index } => ContactNodeGrad::Node {
                p: p * scalar,
                index: index.clone(),
            },
            ContactNodeGrad::Affine { t, a, index } => ContactNodeGrad::Affine {
                t: t * scalar,
                a: a * scalar,
                index: index.clone(),
            },
        }
    }
}

#[derive(Debug)]
pub enum ContactNode {
    Static(glm::Vec2),
    /// node position
    Node {
        p: glm::Vec2,
        index: (usize, usize),
    },
    /// a, t and initial pos
    Affine {
        t: glm::Vec2,
        a: glm::Mat2,
        x0: glm::Vec2,
        /// (t1, t2, a11, a12, a21, a22)
        index: (usize, usize, usize, usize, usize, usize),
    },
}

impl ContactNode {
    pub fn ndof_diff(&self) -> usize {
        match self {
            ContactNode::Static(_) => 0,
            ContactNode::Node { p, index } => 2,
            ContactNode::Affine { t, a, x0, index } => 6,
        }
    }

    pub fn ndof(&self) -> usize {
        match self {
            ContactNode::Static(_) => 2,
            ContactNode::Node { p, index } => 2,
            ContactNode::Affine { t, a, x0, index } => 6,
        }
    }

    pub fn pos(&self) -> glm::Vec2 {
        match self {
            ContactNode::Static(p) => p.clone(),
            ContactNode::Node { p, index } => p.clone(),
            ContactNode::Affine { a, t, x0, index } => a * x0 + t,
        }
    }
}

pub struct ContactElemHess {
    pub indices: Vec<usize>,
    pub storage: faer::Mat<f32>,
}

impl Index<(usize, usize)> for ContactElemHess {
    type Output = f32;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (row, col) = index;
        if row < self.storage.nrows() && col < self.storage.ncols() {
            &self.storage[(row, col)]
        } else {
            panic!("Out of bounds! i={}, j={}", row, col);
        }
    }
}

impl IndexMut<(usize, usize)> for ContactElemHess {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let (row, col) = index;
        if row < self.storage.nrows() && col < self.storage.ncols() {
            &mut self.storage[(row, col)]
        } else {
            panic!("Out of bounds! i={}, j={}", row, col);
        }
    }
}

impl ContactElemHess {
    pub fn new(elem: &ContactElem) -> Self {
        let p = &elem.p;
        let e = &elem.e;
        let nrows = p.ndof_diff() + e.0.ndof_diff() + e.1.ndof_diff();

        let mut indices = Vec::new();
        let pts = [&p, &e.0, &e.1];
        for pt in pts {
            match pt {
                ContactNode::Static(_) => (),
                ContactNode::Node { p, index } => {
                    indices.push(index.0);
                    indices.push(index.1);
                }
                ContactNode::Affine { t, a, x0, index } => {
                    indices.push(index.0);
                    indices.push(index.1);
                    indices.push(index.2);
                    indices.push(index.3);
                    indices.push(index.4);
                    indices.push(index.5);
                }
            }
        }
        Self {
            indices,
            storage: faer::Mat::zeros(nrows, nrows),
        }
    }

    /// cartprod with itself, row major
    pub fn iterate(&self) -> Vec<(usize, usize, f32)> {
        let mut result = Vec::new();
        for i in 0..self.indices.len() {
            for j in 0..self.indices.len() {
                result.push((self.indices[i], self.indices[j], self.storage[(i, j)]));
            }
        }
        result
    }

    pub fn add_to(&self, hess: &mut Hess) {
        for (iglob, jglob, val) in self.iterate() {
            hess.add_elem(iglob, jglob, val);
        }
    }
}

pub struct ContactElemGrad {
    pub p: ContactNodeGrad,
    pub e: (ContactNodeGrad, ContactNodeGrad),
}

impl ContactElemGrad {
    pub fn scaled_by(&self, scalar: f32) -> ContactElemGrad {
        ContactElemGrad {
            p: self.p.scaled_by(scalar),
            e: (self.e.0.scaled_by(scalar), self.e.1.scaled_by(scalar)),
        }
    }
    pub fn assemble(&self) -> Col<f32> {
        let ndof = self.p.ndof() + self.e.0.ndof() + self.e.1.ndof();
        let mut res = Col::zeros(ndof);

        let mut i = 0;
        match self.p {
            ContactNodeGrad::Static() => (),
            ContactNodeGrad::Node { p, index } => {
                res[i] = p.x;
                res[i + 1] = p.y;
                i += 2;
            }
            ContactNodeGrad::Affine { t, a, index } => {
                res[i] = t.x;
                res[i + 1] = t.y;
                res[i + 2] = a[(0, 0)];
                res[i + 3] = a[(0, 1)];
                res[i + 4] = a[(1, 0)];
                res[i + 5] = a[(1, 1)];
                i += 6;
            }
        }
        match self.e.0 {
            ContactNodeGrad::Static() => (),
            ContactNodeGrad::Node { p, index } => {
                res[i] = p.x;
                res[i + 1] = p.y;
                i += 2;
            }
            ContactNodeGrad::Affine { t, a, index } => {
                res[i] = t.x;
                res[i + 1] = t.y;
                res[i + 2] = a[(0, 0)];
                res[i + 3] = a[(0, 1)];
                res[i + 4] = a[(1, 0)];
                res[i + 5] = a[(1, 1)];
                i += 6;
            }
        }
        match self.e.1 {
            ContactNodeGrad::Static() => (),
            ContactNodeGrad::Node { p, index } => {
                res[i] = p.x;
                res[i + 1] = p.y;
                i += 2;
            }
            ContactNodeGrad::Affine { t, a, index } => {
                res[i] = t.x;
                res[i + 1] = t.y;
                res[i + 2] = a[(0, 0)];
                res[i + 3] = a[(0, 1)];
                res[i + 4] = a[(1, 0)];
                res[i + 5] = a[(1, 1)];
                i += 6;
            }
        }

        res
    }

    pub fn add_to(&self, result: &mut Col<f32>) {
        match self.p {
            ContactNodeGrad::Static() => (),
            ContactNodeGrad::Node { p, index } => {
                result[index.0] = p.x;
                result[index.1] = p.y;
            }
            ContactNodeGrad::Affine { t, a, index } => {
                result[index.0] = t.x;
                result[index.1] = t.y;
                result[index.2] = a[(0, 0)];
                result[index.3] = a[(0, 1)];
                result[index.4] = a[(1, 0)];
                result[index.5] = a[(1, 1)];
            }
        }
    }
}

#[derive(Debug)]
pub struct ContactElem {
    pub p: ContactNode,
    pub e: (ContactNode, ContactNode),
}

impl ContactElem {
    pub fn distance(&self) -> f32 {
        let p = self.p.pos();
        let (a, b) = (&self.e.0.pos(), &self.e.1.pos());

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

    pub fn distance_grad(&self) -> ContactElemGrad {
        // the point and edge in the contact pair
        let p = self.p.pos();
        let (e0, e1) = (self.e.0.pos(), self.e.1.pos());

        let g_p = match self.p {
            ContactNode::Static(_) => todo!(),
            ContactNode::Node { p: _, index } => ContactNodeGrad::Node {
                p: d_grad_p_node(p, e0, e1),
                index: index,
            },
            ContactNode::Affine { a, t, x0, index } => todo!(),
        };

        let g_e0 = match self.e.0 {
            ContactNode::Static(_) => ContactNodeGrad::Static(),
            ContactNode::Node { p: _, index } => ContactNodeGrad::Node {
                p: d_grad_e0_node(p, e0, e1),
                index: index,
            },
            ContactNode::Affine {
                a: _,
                t: _,
                x0: _,
                index,
            } => todo!(),
        };

        let g_e1 = match self.e.1 {
            ContactNode::Static(_) => ContactNodeGrad::Static(),
            ContactNode::Node { p: _, index } => ContactNodeGrad::Node {
                p: d_grad_e1_node(p, e0, e1),
                index: index,
            },
            ContactNode::Affine {
                a: _,
                t: _,
                x0: _,
                index,
            } => todo!(),
        };

        ContactElemGrad {
            p: g_p,
            e: (g_e0, g_e1),
        }
    }

    pub fn distance_hess(&self) -> ContactElemHess {
        let mut res = ContactElemHess::new(self);

        match (&self.p, &self.e) {
            // All are nodes: hess : 6x6
            (
                ContactNode::Node { p, index: ip },
                (ContactNode::Node { p: e0, index: ie0 }, ContactNode::Node { p: e1, index: ie1 }),
            ) => {
                let p = *p;
                let (a, b) = (*e0, *e1);

                let ab = b - a;
                let ap = p - a;
                let bp = p - b;

                // Check if the point is closest to the start of the edge
                if ab.dot(&ap) <= 0.0 {
                    let ap_norm = ap.norm();
                    let ap_norm_inv_3 = ap_norm.powi(3);

                    // Hessian for point

                    let subhess_00 = ap.y * ap.y * ap_norm_inv_3;
                    let subhess_01 = -ap.x * ap.y * ap_norm_inv_3;
                    let subhess_11 = ap.x * ap.x * ap_norm_inv_3;
                    // subhess 0
                    res[(0, 0)] = subhess_00;
                    res[(0, 1)] = subhess_01;
                    res[(1, 0)] = subhess_01;
                    res[(1, 1)] = subhess_11;

                    // subhess 1
                    res[(0, 2)] = -subhess_00;
                    res[(0, 3)] = -subhess_01;
                    res[(1, 2)] = -subhess_01;
                    res[(1, 3)] = -subhess_11;

                    // subhess 2
                    res[(2, 0)] = -subhess_00;
                    res[(2, 1)] = -subhess_01;
                    res[(3, 0)] = -subhess_01;
                    res[(3, 1)] = -subhess_11;

                    // subhess 3
                    res[(2, 2)] = subhess_00;
                    res[(2, 3)] = subhess_01;
                    res[(3, 2)] = subhess_01;
                    res[(3, 3)] = subhess_11;
                } else if ab.dot(&bp) >= 0.0 {
                    let bp_norm = bp.norm();
                    let bp_norm_inv_3 = bp_norm.powi(3);

                    // Hessian for point

                    let subhess_00 = bp.y * bp.y * bp_norm_inv_3;
                    let subhess_01 = -bp.x * bp.y * bp_norm_inv_3;
                    let subhess_11 = bp.x * bp.x * bp_norm_inv_3;
                    // subhess 0
                    res[(0, 0)] = subhess_00;
                    res[(0, 1)] = subhess_01;
                    res[(1, 0)] = subhess_01;
                    res[(1, 1)] = subhess_11;

                    // subhess 1
                    res[(0, 4)] = -subhess_00;
                    res[(0, 5)] = -subhess_01;
                    res[(1, 4)] = -subhess_01;
                    res[(1, 5)] = -subhess_11;

                    // subhess 2
                    res[(4, 0)] = -subhess_00;
                    res[(4, 1)] = -subhess_01;
                    res[(5, 0)] = -subhess_01;
                    res[(5, 1)] = -subhess_11;

                    // subhess 3
                    res[(4, 4)] = subhess_00;
                    res[(4, 5)] = subhess_01;
                    res[(5, 4)] = subhess_01;
                    res[(5, 5)] = subhess_11;
                }
                // The point is closest to the edge itself
                else {
                    // do nothing for now
                    // todo!()
                }
            }

            // node point with static edge. hess: 2x2
            (
                ContactNode::Node { p, index: ip },
                (ContactNode::Static(e0), ContactNode::Static(e1)),
            ) => {
                let p = *p;
                let (a, b) = (*e0, *e1);

                let ab = b - a;
                let ap = p - a;
                let bp = p - b;

                // Check if the point is closest to the start of the edge
                if ab.dot(&ap) <= 0.0 {
                    let ap_norm = ap.norm();
                    let ap_norm_inv_3 = ap_norm.powi(3);

                    // Hessian for point
                    let subhess_00 = ap.y * ap.y * ap_norm_inv_3;
                    let subhess_01 = -ap.x * ap.y * ap_norm_inv_3;
                    let subhess_11 = ap.x * ap.x * ap_norm_inv_3;
                    // subhess 0
                    res[(0, 0)] = subhess_00;
                    res[(0, 1)] = subhess_01;
                    res[(1, 0)] = subhess_01;
                    res[(1, 1)] = subhess_11;
                } else if ab.dot(&bp) >= 0.0 {
                    let bp_norm = bp.norm();
                    let bp_norm_inv_3 = bp_norm.powi(3);

                    // Hessian for point
                    let subhess_00 = bp.y * bp.y * bp_norm_inv_3;
                    let subhess_01 = -bp.x * bp.y * bp_norm_inv_3;
                    let subhess_11 = bp.x * bp.x * bp_norm_inv_3;
                    // subhess 0
                    res[(0, 0)] = subhess_00;
                    res[(0, 1)] = subhess_01;
                    res[(1, 0)] = subhess_01;
                    res[(1, 1)] = subhess_11;
                }
                // The point is closest to the edge itself
                else {
                    // do nothing for now
                    // todo!()
                }
            }

            // no hess
            (ContactNode::Static(_), (ContactNode::Static(_), ContactNode::Static(_))) => (),

            _ => (),
        }
        res
    }
}

impl ContactElem {
    pub fn pos(&self) -> CcdPair {
        CcdPair {
            p: self.p.pos(),
            e: (self.e.0.pos(), self.e.1.pos()),
        }
    }
}

pub struct ContactElemIp {
    pub run_config: RunConfig,
}

impl ContactElemIp {
    pub fn new(run_config: &RunConfig) -> Self {
        Self {
            run_config: run_config.clone(),
        }
    }
}

impl ContactElemIp {
    pub fn value(&self, body: &ContactElem) -> f32 {
        let d = body.distance();
        -(d - self.run_config.dhat) * (d - self.run_config.dhat) * ((d / self.run_config.dhat).ln())
    }

    pub fn append_grad(&self, body: &ContactElem, result: &mut Col<f32>) {
        let d = body.distance();
        let diff1 = (d - self.run_config.dhat)
            * (-2f32 * d * (d / self.run_config.dhat).ln() - d + self.run_config.dhat)
            / d;

        let ip_grad = body.distance_grad().scaled_by(diff1);
        ip_grad.add_to(result);
    }

    pub fn append_hess(&self, body: &ContactElem, result: &mut Hess) {
        let d = body.distance();
        let diff1 = (d - self.run_config.dhat)
            * (-2f32 * d * (d / self.run_config.dhat).ln() - d + self.run_config.dhat)
            / d;
        let diff2 = -2f32 * (d / self.run_config.dhat).ln() - 3f32
            + 2f32 * self.run_config.dhat / d
            + (self.run_config.dhat * self.run_config.dhat) / (d * d);

        let d_grad = body.distance_grad().assemble();
        let d_hess = body.distance_hess();

        // dbg!(&body);
        // dbg!(&d_grad);
        // dbg!(&d_grad * d_grad.transpose());
        // dbg!(&d_hess.storage);

        let hess_raw =
            faer::scale(diff2) * &d_grad * d_grad.transpose() + faer::scale(diff1) * d_hess.storage;
        // hess projection
        // let spd_hess_raw = hess_spd_proj(&hess_raw);
        // dbg!(&hess_raw);
        let spd_hess_raw = hess_raw.clone();

        let ip_hess = ContactElemHess {
            indices: d_hess.indices,
            storage: spd_hess_raw,
        };

        ip_hess.add_to(result);
    }
}
