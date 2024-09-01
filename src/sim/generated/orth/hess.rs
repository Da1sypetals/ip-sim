/*
 *                      Code generated with SymPy 1.13.2
 *
 *              See http://www.sympy.org/ for more information.
 *
 *                       This file is part of 'project'
 */

use crate::sim::utils::types::Mat6x6;

fn affine_hess_0_0(a11: f32, a12: f32, a21: f32, a22: f32) -> f32 {
    let out1 = 12f32 * a11.powi(2) + 4f32 * a12.powi(2) + 4f32 * a21.powi(2) - 4f32;
    out1
}

fn affine_hess_0_1(a11: f32, a12: f32, a21: f32, a22: f32) -> f32 {
    let out1 = 8f32 * a11 * a12 + 4f32 * a21 * a22;
    out1
}

fn affine_hess_0_2(a11: f32, a12: f32, a21: f32, a22: f32) -> f32 {
    let out1 = 8f32 * a11 * a21 + 4f32 * a12 * a22;
    out1
}

fn affine_hess_0_3(a11: f32, a12: f32, a21: f32, a22: f32) -> f32 {
    let out1 = 4f32 * a12 * a21;
    out1
}

fn affine_hess_1_0(a11: f32, a12: f32, a21: f32, a22: f32) -> f32 {
    let out1 = 8f32 * a11 * a12 + 4f32 * a21 * a22;
    out1
}

fn affine_hess_1_1(a11: f32, a12: f32, a21: f32, a22: f32) -> f32 {
    let out1 = 4f32 * a11.powi(2) + 12f32 * a12.powi(2) + 4f32 * a22.powi(2) - 4f32;
    out1
}

fn affine_hess_1_2(a11: f32, a12: f32, a21: f32, a22: f32) -> f32 {
    let out1 = 4f32 * a11 * a22;
    out1
}

fn affine_hess_1_3(a11: f32, a12: f32, a21: f32, a22: f32) -> f32 {
    let out1 = 4f32 * a11 * a21 + 8f32 * a12 * a22;
    out1
}

fn affine_hess_2_0(a11: f32, a12: f32, a21: f32, a22: f32) -> f32 {
    let out1 = 8f32 * a11 * a21 + 4f32 * a12 * a22;
    out1
}

fn affine_hess_2_1(a11: f32, a12: f32, a21: f32, a22: f32) -> f32 {
    let out1 = 4f32 * a11 * a22;
    out1
}

fn affine_hess_2_2(a11: f32, a12: f32, a21: f32, a22: f32) -> f32 {
    let out1 = 4f32 * a11.powi(2) + 12f32 * a21.powi(2) + 4f32 * a22.powi(2) - 4f32;
    out1
}

fn affine_hess_2_3(a11: f32, a12: f32, a21: f32, a22: f32) -> f32 {
    let out1 = 4f32 * a11 * a12 + 8f32 * a21 * a22;
    out1
}

fn affine_hess_3_0(a11: f32, a12: f32, a21: f32, a22: f32) -> f32 {
    let out1 = 4f32 * a12 * a21;
    out1
}

fn affine_hess_3_1(a11: f32, a12: f32, a21: f32, a22: f32) -> f32 {
    let out1 = 4f32 * a11 * a21 + 8f32 * a12 * a22;
    out1
}

fn affine_hess_3_2(a11: f32, a12: f32, a21: f32, a22: f32) -> f32 {
    let out1 = 4f32 * a11 * a12 + 8f32 * a21 * a22;
    out1
}

fn affine_hess_3_3(a11: f32, a12: f32, a21: f32, a22: f32) -> f32 {
    let out1 = 4f32 * a12.powi(2) + 4f32 * a21.powi(2) + 12f32 * a22.powi(2) - 4f32;
    out1
}

pub fn subhess(a11: f32, a12: f32, a21: f32, a22: f32) -> glm::Mat4x4 {
    let mut res = glm::Mat4x4::zeros();

    res[(0, 0)] = affine_hess_0_0(a11, a12, a21, a22);
    res[(0, 1)] = affine_hess_0_1(a11, a12, a21, a22);
    res[(0, 2)] = affine_hess_0_2(a11, a12, a21, a22);
    res[(0, 3)] = affine_hess_0_3(a11, a12, a21, a22);

    res[(1, 0)] = affine_hess_1_0(a11, a12, a21, a22);
    res[(1, 1)] = affine_hess_1_1(a11, a12, a21, a22);
    res[(1, 2)] = affine_hess_1_2(a11, a12, a21, a22);
    res[(1, 3)] = affine_hess_1_3(a11, a12, a21, a22);

    res[(2, 0)] = affine_hess_2_0(a11, a12, a21, a22);
    res[(2, 1)] = affine_hess_2_1(a11, a12, a21, a22);
    res[(2, 2)] = affine_hess_2_2(a11, a12, a21, a22);
    res[(2, 3)] = affine_hess_2_3(a11, a12, a21, a22);

    res[(3, 0)] = affine_hess_3_0(a11, a12, a21, a22);
    res[(3, 1)] = affine_hess_3_1(a11, a12, a21, a22);
    res[(3, 2)] = affine_hess_3_2(a11, a12, a21, a22);
    res[(3, 3)] = affine_hess_3_3(a11, a12, a21, a22);

    res
}

pub fn orth_hess(a11: f32, a12: f32, a21: f32, a22: f32) -> Mat6x6 {
    let sub = subhess(a11, a12, a21, a22);
    let mut res = Mat6x6::zeros();

    for (i, j) in (0..4).zip(0..4) {
        res[(i + 2, j + 2)] = sub[(i, j)];
    }

    res
}
