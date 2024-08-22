use faer::{linalg::svd::SvdParams, Col};
use glm::sign;
use nalgebra::{ArrayStorage, U6};

use crate::{
    sim::utils::{hess::Hess, misc::hess_spd_proj},
    RunConfig,
};

pub type ContactHessType = nalgebra::Matrix<f32, U6, U6, ArrayStorage<f32, 6, 6>>;
pub type Vec6 = glm::TVec<f32, 6>;

#[derive(Clone, Copy)]
/// ## Trivially Copyable: No moving.
/// - currently each point is 2 dof
pub struct ContactIndex {
    pub p: Option<(usize, usize)>,
    pub e: Option<((usize, usize), (usize, usize))>,
}

impl ContactIndex {
    /// ((raw_i, raw_j), irow, icol)
    pub fn hess_indices(&self) -> Vec<((usize, usize), Option<usize>, Option<usize>)> {
        // return:
        // [p*2, p*2+1, e.0*2, e.0*2+1, e.1*2, e.1*2+1]^2
        // (cart prod) in order.
        let mut indices = Vec::new();

        // Indices for the point p
        let (ipx, ipy) = match self.p {
            Some((ix, iy)) => (Some(ix), Some(iy)),
            None => (None, None),
        };

        // Indices for the edge e
        let (ie0x, ie0y, ie1x, ie1y) = match self.e {
            Some(((i0x, i0y), (i1x, i1y))) => (Some(i0x), Some(i0y), Some(i1x), Some(i1y)),
            None => (None, None, None, None),
        };

        // List of all indices to consider
        let all_indices = [ipx, ipy, ie0x, ie0y, ie1x, ie1y];

        // Generate all pairs of indices
        for (i, &irow) in all_indices.iter().enumerate() {
            for (j, &icol) in all_indices.iter().enumerate() {
                indices.push(((i, j), irow, icol));
            }
        }

        indices
    }
}

pub struct ContactHess {
    pub hess: ContactHessType,

    pub index: ContactIndex,
}

impl ContactHess {
    pub fn add_elem(&mut self, i: usize, j: usize, val: f32) {
        self.hess[(i, j)] += val;
    }

    pub fn zeros(index: ContactIndex) -> Self {
        Self {
            hess: ContactHessType::zeros(),
            index,
        }
    }
}

pub struct ContactGrad {
    pub g_point: glm::Vec2,
    pub g_edge: (glm::Vec2, glm::Vec2),

    pub index: ContactIndex,
}

impl ContactGrad {
    pub fn scaled_by(&self, scalar: f32) -> ContactGrad {
        ContactGrad {
            g_edge: (self.g_edge.0 * scalar, self.g_edge.1 * scalar),
            g_point: self.g_point * scalar,
            index: self.index,
        }
    }

    /// - First 2 dofs are from point;
    /// - The 4 rest are from the two ends of edge.
    pub fn assemble(&self) -> Vec6 {
        let mut res = Vec6::zeros();

        res[0] = self.g_point.x;
        res[1] = self.g_point.y;

        res[2] = self.g_edge.0.x;
        res[3] = self.g_edge.0.y;

        res[4] = self.g_edge.1.x;
        res[5] = self.g_edge.1.y;

        res
    }
}

#[derive(Clone)]
pub struct ContactPair {
    pub edge: (glm::Vec2, glm::Vec2),
    pub point: glm::Vec2,

    pub index: ContactIndex,
}

impl ContactPair {
    pub fn distance(&self) -> f32 {
        let (a, b) = self.edge;
        let p = self.point;

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

    pub fn distance_grad(&self) -> ContactGrad {
        let (a, b) = self.edge;
        let p = self.point;

        let ab = b - a;
        let ap = p - a;
        let bp = p - b;

        let mut g_edge_a = glm::vec2(0.0, 0.0);
        let mut g_edge_b = glm::vec2(0.0, 0.0);
        let mut g_point = glm::vec2(0.0, 0.0);

        // Check if the point is closest to the start of the edge
        if ab.dot(&ap) <= 0.0 {
            let ap_norm = ap.norm();
            g_point = ap / ap_norm;
            g_edge_a = -ap / ap_norm;
        }
        // Check if the point is closest to the end of the edge
        else if ab.dot(&bp) >= 0.0 {
            let bp_norm = bp.norm();
            g_point = bp / bp_norm;
            g_edge_b = -bp / bp_norm;
        }
        // The point is closest to the edge itself
        else {
            let ab_norm = ab.norm();
            let ab_norm_3 = ab_norm.powi(3);

            let cross = ab.x * ap.y - ap.x * ab.y;

            g_point = glm::vec2(
                -ab.y * cross.signum() / ab_norm,
                ab.x * cross.signum() / ab_norm,
            );
            g_edge_a = glm::vec2(
                ab.x * cross.abs() / ab_norm_3 - bp.y * cross.signum() / ab_norm,
                ab.y * cross.abs() / ab_norm_3 + bp.x * cross.signum() / ab_norm,
            );
            // todo: g_edge_b
            g_edge_b = glm::vec2(
                -ab.x * cross.abs() / ab_norm_3 + ap.y * cross.signum() / ab_norm,
                -ab.y * cross.abs() / ab_norm_3 - ap.x * cross.signum() / ab_norm,
            );
        }

        ContactGrad {
            g_edge: (g_edge_a, g_edge_b),
            g_point: g_point,
            index: self.index,
        }
    }

    /// - First 2 dofs are from point;
    /// - The 4 rest are from the two ends of edge.
    pub fn distance_hess(&self) -> ContactHess {
        let mut res = ContactHess::zeros(self.index);

        let (a, b) = self.edge;
        let p = self.point;

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
            res.add_elem(0, 0, subhess_00);
            res.add_elem(0, 1, subhess_01);
            res.add_elem(1, 0, subhess_01);
            res.add_elem(1, 1, subhess_11);

            // subhess 1
            res.add_elem(0, 2, -subhess_00);
            res.add_elem(0, 3, -subhess_01);
            res.add_elem(1, 2, -subhess_01);
            res.add_elem(1, 3, -subhess_11);

            // subhess 2
            res.add_elem(2, 0, -subhess_00);
            res.add_elem(2, 1, -subhess_01);
            res.add_elem(3, 0, -subhess_01);
            res.add_elem(3, 1, -subhess_11);

            // subhess 0
            res.add_elem(2, 2, subhess_00);
            res.add_elem(2, 3, subhess_01);
            res.add_elem(3, 2, subhess_01);
            res.add_elem(3, 3, subhess_11);
        } else if ab.dot(&bp) >= 0.0 {
            let bp_norm = bp.norm();
            let bp_norm_inv_3 = bp_norm.powi(3);

            // Hessian for point

            let subhess_00 = bp.y * bp.y * bp_norm_inv_3;
            let subhess_01 = -bp.x * bp.y * bp_norm_inv_3;
            let subhess_11 = bp.x * bp.x * bp_norm_inv_3;
            // subhess 0
            res.add_elem(0, 0, subhess_00);
            res.add_elem(0, 1, subhess_01);
            res.add_elem(1, 0, subhess_01);
            res.add_elem(1, 1, subhess_11);

            // subhess 1
            res.add_elem(0, 2, -subhess_00);
            res.add_elem(0, 3, -subhess_01);
            res.add_elem(1, 2, -subhess_01);
            res.add_elem(1, 3, -subhess_11);

            // subhess 2
            res.add_elem(2, 0, -subhess_00);
            res.add_elem(2, 1, -subhess_01);
            res.add_elem(3, 0, -subhess_01);
            res.add_elem(3, 1, -subhess_11);

            // subhess 0
            res.add_elem(2, 2, subhess_00);
            res.add_elem(2, 3, subhess_01);
            res.add_elem(3, 2, subhess_01);
            res.add_elem(3, 3, subhess_11);
        }
        // The point is closest to the edge itself
        else {
            // do nothing for now
            // todo!()
        }
        res
    }
}

pub struct ContactPairIp {
    pub run_config: RunConfig,
}

impl ContactPairIp {
    pub fn new(run_config: &RunConfig) -> Self {
        Self {
            run_config: run_config.clone(),
        }
    }
}

impl ContactPairIp {
    pub fn value(&self, body: &ContactPair) -> f32 {
        let d = body.distance();
        -(d - self.run_config.dhat) * (d - self.run_config.dhat) * ((d / self.run_config.dhat).ln())
    }

    pub fn append_grad(&self, body: &ContactPair, result: &mut Col<f32>) {
        let d = body.distance();
        let diff1 = (d - self.run_config.dhat)
            * (-2f32 * d * (d / self.run_config.dhat).ln() - d + self.run_config.dhat)
            / d;

        let grad = body.distance_grad().scaled_by(diff1);

        // fill into `result`
        match grad.index.e {
            Some((e0, e1)) => {
                // first node
                result[e0.0] += grad.g_edge.0.x;
                result[e0.1] += grad.g_edge.0.y;
                // seconde node
                result[e1.0] += grad.g_edge.1.x;
                result[e1.1] += grad.g_edge.1.y;
            }
            None => (), // target dofs is not driven by dynamics
        }

        match grad.index.p {
            Some((px, py)) => {
                result[px] += grad.g_point.x;
                result[py] += grad.g_point.y;
            }
            None => (), // target dof is not driven by dynamics
        }
    }

    pub fn append_hess(&self, body: &ContactPair, result: &mut Hess) {
        let d = body.distance();
        let diff1 = (d - self.run_config.dhat)
            * (-2f32 * d * (d / self.run_config.dhat).ln() - d + self.run_config.dhat)
            / d;
        let diff2 = -2f32 * (d / self.run_config.dhat).ln() - 3f32
            + 2f32 * self.run_config.dhat / d
            + (self.run_config.dhat * self.run_config.dhat) / (d * d);

        let d_hess = body.distance_hess();
        let d_grad = body.distance_grad().assemble();
        let hess_raw = diff2 * d_grad * d_grad.transpose() + diff1 * d_hess.hess;
        // hess projection
        let hess = hess_spd_proj(&hess_raw);

        // fill into `result`
        // raw_idx: index within the contact pair.
        for (raw_idx, irow, icol) in body.index.hess_indices() {
            if let Some(i) = irow {
                // if let cannot use && ???
                if let Some(j) = icol {
                    result.add_elem(i, j, hess[raw_idx]);
                }
            }
        }
    }
}
