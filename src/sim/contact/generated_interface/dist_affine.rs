use crate::sim::utils::{
    affine_utils::interop::AffineDof,
    types::{Mat6x6, Vec6},
};

use crate::sim::generated::contact_p_affine::grad::*;
use crate::sim::generated::contact_p_affine::hess::case1::*;
use crate::sim::generated::contact_p_affine::hess::case2::*;
use crate::sim::generated::contact_p_affine::hess::case3::*;

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
