use faer::Col;
use glm::zero;
use nalgebra::{ArrayStorage, Matrix6x5, U6};

use crate::sim::utils::hess::Hess;

pub type ContactHessType = nalgebra::Matrix<f32, U6, U6, ArrayStorage<f32, 6, 6>>;
pub type Vec6 = glm::TVec<f32, 6>;

#[derive(Clone, Copy)]
/// ## Trivially Copyable: No moving.
pub struct ContactIndex {
    pub p: usize,
    pub e: (usize, usize),
}

impl ContactIndex {
    /// ((raw_i, raw_j), irow, icol)
    pub fn hess_indices(&self) -> Vec<((usize, usize), usize, usize)> {
        // return:
        // [p*2, p*2+1, e.0*2, e.0*2+1, e.1*2, e.1*2+1]^2
        // (cart prod) in order.
        let mut indices = Vec::new();

        // Indices for the point p
        let p_indices = self.p * 2;

        // Indices for the edge e
        let e0_indices = self.e.0 * 2;
        let e1_indices = self.e.1 * 2;

        // List of all indices to consider
        let all_indices = [
            p_indices,
            p_indices + 1,
            e0_indices,
            e0_indices + 1,
            e1_indices,
            e1_indices + 1,
        ];

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

pub struct ContactPairGrad {
    pub g_point: glm::Vec2,
    pub g_edge: (glm::Vec2, glm::Vec2),

    pub index: ContactIndex,
}

impl ContactPairGrad {
    pub fn scaled_by(&self, scalar: f32) -> ContactPairGrad {
        ContactPairGrad {
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

    pub fn distance_grad(&self) -> ContactPairGrad {
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
            g_edge_a = -ap / ap_norm;
            g_point = ap / ap_norm;
        }
        // Check if the point is closest to the end of the edge
        else if ab.dot(&bp) >= 0.0 {
            let bp_norm = bp.norm();
            g_edge_b = -bp / bp_norm;
            g_point = bp / bp_norm;
        }
        // The point is closest to the edge itself
        else {
            let ap_proj_on_ab = ap.dot(&ab) / ab.norm_squared();
            let proj_point = a + ap_proj_on_ab * ab;
            let d = p - proj_point;
            let d_norm = d.norm();

            let g_proj_point = -d / d_norm;
            let g_ap_proj_on_ab = g_proj_point.dot(&ab);

            g_edge_a = g_proj_point - g_ap_proj_on_ab * ab / ab.norm_squared();
            g_edge_b = g_ap_proj_on_ab * ab / ab.norm_squared();
            g_point = -g_proj_point;
        }

        ContactPairGrad {
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
            let ap_norm_inv = 1.0 / ap_norm;
            let ap_normalized = ap * ap_norm_inv;

            // Hessian for point
            res.add_elem(
                0,
                0,
                ap_norm_inv * (1.0 - ap_normalized.x * ap_normalized.x),
            );
            res.add_elem(0, 1, -ap_norm_inv * ap_normalized.x * ap_normalized.y);
            res.add_elem(1, 0, -ap_norm_inv * ap_normalized.x * ap_normalized.y);
            res.add_elem(
                1,
                1,
                ap_norm_inv * (1.0 - ap_normalized.y * ap_normalized.y),
            );

            // Hessian for edge start
            res.add_elem(
                2,
                2,
                ap_norm_inv * (1.0 - ap_normalized.x * ap_normalized.x),
            );
            res.add_elem(2, 3, -ap_norm_inv * ap_normalized.x * ap_normalized.y);
            res.add_elem(3, 2, -ap_norm_inv * ap_normalized.x * ap_normalized.y);
            res.add_elem(
                3,
                3,
                ap_norm_inv * (1.0 - ap_normalized.y * ap_normalized.y),
            );

            // Cross terms
            res.add_elem(
                0,
                2,
                -ap_norm_inv * (1.0 - ap_normalized.x * ap_normalized.x),
            );
            res.add_elem(0, 3, ap_norm_inv * ap_normalized.x * ap_normalized.y);
            res.add_elem(1, 2, ap_norm_inv * ap_normalized.x * ap_normalized.y);
            res.add_elem(
                1,
                3,
                -ap_norm_inv * (1.0 - ap_normalized.y * ap_normalized.y),
            );
        }
        // Check if the point is closest to the end of the edge
        else if ab.dot(&bp) >= 0.0 {
            let bp_norm = bp.norm();
            let bp_norm_inv = 1.0 / bp_norm;
            let bp_normalized = bp * bp_norm_inv;

            // Hessian for point
            res.add_elem(
                0,
                0,
                bp_norm_inv * (1.0 - bp_normalized.x * bp_normalized.x),
            );
            res.add_elem(0, 1, -bp_norm_inv * bp_normalized.x * bp_normalized.y);
            res.add_elem(1, 0, -bp_norm_inv * bp_normalized.x * bp_normalized.y);
            res.add_elem(
                1,
                1,
                bp_norm_inv * (1.0 - bp_normalized.y * bp_normalized.y),
            );

            // Hessian for edge end
            res.add_elem(
                4,
                4,
                bp_norm_inv * (1.0 - bp_normalized.x * bp_normalized.x),
            );
            res.add_elem(4, 5, -bp_norm_inv * bp_normalized.x * bp_normalized.y);
            res.add_elem(5, 4, -bp_norm_inv * bp_normalized.x * bp_normalized.y);
            res.add_elem(
                5,
                5,
                bp_norm_inv * (1.0 - bp_normalized.y * bp_normalized.y),
            );

            // Cross terms
            res.add_elem(
                0,
                4,
                -bp_norm_inv * (1.0 - bp_normalized.x * bp_normalized.x),
            );
            res.add_elem(0, 5, bp_norm_inv * bp_normalized.x * bp_normalized.y);
            res.add_elem(1, 4, bp_norm_inv * bp_normalized.x * bp_normalized.y);
            res.add_elem(
                1,
                5,
                -bp_norm_inv * (1.0 - bp_normalized.y * bp_normalized.y),
            );
        }
        // The point is closest to the edge itself
        else {
            let ap_proj_on_ab = ap.dot(&ab) / ab.norm_squared();
            let proj_point = a + ap_proj_on_ab * ab;
            let d = p - proj_point;
            let d_norm = d.norm();
            let d_norm_inv = 1.0 / d_norm;
            let d_normalized = d * d_norm_inv;

            let ab_norm_squared_inv = 1.0 / ab.norm_squared();
            let ab_normalized = ab * ab_norm_squared_inv;

            // Hessian for point
            res.add_elem(0, 0, d_norm_inv * (1.0 - d_normalized.x * d_normalized.x));
            res.add_elem(0, 1, -d_norm_inv * d_normalized.x * d_normalized.y);
            res.add_elem(1, 0, -d_norm_inv * d_normalized.x * d_normalized.y);
            res.add_elem(1, 1, d_norm_inv * (1.0 - d_normalized.y * d_normalized.y));

            // Hessian for edge start and end
            res.add_elem(2, 2, d_norm_inv * (1.0 - d_normalized.x * d_normalized.x));
            res.add_elem(2, 3, -d_norm_inv * d_normalized.x * d_normalized.y);
            res.add_elem(3, 2, -d_norm_inv * d_normalized.x * d_normalized.y);
            res.add_elem(3, 3, d_norm_inv * (1.0 - d_normalized.y * d_normalized.y));

            res.add_elem(4, 4, d_norm_inv * (1.0 - d_normalized.x * d_normalized.x));
            res.add_elem(4, 5, -d_norm_inv * d_normalized.x * d_normalized.y);
            res.add_elem(5, 4, -d_norm_inv * d_normalized.x * d_normalized.y);
            res.add_elem(5, 5, d_norm_inv * (1.0 - d_normalized.y * d_normalized.y));

            // Cross terms
            res.add_elem(0, 2, -d_norm_inv * (1.0 - d_normalized.x * d_normalized.x));
            res.add_elem(0, 3, d_norm_inv * d_normalized.x * d_normalized.y);
            res.add_elem(1, 2, d_norm_inv * d_normalized.x * d_normalized.y);
            res.add_elem(1, 3, -d_norm_inv * (1.0 - d_normalized.y * d_normalized.y));

            res.add_elem(0, 4, -d_norm_inv * (1.0 - d_normalized.x * d_normalized.x));
            res.add_elem(0, 5, d_norm_inv * d_normalized.x * d_normalized.y);
            res.add_elem(1, 4, d_norm_inv * d_normalized.x * d_normalized.y);
            res.add_elem(1, 5, -d_norm_inv * (1.0 - d_normalized.y * d_normalized.y));
        }

        res
    }

    pub fn potential(&self, dhat: f32) -> f32 {
        let d = self.distance();
        return -(d - dhat) * (d - dhat) * ((d / dhat).ln());
    }

    pub fn grad_potential(&self, dhat: f32, result: &mut Col<f32>) {
        let d = self.distance();
        let diff1 = (d - dhat) * (-2f32 * d * (d / dhat).ln() - d + dhat) / d;

        let grad = self.distance_grad().scaled_by(diff1);

        // fill into `result`
        result[grad.index.e.0 * 2] += grad.g_edge.0.x;
        result[grad.index.e.0 * 2 + 1] += grad.g_edge.0.y;

        result[grad.index.e.1 * 2] += grad.g_edge.1.x;
        result[grad.index.e.1 * 2 + 1] += grad.g_edge.1.y;

        result[grad.index.p * 2] += grad.g_point.x;
        result[grad.index.p * 2 + 1] += grad.g_point.y;
    }

    pub fn hess_potential(&self, dhat: f32, result: &mut Hess) {
        let d = self.distance();
        let diff1 = (d - dhat) * (-2f32 * d * (d / dhat).ln() - d + dhat) / d;
        let diff2 = -2f32 * (d / dhat).ln() - 3f32 + 2f32 * dhat / d + (dhat * dhat) / (d * d);

        let d_hess = self.distance_hess();
        let d_grad = self.distance_grad().assemble();
        let hess = diff2 * d_grad * d_grad.transpose() + diff1 * d_hess.hess;

        // todo: fill into `result`
        for (raw_idx, irow, icol) in self.index.hess_indices() {
            result.add_elem(irow, icol, hess[raw_idx]);
        }
    }
}

pub struct ContactIp {}

impl ContactIp {}
