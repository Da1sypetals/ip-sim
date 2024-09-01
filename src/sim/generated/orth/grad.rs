/*
 *                      Code generated with SymPy 1.13.2
 *
 *              See http://www.sympy.org/ for more information.
 *
 *                       This file is part of 'project'
 */

use crate::sim::utils::types::Vec6;

fn grad_0(a11: f32, a12: f32, a21: f32, a22: f32) -> f32 {
    let out1 =
        4f32 * a11 * (a11.powi(2) + a21.powi(2) - 1f32) + 4f32 * a12 * (a11 * a12 + a21 * a22);
    out1
}

fn grad_1(a11: f32, a12: f32, a21: f32, a22: f32) -> f32 {
    let out1 =
        4f32 * a11 * (a11 * a12 + a21 * a22) + 4f32 * a12 * (a12.powi(2) + a22.powi(2) - 1f32);
    out1
}

fn grad_2(a11: f32, a12: f32, a21: f32, a22: f32) -> f32 {
    let out1 =
        4f32 * a21 * (a11.powi(2) + a21.powi(2) - 1f32) + 4f32 * a22 * (a11 * a12 + a21 * a22);
    out1
}

fn grad_3(a11: f32, a12: f32, a21: f32, a22: f32) -> f32 {
    let out1 =
        4f32 * a21 * (a11 * a12 + a21 * a22) + 4f32 * a22 * (a12.powi(2) + a22.powi(2) - 1f32);
    out1
}

pub fn orth_grad(a11: f32, a12: f32, a21: f32, a22: f32) -> Vec6 {
    Vec6::new(
        0f32,
        0f32,
        grad_0(a11, a12, a21, a22),
        grad_1(a11, a12, a21, a22),
        grad_2(a11, a12, a21, a22),
        grad_3(a11, a12, a21, a22),
    )
}
