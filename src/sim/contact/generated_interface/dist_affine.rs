use crate::sim::{
    generated::contact_edge_affine::{
        grad::*,
        hess::{case1::hess_edge_case1, case2::hess_edge_case2, case3::hess_edge_case3},
    },
    utils::{
        affine_utils::interop::AffineDof,
        types::{Mat6x6, Vec6},
    },
};

use crate::sim::generated::contact_point_affine::grad::*;
use crate::sim::generated::contact_point_affine::hess::case1::*;
use crate::sim::generated::contact_point_affine::hess::case2::*;
use crate::sim::generated::contact_point_affine::hess::case3::*;

// ############## point ####################

pub struct PointContactArg {
    // edge ends
    pub u: glm::Vec2,
    pub v: glm::Vec2,

    // rest point position
    pub x0: glm::Vec2,

    // q: abd configuration vector
    pub q: Vec6,
}

impl PointContactArg {
    pub fn x(&self) -> glm::Vec2 {
        let a = self.q.a_mat();
        let t = self.q.t_vec();
        a * self.x0 + t
    }
}

impl PointContactArg {
    /// gradient of distance
    /// for point (vertex) on affine body
    pub fn d_grad_p_affine(&self) -> Vec6 {
        let uv = self.v - self.u;
        let ux = self.x() - self.u;

        let uv_len_sq = uv.dot(&uv);
        let t = ux.dot(&uv) / uv_len_sq;

        let (a11, a12, a21, a22) = self.q.a_unpack();
        let (tx, ty) = self.q.t_unpack();
        let (px, py) = (self.x0.x, self.x0.y);
        let (ux, uy, vx, vy) = (self.u.x, self.u.y, self.v.x, self.v.y);

        if t < 0.0 {
            // p is closer to u
            Vec6::new(
                grad_case1_0(a11, a12, a21, a22, px, py, tx, ty, ux, uy),
                grad_case1_1(a11, a12, a21, a22, px, py, tx, ty, ux, uy),
                grad_case1_2(a11, a12, a21, a22, px, py, tx, ty, ux, uy),
                grad_case1_3(a11, a12, a21, a22, px, py, tx, ty, ux, uy),
                grad_case1_4(a11, a12, a21, a22, px, py, tx, ty, ux, uy),
                grad_case1_5(a11, a12, a21, a22, px, py, tx, ty, ux, uy),
            )
        } else if t > 1.0 {
            // p is closer to v
            Vec6::new(
                grad_case2_0(a11, a12, a21, a22, px, py, tx, ty, vx, vy),
                grad_case2_1(a11, a12, a21, a22, px, py, tx, ty, vx, vy),
                grad_case2_2(a11, a12, a21, a22, px, py, tx, ty, vx, vy),
                grad_case2_3(a11, a12, a21, a22, px, py, tx, ty, vx, vy),
                grad_case2_4(a11, a12, a21, a22, px, py, tx, ty, vx, vy),
                grad_case2_5(a11, a12, a21, a22, px, py, tx, ty, vx, vy),
            )
        } else {
            // p is closest to the line segment uv
            Vec6::new(
                grad_case3_0(a11, a12, a21, a22, px, py, tx, ty, ux, uy, vx, vy),
                grad_case3_1(a11, a12, a21, a22, px, py, tx, ty, ux, uy, vx, vy),
                grad_case3_2(a11, a12, a21, a22, px, py, tx, ty, ux, uy, vx, vy),
                grad_case3_3(a11, a12, a21, a22, px, py, tx, ty, ux, uy, vx, vy),
                grad_case3_4(a11, a12, a21, a22, px, py, tx, ty, ux, uy, vx, vy),
                grad_case3_5(a11, a12, a21, a22, px, py, tx, ty, ux, uy, vx, vy),
            )
        }
    }

    pub fn d_hess_p_affine(&self) -> Mat6x6 {
        let uv = self.v - self.u;
        let ux = self.x() - self.u;

        let uv_len_sq = uv.dot(&uv);
        let t = ux.dot(&uv) / uv_len_sq;

        let (a11, a12, a21, a22) = self.q.a_unpack();
        let (tx, ty) = self.q.t_unpack();
        let (px, py) = (self.x0.x, self.x0.y);
        let (ux, uy, vx, vy) = (self.u.x, self.u.y, self.v.x, self.v.y);

        if t < 0.0 {
            // p is closer to u
            hess_case1(a11, a12, a21, a22, px, py, tx, ty, ux, uy)
        } else if t > 1.0 {
            // p is closer to v
            hess_case2(a11, a12, a21, a22, px, py, tx, ty, vx, vy)
        } else {
            // p is closest to the line segment uv
            hess_case3()
        }
    }
}

// ############## edge ####################

pub struct EdgeContactArg {
    // initial position of ends of edge
    pub u0: glm::Vec2,
    pub v0: glm::Vec2,

    pub q: Vec6,

    /// position of point in the contact
    pub x: glm::Vec2,
}

impl EdgeContactArg {
    pub fn u(&self) -> glm::Vec2 {
        let a = self.q.a_mat();
        let t = self.q.t_vec();
        a * self.u0 + t
    }

    pub fn v(&self) -> glm::Vec2 {
        let a = self.q.a_mat();
        let t = self.q.t_vec();
        a * self.v0 + t
    }
}

impl EdgeContactArg {
    pub fn d_grad_e_affine(&self) -> Vec6 {
        let (a11, a12, a21, a22) = self.q.a_unpack();
        let (tx, ty) = self.q.t_unpack();
        let (px, py) = (self.x.x, self.x.y);
        let (u0x, u0y) = (self.u0.x, self.u0.y);
        let (v0x, v0y) = (self.v0.x, self.v0.y);

        let uv = self.v() - self.u();
        let ux = self.x - self.u();
        let uv_len_sq = uv.dot(&uv);
        let t = ux.dot(&uv) / uv_len_sq;

        if t < 0.0 {
            // case 1
            Vec6::new(
                grad_edge_case1_0(a11, a12, a21, a22, px, py, tx, ty, u0x, u0y),
                grad_edge_case1_1(a11, a12, a21, a22, px, py, tx, ty, u0x, u0y),
                grad_edge_case1_2(a11, a12, a21, a22, px, py, tx, ty, u0x, u0y),
                grad_edge_case1_3(a11, a12, a21, a22, px, py, tx, ty, u0x, u0y),
                grad_edge_case1_4(a11, a12, a21, a22, px, py, tx, ty, u0x, u0y),
                grad_edge_case1_5(a11, a12, a21, a22, px, py, tx, ty, u0x, u0y),
            )
        } else if t > 1.0 {
            // case 2
            Vec6::new(
                grad_edge_case2_0(a11, a12, a21, a22, px, py, tx, ty, v0x, v0y),
                grad_edge_case2_1(a11, a12, a21, a22, px, py, tx, ty, v0x, v0y),
                grad_edge_case2_2(a11, a12, a21, a22, px, py, tx, ty, v0x, v0y),
                grad_edge_case2_3(a11, a12, a21, a22, px, py, tx, ty, v0x, v0y),
                grad_edge_case2_4(a11, a12, a21, a22, px, py, tx, ty, v0x, v0y),
                grad_edge_case2_5(a11, a12, a21, a22, px, py, tx, ty, v0x, v0y),
            )
        } else {
            // case 3
            Vec6::new(
                grad_edge_case3_0(a11, a12, a21, a22, u0x, u0y, v0x, v0y),
                grad_edge_case3_1(a11, a12, a21, a22, u0x, u0y, v0x, v0y),
                grad_edge_case3_2(a11, a12, a21, a22, px, py, tx, ty, u0x, u0y, v0x, v0y),
                grad_edge_case3_3(a11, a12, a21, a22, px, py, tx, ty, u0x, u0y, v0x, v0y),
                grad_edge_case3_4(a11, a12, a21, a22, px, py, tx, ty, u0x, u0y, v0x, v0y),
                grad_edge_case3_5(a11, a12, a21, a22, px, py, tx, ty, u0x, u0y, v0x, v0y),
            )
        }
    }

    pub fn d_hess_e_affine(&self) -> Mat6x6 {
        let (a11, a12, a21, a22) = self.q.a_unpack();
        let (tx, ty) = self.q.t_unpack();
        let (px, py) = (self.x.x, self.x.y);
        let (u0x, u0y) = (self.u0.x, self.u0.y);
        let (v0x, v0y) = (self.v0.x, self.v0.y);

        let uv = self.v() - self.u();
        let ux = self.x - self.u();
        let uv_len_sq = uv.dot(&uv);
        let t = ux.dot(&uv) / uv_len_sq;

        if t < 0.0 {
            // p is closer to u
            hess_edge_case1(a11, a12, a21, a22, px, py, tx, ty, u0x, u0y)
        } else if t > 1.0 {
            // p is closer to v
            hess_edge_case2(a11, a12, a21, a22, px, py, tx, ty, v0x, v0y)
        } else {
            // p is closest to the line segment uv
            hess_edge_case3(a11, a12, a21, a22, px, py, tx, ty, u0x, u0y, v0x, v0y)
        }
    }
}
