/*
 *                      Code generated with SymPy 1.13.2
 *
 *              See http://www.sympy.org/ for more information.
 *
 *                       This file is part of 'project'
 */

use crate::sim::utils::types::Mat6x6;

pub fn hess_edge_case2(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    px: f32,
    py: f32,
    tx: f32,
    ty: f32,
    v0x: f32,
    v0y: f32,
) -> Mat6x6 {
    Mat6x6::new(
        hess_edge_case2_0_0(a11, a12, a21, a22, px, py, tx, ty, v0x, v0y),
        hess_edge_case2_0_1(a11, a12, a21, a22, px, py, tx, ty, v0x, v0y),
        hess_edge_case2_0_2(a11, a12, a21, a22, px, py, tx, ty, v0x, v0y),
        hess_edge_case2_0_3(a11, a12, a21, a22, px, py, tx, ty, v0x, v0y),
        hess_edge_case2_0_4(a11, a12, a21, a22, px, py, tx, ty, v0x, v0y),
        hess_edge_case2_0_5(a11, a12, a21, a22, px, py, tx, ty, v0x, v0y),
        hess_edge_case2_1_0(a11, a12, a21, a22, px, py, tx, ty, v0x, v0y),
        hess_edge_case2_1_1(a11, a12, a21, a22, px, py, tx, ty, v0x, v0y),
        hess_edge_case2_1_2(a11, a12, a21, a22, px, py, tx, ty, v0x, v0y),
        hess_edge_case2_1_3(a11, a12, a21, a22, px, py, tx, ty, v0x, v0y),
        hess_edge_case2_1_4(a11, a12, a21, a22, px, py, tx, ty, v0x, v0y),
        hess_edge_case2_1_5(a11, a12, a21, a22, px, py, tx, ty, v0x, v0y),
        hess_edge_case2_2_0(a11, a12, a21, a22, px, py, tx, ty, v0x, v0y),
        hess_edge_case2_2_1(a11, a12, a21, a22, px, py, tx, ty, v0x, v0y),
        hess_edge_case2_2_2(a11, a12, a21, a22, px, py, tx, ty, v0x, v0y),
        hess_edge_case2_2_3(a11, a12, a21, a22, px, py, tx, ty, v0x, v0y),
        hess_edge_case2_2_4(a11, a12, a21, a22, px, py, tx, ty, v0x, v0y),
        hess_edge_case2_2_5(a11, a12, a21, a22, px, py, tx, ty, v0x, v0y),
        hess_edge_case2_3_0(a11, a12, a21, a22, px, py, tx, ty, v0x, v0y),
        hess_edge_case2_3_1(a11, a12, a21, a22, px, py, tx, ty, v0x, v0y),
        hess_edge_case2_3_2(a11, a12, a21, a22, px, py, tx, ty, v0x, v0y),
        hess_edge_case2_3_3(a11, a12, a21, a22, px, py, tx, ty, v0x, v0y),
        hess_edge_case2_3_4(a11, a12, a21, a22, px, py, tx, ty, v0x, v0y),
        hess_edge_case2_3_5(a11, a12, a21, a22, px, py, tx, ty, v0x, v0y),
        hess_edge_case2_4_0(a11, a12, a21, a22, px, py, tx, ty, v0x, v0y),
        hess_edge_case2_4_1(a11, a12, a21, a22, px, py, tx, ty, v0x, v0y),
        hess_edge_case2_4_2(a11, a12, a21, a22, px, py, tx, ty, v0x, v0y),
        hess_edge_case2_4_3(a11, a12, a21, a22, px, py, tx, ty, v0x, v0y),
        hess_edge_case2_4_4(a11, a12, a21, a22, px, py, tx, ty, v0x, v0y),
        hess_edge_case2_4_5(a11, a12, a21, a22, px, py, tx, ty, v0x, v0y),
        hess_edge_case2_5_0(a11, a12, a21, a22, px, py, tx, ty, v0x, v0y),
        hess_edge_case2_5_1(a11, a12, a21, a22, px, py, tx, ty, v0x, v0y),
        hess_edge_case2_5_2(a11, a12, a21, a22, px, py, tx, ty, v0x, v0y),
        hess_edge_case2_5_3(a11, a12, a21, a22, px, py, tx, ty, v0x, v0y),
        hess_edge_case2_5_4(a11, a12, a21, a22, px, py, tx, ty, v0x, v0y),
        hess_edge_case2_5_5(a11, a12, a21, a22, px, py, tx, ty, v0x, v0y),
    )
}

pub fn hess_edge_case2_0_0(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    px: f32,
    py: f32,
    tx: f32,
    ty: f32,
    v0x: f32,
    v0y: f32,
) -> f32 {
    let out1 = ((a11 * v0x + a12 * v0y - px + tx).powi(2)
        + (a21 * v0x + a22 * v0y - py + ty).powi(2))
    .sqrt()
    .recip()
        + (-a11 * v0x - a12 * v0y + px - tx) * (a11 * v0x + a12 * v0y - px + tx)
            / ((a11 * v0x + a12 * v0y - px + tx).powi(2)
                + (a21 * v0x + a22 * v0y - py + ty).powi(2))
            .powf(3_f32 / 2.0);
    out1
}

pub fn hess_edge_case2_0_1(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    px: f32,
    py: f32,
    tx: f32,
    ty: f32,
    v0x: f32,
    v0y: f32,
) -> f32 {
    let out1 = (-a11 * v0x - a12 * v0y + px - tx) * (a21 * v0x + a22 * v0y - py + ty)
        / ((a11 * v0x + a12 * v0y - px + tx).powi(2) + (a21 * v0x + a22 * v0y - py + ty).powi(2))
            .powf(3_f32 / 2.0);
    out1
}

pub fn hess_edge_case2_0_2(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    px: f32,
    py: f32,
    tx: f32,
    ty: f32,
    v0x: f32,
    v0y: f32,
) -> f32 {
    let out1 = v0x
        / ((a11 * v0x + a12 * v0y - px + tx).powi(2) + (a21 * v0x + a22 * v0y - py + ty).powi(2))
            .sqrt()
        + v0x * (-a11 * v0x - a12 * v0y + px - tx) * (a11 * v0x + a12 * v0y - px + tx)
            / ((a11 * v0x + a12 * v0y - px + tx).powi(2)
                + (a21 * v0x + a22 * v0y - py + ty).powi(2))
            .powf(3_f32 / 2.0);
    out1
}

pub fn hess_edge_case2_0_3(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    px: f32,
    py: f32,
    tx: f32,
    ty: f32,
    v0x: f32,
    v0y: f32,
) -> f32 {
    let out1 = v0y
        / ((a11 * v0x + a12 * v0y - px + tx).powi(2) + (a21 * v0x + a22 * v0y - py + ty).powi(2))
            .sqrt()
        + v0y * (-a11 * v0x - a12 * v0y + px - tx) * (a11 * v0x + a12 * v0y - px + tx)
            / ((a11 * v0x + a12 * v0y - px + tx).powi(2)
                + (a21 * v0x + a22 * v0y - py + ty).powi(2))
            .powf(3_f32 / 2.0);
    out1
}

pub fn hess_edge_case2_0_4(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    px: f32,
    py: f32,
    tx: f32,
    ty: f32,
    v0x: f32,
    v0y: f32,
) -> f32 {
    let out1 = v0x * (-a11 * v0x - a12 * v0y + px - tx) * (a21 * v0x + a22 * v0y - py + ty)
        / ((a11 * v0x + a12 * v0y - px + tx).powi(2) + (a21 * v0x + a22 * v0y - py + ty).powi(2))
            .powf(3_f32 / 2.0);
    out1
}

pub fn hess_edge_case2_0_5(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    px: f32,
    py: f32,
    tx: f32,
    ty: f32,
    v0x: f32,
    v0y: f32,
) -> f32 {
    let out1 = v0y * (-a11 * v0x - a12 * v0y + px - tx) * (a21 * v0x + a22 * v0y - py + ty)
        / ((a11 * v0x + a12 * v0y - px + tx).powi(2) + (a21 * v0x + a22 * v0y - py + ty).powi(2))
            .powf(3_f32 / 2.0);
    out1
}

pub fn hess_edge_case2_1_0(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    px: f32,
    py: f32,
    tx: f32,
    ty: f32,
    v0x: f32,
    v0y: f32,
) -> f32 {
    let out1 = (a11 * v0x + a12 * v0y - px + tx) * (-a21 * v0x - a22 * v0y + py - ty)
        / ((a11 * v0x + a12 * v0y - px + tx).powi(2) + (a21 * v0x + a22 * v0y - py + ty).powi(2))
            .powf(3_f32 / 2.0);
    out1
}

pub fn hess_edge_case2_1_1(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    px: f32,
    py: f32,
    tx: f32,
    ty: f32,
    v0x: f32,
    v0y: f32,
) -> f32 {
    let out1 = ((a11 * v0x + a12 * v0y - px + tx).powi(2)
        + (a21 * v0x + a22 * v0y - py + ty).powi(2))
    .sqrt()
    .recip()
        + (-a21 * v0x - a22 * v0y + py - ty) * (a21 * v0x + a22 * v0y - py + ty)
            / ((a11 * v0x + a12 * v0y - px + tx).powi(2)
                + (a21 * v0x + a22 * v0y - py + ty).powi(2))
            .powf(3_f32 / 2.0);
    out1
}

pub fn hess_edge_case2_1_2(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    px: f32,
    py: f32,
    tx: f32,
    ty: f32,
    v0x: f32,
    v0y: f32,
) -> f32 {
    let out1 = v0x * (a11 * v0x + a12 * v0y - px + tx) * (-a21 * v0x - a22 * v0y + py - ty)
        / ((a11 * v0x + a12 * v0y - px + tx).powi(2) + (a21 * v0x + a22 * v0y - py + ty).powi(2))
            .powf(3_f32 / 2.0);
    out1
}

pub fn hess_edge_case2_1_3(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    px: f32,
    py: f32,
    tx: f32,
    ty: f32,
    v0x: f32,
    v0y: f32,
) -> f32 {
    let out1 = v0y * (a11 * v0x + a12 * v0y - px + tx) * (-a21 * v0x - a22 * v0y + py - ty)
        / ((a11 * v0x + a12 * v0y - px + tx).powi(2) + (a21 * v0x + a22 * v0y - py + ty).powi(2))
            .powf(3_f32 / 2.0);
    out1
}

pub fn hess_edge_case2_1_4(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    px: f32,
    py: f32,
    tx: f32,
    ty: f32,
    v0x: f32,
    v0y: f32,
) -> f32 {
    let out1 = v0x
        / ((a11 * v0x + a12 * v0y - px + tx).powi(2) + (a21 * v0x + a22 * v0y - py + ty).powi(2))
            .sqrt()
        + v0x * (-a21 * v0x - a22 * v0y + py - ty) * (a21 * v0x + a22 * v0y - py + ty)
            / ((a11 * v0x + a12 * v0y - px + tx).powi(2)
                + (a21 * v0x + a22 * v0y - py + ty).powi(2))
            .powf(3_f32 / 2.0);
    out1
}

pub fn hess_edge_case2_1_5(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    px: f32,
    py: f32,
    tx: f32,
    ty: f32,
    v0x: f32,
    v0y: f32,
) -> f32 {
    let out1 = v0y
        / ((a11 * v0x + a12 * v0y - px + tx).powi(2) + (a21 * v0x + a22 * v0y - py + ty).powi(2))
            .sqrt()
        + v0y * (-a21 * v0x - a22 * v0y + py - ty) * (a21 * v0x + a22 * v0y - py + ty)
            / ((a11 * v0x + a12 * v0y - px + tx).powi(2)
                + (a21 * v0x + a22 * v0y - py + ty).powi(2))
            .powf(3_f32 / 2.0);
    out1
}

pub fn hess_edge_case2_2_0(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    px: f32,
    py: f32,
    tx: f32,
    ty: f32,
    v0x: f32,
    v0y: f32,
) -> f32 {
    let out1 = v0x
        / ((a11 * v0x + a12 * v0y - px + tx).powi(2) + (a21 * v0x + a22 * v0y - py + ty).powi(2))
            .sqrt()
        - v0x * (a11 * v0x + a12 * v0y - px + tx).powi(2)
            / ((a11 * v0x + a12 * v0y - px + tx).powi(2)
                + (a21 * v0x + a22 * v0y - py + ty).powi(2))
            .powf(3_f32 / 2.0);
    out1
}

pub fn hess_edge_case2_2_1(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    px: f32,
    py: f32,
    tx: f32,
    ty: f32,
    v0x: f32,
    v0y: f32,
) -> f32 {
    let out1 = -v0x * (a11 * v0x + a12 * v0y - px + tx) * (a21 * v0x + a22 * v0y - py + ty)
        / ((a11 * v0x + a12 * v0y - px + tx).powi(2) + (a21 * v0x + a22 * v0y - py + ty).powi(2))
            .powf(3_f32 / 2.0);
    out1
}

pub fn hess_edge_case2_2_2(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    px: f32,
    py: f32,
    tx: f32,
    ty: f32,
    v0x: f32,
    v0y: f32,
) -> f32 {
    let out1 = v0x.powi(2)
        / ((a11 * v0x + a12 * v0y - px + tx).powi(2) + (a21 * v0x + a22 * v0y - py + ty).powi(2))
            .sqrt()
        - v0x.powi(2) * (a11 * v0x + a12 * v0y - px + tx).powi(2)
            / ((a11 * v0x + a12 * v0y - px + tx).powi(2)
                + (a21 * v0x + a22 * v0y - py + ty).powi(2))
            .powf(3_f32 / 2.0);
    out1
}

pub fn hess_edge_case2_2_3(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    px: f32,
    py: f32,
    tx: f32,
    ty: f32,
    v0x: f32,
    v0y: f32,
) -> f32 {
    let out1 = v0x * v0y
        / ((a11 * v0x + a12 * v0y - px + tx).powi(2) + (a21 * v0x + a22 * v0y - py + ty).powi(2))
            .sqrt()
        - v0x * v0y * (a11 * v0x + a12 * v0y - px + tx).powi(2)
            / ((a11 * v0x + a12 * v0y - px + tx).powi(2)
                + (a21 * v0x + a22 * v0y - py + ty).powi(2))
            .powf(3_f32 / 2.0);
    out1
}

pub fn hess_edge_case2_2_4(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    px: f32,
    py: f32,
    tx: f32,
    ty: f32,
    v0x: f32,
    v0y: f32,
) -> f32 {
    let out1 = -v0x.powi(2) * (a11 * v0x + a12 * v0y - px + tx) * (a21 * v0x + a22 * v0y - py + ty)
        / ((a11 * v0x + a12 * v0y - px + tx).powi(2) + (a21 * v0x + a22 * v0y - py + ty).powi(2))
            .powf(3_f32 / 2.0);
    out1
}

pub fn hess_edge_case2_2_5(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    px: f32,
    py: f32,
    tx: f32,
    ty: f32,
    v0x: f32,
    v0y: f32,
) -> f32 {
    let out1 = -v0x * v0y * (a11 * v0x + a12 * v0y - px + tx) * (a21 * v0x + a22 * v0y - py + ty)
        / ((a11 * v0x + a12 * v0y - px + tx).powi(2) + (a21 * v0x + a22 * v0y - py + ty).powi(2))
            .powf(3_f32 / 2.0);
    out1
}

pub fn hess_edge_case2_3_0(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    px: f32,
    py: f32,
    tx: f32,
    ty: f32,
    v0x: f32,
    v0y: f32,
) -> f32 {
    let out1 = v0y
        / ((a11 * v0x + a12 * v0y - px + tx).powi(2) + (a21 * v0x + a22 * v0y - py + ty).powi(2))
            .sqrt()
        - v0y * (a11 * v0x + a12 * v0y - px + tx).powi(2)
            / ((a11 * v0x + a12 * v0y - px + tx).powi(2)
                + (a21 * v0x + a22 * v0y - py + ty).powi(2))
            .powf(3_f32 / 2.0);
    out1
}

pub fn hess_edge_case2_3_1(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    px: f32,
    py: f32,
    tx: f32,
    ty: f32,
    v0x: f32,
    v0y: f32,
) -> f32 {
    let out1 = -v0y * (a11 * v0x + a12 * v0y - px + tx) * (a21 * v0x + a22 * v0y - py + ty)
        / ((a11 * v0x + a12 * v0y - px + tx).powi(2) + (a21 * v0x + a22 * v0y - py + ty).powi(2))
            .powf(3_f32 / 2.0);
    out1
}

pub fn hess_edge_case2_3_2(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    px: f32,
    py: f32,
    tx: f32,
    ty: f32,
    v0x: f32,
    v0y: f32,
) -> f32 {
    let out1 = v0x * v0y
        / ((a11 * v0x + a12 * v0y - px + tx).powi(2) + (a21 * v0x + a22 * v0y - py + ty).powi(2))
            .sqrt()
        - v0x * v0y * (a11 * v0x + a12 * v0y - px + tx).powi(2)
            / ((a11 * v0x + a12 * v0y - px + tx).powi(2)
                + (a21 * v0x + a22 * v0y - py + ty).powi(2))
            .powf(3_f32 / 2.0);
    out1
}

pub fn hess_edge_case2_3_3(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    px: f32,
    py: f32,
    tx: f32,
    ty: f32,
    v0x: f32,
    v0y: f32,
) -> f32 {
    let out1 = v0y.powi(2)
        / ((a11 * v0x + a12 * v0y - px + tx).powi(2) + (a21 * v0x + a22 * v0y - py + ty).powi(2))
            .sqrt()
        - v0y.powi(2) * (a11 * v0x + a12 * v0y - px + tx).powi(2)
            / ((a11 * v0x + a12 * v0y - px + tx).powi(2)
                + (a21 * v0x + a22 * v0y - py + ty).powi(2))
            .powf(3_f32 / 2.0);
    out1
}

pub fn hess_edge_case2_3_4(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    px: f32,
    py: f32,
    tx: f32,
    ty: f32,
    v0x: f32,
    v0y: f32,
) -> f32 {
    let out1 = -v0x * v0y * (a11 * v0x + a12 * v0y - px + tx) * (a21 * v0x + a22 * v0y - py + ty)
        / ((a11 * v0x + a12 * v0y - px + tx).powi(2) + (a21 * v0x + a22 * v0y - py + ty).powi(2))
            .powf(3_f32 / 2.0);
    out1
}

pub fn hess_edge_case2_3_5(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    px: f32,
    py: f32,
    tx: f32,
    ty: f32,
    v0x: f32,
    v0y: f32,
) -> f32 {
    let out1 = -v0y.powi(2) * (a11 * v0x + a12 * v0y - px + tx) * (a21 * v0x + a22 * v0y - py + ty)
        / ((a11 * v0x + a12 * v0y - px + tx).powi(2) + (a21 * v0x + a22 * v0y - py + ty).powi(2))
            .powf(3_f32 / 2.0);
    out1
}

pub fn hess_edge_case2_4_0(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    px: f32,
    py: f32,
    tx: f32,
    ty: f32,
    v0x: f32,
    v0y: f32,
) -> f32 {
    let out1 = -v0x * (a11 * v0x + a12 * v0y - px + tx) * (a21 * v0x + a22 * v0y - py + ty)
        / ((a11 * v0x + a12 * v0y - px + tx).powi(2) + (a21 * v0x + a22 * v0y - py + ty).powi(2))
            .powf(3_f32 / 2.0);
    out1
}

pub fn hess_edge_case2_4_1(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    px: f32,
    py: f32,
    tx: f32,
    ty: f32,
    v0x: f32,
    v0y: f32,
) -> f32 {
    let out1 = v0x
        / ((a11 * v0x + a12 * v0y - px + tx).powi(2) + (a21 * v0x + a22 * v0y - py + ty).powi(2))
            .sqrt()
        - v0x * (a21 * v0x + a22 * v0y - py + ty).powi(2)
            / ((a11 * v0x + a12 * v0y - px + tx).powi(2)
                + (a21 * v0x + a22 * v0y - py + ty).powi(2))
            .powf(3_f32 / 2.0);
    out1
}

pub fn hess_edge_case2_4_2(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    px: f32,
    py: f32,
    tx: f32,
    ty: f32,
    v0x: f32,
    v0y: f32,
) -> f32 {
    let out1 = -v0x.powi(2) * (a11 * v0x + a12 * v0y - px + tx) * (a21 * v0x + a22 * v0y - py + ty)
        / ((a11 * v0x + a12 * v0y - px + tx).powi(2) + (a21 * v0x + a22 * v0y - py + ty).powi(2))
            .powf(3_f32 / 2.0);
    out1
}

pub fn hess_edge_case2_4_3(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    px: f32,
    py: f32,
    tx: f32,
    ty: f32,
    v0x: f32,
    v0y: f32,
) -> f32 {
    let out1 = -v0x * v0y * (a11 * v0x + a12 * v0y - px + tx) * (a21 * v0x + a22 * v0y - py + ty)
        / ((a11 * v0x + a12 * v0y - px + tx).powi(2) + (a21 * v0x + a22 * v0y - py + ty).powi(2))
            .powf(3_f32 / 2.0);
    out1
}

pub fn hess_edge_case2_4_4(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    px: f32,
    py: f32,
    tx: f32,
    ty: f32,
    v0x: f32,
    v0y: f32,
) -> f32 {
    let out1 = v0x.powi(2)
        / ((a11 * v0x + a12 * v0y - px + tx).powi(2) + (a21 * v0x + a22 * v0y - py + ty).powi(2))
            .sqrt()
        - v0x.powi(2) * (a21 * v0x + a22 * v0y - py + ty).powi(2)
            / ((a11 * v0x + a12 * v0y - px + tx).powi(2)
                + (a21 * v0x + a22 * v0y - py + ty).powi(2))
            .powf(3_f32 / 2.0);
    out1
}

pub fn hess_edge_case2_4_5(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    px: f32,
    py: f32,
    tx: f32,
    ty: f32,
    v0x: f32,
    v0y: f32,
) -> f32 {
    let out1 = v0x * v0y
        / ((a11 * v0x + a12 * v0y - px + tx).powi(2) + (a21 * v0x + a22 * v0y - py + ty).powi(2))
            .sqrt()
        - v0x * v0y * (a21 * v0x + a22 * v0y - py + ty).powi(2)
            / ((a11 * v0x + a12 * v0y - px + tx).powi(2)
                + (a21 * v0x + a22 * v0y - py + ty).powi(2))
            .powf(3_f32 / 2.0);
    out1
}

pub fn hess_edge_case2_5_0(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    px: f32,
    py: f32,
    tx: f32,
    ty: f32,
    v0x: f32,
    v0y: f32,
) -> f32 {
    let out1 = -v0y * (a11 * v0x + a12 * v0y - px + tx) * (a21 * v0x + a22 * v0y - py + ty)
        / ((a11 * v0x + a12 * v0y - px + tx).powi(2) + (a21 * v0x + a22 * v0y - py + ty).powi(2))
            .powf(3_f32 / 2.0);
    out1
}

pub fn hess_edge_case2_5_1(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    px: f32,
    py: f32,
    tx: f32,
    ty: f32,
    v0x: f32,
    v0y: f32,
) -> f32 {
    let out1 = v0y
        / ((a11 * v0x + a12 * v0y - px + tx).powi(2) + (a21 * v0x + a22 * v0y - py + ty).powi(2))
            .sqrt()
        - v0y * (a21 * v0x + a22 * v0y - py + ty).powi(2)
            / ((a11 * v0x + a12 * v0y - px + tx).powi(2)
                + (a21 * v0x + a22 * v0y - py + ty).powi(2))
            .powf(3_f32 / 2.0);
    out1
}

pub fn hess_edge_case2_5_2(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    px: f32,
    py: f32,
    tx: f32,
    ty: f32,
    v0x: f32,
    v0y: f32,
) -> f32 {
    let out1 = -v0x * v0y * (a11 * v0x + a12 * v0y - px + tx) * (a21 * v0x + a22 * v0y - py + ty)
        / ((a11 * v0x + a12 * v0y - px + tx).powi(2) + (a21 * v0x + a22 * v0y - py + ty).powi(2))
            .powf(3_f32 / 2.0);
    out1
}

pub fn hess_edge_case2_5_3(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    px: f32,
    py: f32,
    tx: f32,
    ty: f32,
    v0x: f32,
    v0y: f32,
) -> f32 {
    let out1 = -v0y.powi(2) * (a11 * v0x + a12 * v0y - px + tx) * (a21 * v0x + a22 * v0y - py + ty)
        / ((a11 * v0x + a12 * v0y - px + tx).powi(2) + (a21 * v0x + a22 * v0y - py + ty).powi(2))
            .powf(3_f32 / 2.0);
    out1
}

pub fn hess_edge_case2_5_4(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    px: f32,
    py: f32,
    tx: f32,
    ty: f32,
    v0x: f32,
    v0y: f32,
) -> f32 {
    let out1 = v0x * v0y
        / ((a11 * v0x + a12 * v0y - px + tx).powi(2) + (a21 * v0x + a22 * v0y - py + ty).powi(2))
            .sqrt()
        - v0x * v0y * (a21 * v0x + a22 * v0y - py + ty).powi(2)
            / ((a11 * v0x + a12 * v0y - px + tx).powi(2)
                + (a21 * v0x + a22 * v0y - py + ty).powi(2))
            .powf(3_f32 / 2.0);
    out1
}

pub fn hess_edge_case2_5_5(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    px: f32,
    py: f32,
    tx: f32,
    ty: f32,
    v0x: f32,
    v0y: f32,
) -> f32 {
    let out1 = v0y.powi(2)
        / ((a11 * v0x + a12 * v0y - px + tx).powi(2) + (a21 * v0x + a22 * v0y - py + ty).powi(2))
            .sqrt()
        - v0y.powi(2) * (a21 * v0x + a22 * v0y - py + ty).powi(2)
            / ((a11 * v0x + a12 * v0y - px + tx).powi(2)
                + (a21 * v0x + a22 * v0y - py + ty).powi(2))
            .powf(3_f32 / 2.0);
    out1
}
