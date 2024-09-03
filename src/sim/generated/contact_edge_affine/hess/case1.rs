/*
 *                      Code generated with SymPy 1.13.2
 *
 *              See http://www.sympy.org/ for more information.
 *
 *                       This file is part of 'project'
 */

use crate::sim::utils::types::Mat6x6;

pub fn hess_edge_case1(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    px: f32,
    py: f32,
    tx: f32,
    ty: f32,
    u0x: f32,
    u0y: f32,
) -> Mat6x6 {
    Mat6x6::new(
        hess_edge_case1_0_0(a11, a12, a21, a22, px, py, tx, ty, u0x, u0y),
        hess_edge_case1_0_1(a11, a12, a21, a22, px, py, tx, ty, u0x, u0y),
        hess_edge_case1_0_2(a11, a12, a21, a22, px, py, tx, ty, u0x, u0y),
        hess_edge_case1_0_3(a11, a12, a21, a22, px, py, tx, ty, u0x, u0y),
        hess_edge_case1_0_4(a11, a12, a21, a22, px, py, tx, ty, u0x, u0y),
        hess_edge_case1_0_5(a11, a12, a21, a22, px, py, tx, ty, u0x, u0y),
        hess_edge_case1_1_0(a11, a12, a21, a22, px, py, tx, ty, u0x, u0y),
        hess_edge_case1_1_1(a11, a12, a21, a22, px, py, tx, ty, u0x, u0y),
        hess_edge_case1_1_2(a11, a12, a21, a22, px, py, tx, ty, u0x, u0y),
        hess_edge_case1_1_3(a11, a12, a21, a22, px, py, tx, ty, u0x, u0y),
        hess_edge_case1_1_4(a11, a12, a21, a22, px, py, tx, ty, u0x, u0y),
        hess_edge_case1_1_5(a11, a12, a21, a22, px, py, tx, ty, u0x, u0y),
        hess_edge_case1_2_0(a11, a12, a21, a22, px, py, tx, ty, u0x, u0y),
        hess_edge_case1_2_1(a11, a12, a21, a22, px, py, tx, ty, u0x, u0y),
        hess_edge_case1_2_2(a11, a12, a21, a22, px, py, tx, ty, u0x, u0y),
        hess_edge_case1_2_3(a11, a12, a21, a22, px, py, tx, ty, u0x, u0y),
        hess_edge_case1_2_4(a11, a12, a21, a22, px, py, tx, ty, u0x, u0y),
        hess_edge_case1_2_5(a11, a12, a21, a22, px, py, tx, ty, u0x, u0y),
        hess_edge_case1_3_0(a11, a12, a21, a22, px, py, tx, ty, u0x, u0y),
        hess_edge_case1_3_1(a11, a12, a21, a22, px, py, tx, ty, u0x, u0y),
        hess_edge_case1_3_2(a11, a12, a21, a22, px, py, tx, ty, u0x, u0y),
        hess_edge_case1_3_3(a11, a12, a21, a22, px, py, tx, ty, u0x, u0y),
        hess_edge_case1_3_4(a11, a12, a21, a22, px, py, tx, ty, u0x, u0y),
        hess_edge_case1_3_5(a11, a12, a21, a22, px, py, tx, ty, u0x, u0y),
        hess_edge_case1_4_0(a11, a12, a21, a22, px, py, tx, ty, u0x, u0y),
        hess_edge_case1_4_1(a11, a12, a21, a22, px, py, tx, ty, u0x, u0y),
        hess_edge_case1_4_2(a11, a12, a21, a22, px, py, tx, ty, u0x, u0y),
        hess_edge_case1_4_3(a11, a12, a21, a22, px, py, tx, ty, u0x, u0y),
        hess_edge_case1_4_4(a11, a12, a21, a22, px, py, tx, ty, u0x, u0y),
        hess_edge_case1_4_5(a11, a12, a21, a22, px, py, tx, ty, u0x, u0y),
        hess_edge_case1_5_0(a11, a12, a21, a22, px, py, tx, ty, u0x, u0y),
        hess_edge_case1_5_1(a11, a12, a21, a22, px, py, tx, ty, u0x, u0y),
        hess_edge_case1_5_2(a11, a12, a21, a22, px, py, tx, ty, u0x, u0y),
        hess_edge_case1_5_3(a11, a12, a21, a22, px, py, tx, ty, u0x, u0y),
        hess_edge_case1_5_4(a11, a12, a21, a22, px, py, tx, ty, u0x, u0y),
        hess_edge_case1_5_5(a11, a12, a21, a22, px, py, tx, ty, u0x, u0y),
    )
}

pub fn hess_edge_case1_0_0(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    px: f32,
    py: f32,
    tx: f32,
    ty: f32,
    u0x: f32,
    u0y: f32,
) -> f32 {
    let out1 = ((a11 * u0x + a12 * u0y - px + tx).powi(2)
        + (a21 * u0x + a22 * u0y - py + ty).powi(2))
    .sqrt()
    .recip()
        + (-a11 * u0x - a12 * u0y + px - tx) * (a11 * u0x + a12 * u0y - px + tx)
            / ((a11 * u0x + a12 * u0y - px + tx).powi(2)
                + (a21 * u0x + a22 * u0y - py + ty).powi(2))
            .powf(3_f32 / 2.0);
    out1
}

pub fn hess_edge_case1_0_1(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    px: f32,
    py: f32,
    tx: f32,
    ty: f32,
    u0x: f32,
    u0y: f32,
) -> f32 {
    let out1 = (-a11 * u0x - a12 * u0y + px - tx) * (a21 * u0x + a22 * u0y - py + ty)
        / ((a11 * u0x + a12 * u0y - px + tx).powi(2) + (a21 * u0x + a22 * u0y - py + ty).powi(2))
            .powf(3_f32 / 2.0);
    out1
}

pub fn hess_edge_case1_0_2(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    px: f32,
    py: f32,
    tx: f32,
    ty: f32,
    u0x: f32,
    u0y: f32,
) -> f32 {
    let out1 = u0x
        / ((a11 * u0x + a12 * u0y - px + tx).powi(2) + (a21 * u0x + a22 * u0y - py + ty).powi(2))
            .sqrt()
        + u0x * (-a11 * u0x - a12 * u0y + px - tx) * (a11 * u0x + a12 * u0y - px + tx)
            / ((a11 * u0x + a12 * u0y - px + tx).powi(2)
                + (a21 * u0x + a22 * u0y - py + ty).powi(2))
            .powf(3_f32 / 2.0);
    out1
}

pub fn hess_edge_case1_0_3(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    px: f32,
    py: f32,
    tx: f32,
    ty: f32,
    u0x: f32,
    u0y: f32,
) -> f32 {
    let out1 = u0y
        / ((a11 * u0x + a12 * u0y - px + tx).powi(2) + (a21 * u0x + a22 * u0y - py + ty).powi(2))
            .sqrt()
        + u0y * (-a11 * u0x - a12 * u0y + px - tx) * (a11 * u0x + a12 * u0y - px + tx)
            / ((a11 * u0x + a12 * u0y - px + tx).powi(2)
                + (a21 * u0x + a22 * u0y - py + ty).powi(2))
            .powf(3_f32 / 2.0);
    out1
}

pub fn hess_edge_case1_0_4(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    px: f32,
    py: f32,
    tx: f32,
    ty: f32,
    u0x: f32,
    u0y: f32,
) -> f32 {
    let out1 = u0x * (-a11 * u0x - a12 * u0y + px - tx) * (a21 * u0x + a22 * u0y - py + ty)
        / ((a11 * u0x + a12 * u0y - px + tx).powi(2) + (a21 * u0x + a22 * u0y - py + ty).powi(2))
            .powf(3_f32 / 2.0);
    out1
}

pub fn hess_edge_case1_0_5(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    px: f32,
    py: f32,
    tx: f32,
    ty: f32,
    u0x: f32,
    u0y: f32,
) -> f32 {
    let out1 = u0y * (-a11 * u0x - a12 * u0y + px - tx) * (a21 * u0x + a22 * u0y - py + ty)
        / ((a11 * u0x + a12 * u0y - px + tx).powi(2) + (a21 * u0x + a22 * u0y - py + ty).powi(2))
            .powf(3_f32 / 2.0);
    out1
}

pub fn hess_edge_case1_1_0(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    px: f32,
    py: f32,
    tx: f32,
    ty: f32,
    u0x: f32,
    u0y: f32,
) -> f32 {
    let out1 = (a11 * u0x + a12 * u0y - px + tx) * (-a21 * u0x - a22 * u0y + py - ty)
        / ((a11 * u0x + a12 * u0y - px + tx).powi(2) + (a21 * u0x + a22 * u0y - py + ty).powi(2))
            .powf(3_f32 / 2.0);
    out1
}

pub fn hess_edge_case1_1_1(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    px: f32,
    py: f32,
    tx: f32,
    ty: f32,
    u0x: f32,
    u0y: f32,
) -> f32 {
    let out1 = ((a11 * u0x + a12 * u0y - px + tx).powi(2)
        + (a21 * u0x + a22 * u0y - py + ty).powi(2))
    .sqrt()
    .recip()
        + (-a21 * u0x - a22 * u0y + py - ty) * (a21 * u0x + a22 * u0y - py + ty)
            / ((a11 * u0x + a12 * u0y - px + tx).powi(2)
                + (a21 * u0x + a22 * u0y - py + ty).powi(2))
            .powf(3_f32 / 2.0);
    out1
}

pub fn hess_edge_case1_1_2(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    px: f32,
    py: f32,
    tx: f32,
    ty: f32,
    u0x: f32,
    u0y: f32,
) -> f32 {
    let out1 = u0x * (a11 * u0x + a12 * u0y - px + tx) * (-a21 * u0x - a22 * u0y + py - ty)
        / ((a11 * u0x + a12 * u0y - px + tx).powi(2) + (a21 * u0x + a22 * u0y - py + ty).powi(2))
            .powf(3_f32 / 2.0);
    out1
}

pub fn hess_edge_case1_1_3(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    px: f32,
    py: f32,
    tx: f32,
    ty: f32,
    u0x: f32,
    u0y: f32,
) -> f32 {
    let out1 = u0y * (a11 * u0x + a12 * u0y - px + tx) * (-a21 * u0x - a22 * u0y + py - ty)
        / ((a11 * u0x + a12 * u0y - px + tx).powi(2) + (a21 * u0x + a22 * u0y - py + ty).powi(2))
            .powf(3_f32 / 2.0);
    out1
}

pub fn hess_edge_case1_1_4(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    px: f32,
    py: f32,
    tx: f32,
    ty: f32,
    u0x: f32,
    u0y: f32,
) -> f32 {
    let out1 = u0x
        / ((a11 * u0x + a12 * u0y - px + tx).powi(2) + (a21 * u0x + a22 * u0y - py + ty).powi(2))
            .sqrt()
        + u0x * (-a21 * u0x - a22 * u0y + py - ty) * (a21 * u0x + a22 * u0y - py + ty)
            / ((a11 * u0x + a12 * u0y - px + tx).powi(2)
                + (a21 * u0x + a22 * u0y - py + ty).powi(2))
            .powf(3_f32 / 2.0);
    out1
}

pub fn hess_edge_case1_1_5(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    px: f32,
    py: f32,
    tx: f32,
    ty: f32,
    u0x: f32,
    u0y: f32,
) -> f32 {
    let out1 = u0y
        / ((a11 * u0x + a12 * u0y - px + tx).powi(2) + (a21 * u0x + a22 * u0y - py + ty).powi(2))
            .sqrt()
        + u0y * (-a21 * u0x - a22 * u0y + py - ty) * (a21 * u0x + a22 * u0y - py + ty)
            / ((a11 * u0x + a12 * u0y - px + tx).powi(2)
                + (a21 * u0x + a22 * u0y - py + ty).powi(2))
            .powf(3_f32 / 2.0);
    out1
}

pub fn hess_edge_case1_2_0(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    px: f32,
    py: f32,
    tx: f32,
    ty: f32,
    u0x: f32,
    u0y: f32,
) -> f32 {
    let out1 = u0x
        / ((a11 * u0x + a12 * u0y - px + tx).powi(2) + (a21 * u0x + a22 * u0y - py + ty).powi(2))
            .sqrt()
        - u0x * (a11 * u0x + a12 * u0y - px + tx).powi(2)
            / ((a11 * u0x + a12 * u0y - px + tx).powi(2)
                + (a21 * u0x + a22 * u0y - py + ty).powi(2))
            .powf(3_f32 / 2.0);
    out1
}

pub fn hess_edge_case1_2_1(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    px: f32,
    py: f32,
    tx: f32,
    ty: f32,
    u0x: f32,
    u0y: f32,
) -> f32 {
    let out1 = -u0x * (a11 * u0x + a12 * u0y - px + tx) * (a21 * u0x + a22 * u0y - py + ty)
        / ((a11 * u0x + a12 * u0y - px + tx).powi(2) + (a21 * u0x + a22 * u0y - py + ty).powi(2))
            .powf(3_f32 / 2.0);
    out1
}

pub fn hess_edge_case1_2_2(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    px: f32,
    py: f32,
    tx: f32,
    ty: f32,
    u0x: f32,
    u0y: f32,
) -> f32 {
    let out1 = u0x.powi(2)
        / ((a11 * u0x + a12 * u0y - px + tx).powi(2) + (a21 * u0x + a22 * u0y - py + ty).powi(2))
            .sqrt()
        - u0x.powi(2) * (a11 * u0x + a12 * u0y - px + tx).powi(2)
            / ((a11 * u0x + a12 * u0y - px + tx).powi(2)
                + (a21 * u0x + a22 * u0y - py + ty).powi(2))
            .powf(3_f32 / 2.0);
    out1
}

pub fn hess_edge_case1_2_3(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    px: f32,
    py: f32,
    tx: f32,
    ty: f32,
    u0x: f32,
    u0y: f32,
) -> f32 {
    let out1 = u0x * u0y
        / ((a11 * u0x + a12 * u0y - px + tx).powi(2) + (a21 * u0x + a22 * u0y - py + ty).powi(2))
            .sqrt()
        - u0x * u0y * (a11 * u0x + a12 * u0y - px + tx).powi(2)
            / ((a11 * u0x + a12 * u0y - px + tx).powi(2)
                + (a21 * u0x + a22 * u0y - py + ty).powi(2))
            .powf(3_f32 / 2.0);
    out1
}

pub fn hess_edge_case1_2_4(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    px: f32,
    py: f32,
    tx: f32,
    ty: f32,
    u0x: f32,
    u0y: f32,
) -> f32 {
    let out1 = -u0x.powi(2) * (a11 * u0x + a12 * u0y - px + tx) * (a21 * u0x + a22 * u0y - py + ty)
        / ((a11 * u0x + a12 * u0y - px + tx).powi(2) + (a21 * u0x + a22 * u0y - py + ty).powi(2))
            .powf(3_f32 / 2.0);
    out1
}

pub fn hess_edge_case1_2_5(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    px: f32,
    py: f32,
    tx: f32,
    ty: f32,
    u0x: f32,
    u0y: f32,
) -> f32 {
    let out1 = -u0x * u0y * (a11 * u0x + a12 * u0y - px + tx) * (a21 * u0x + a22 * u0y - py + ty)
        / ((a11 * u0x + a12 * u0y - px + tx).powi(2) + (a21 * u0x + a22 * u0y - py + ty).powi(2))
            .powf(3_f32 / 2.0);
    out1
}

pub fn hess_edge_case1_3_0(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    px: f32,
    py: f32,
    tx: f32,
    ty: f32,
    u0x: f32,
    u0y: f32,
) -> f32 {
    let out1 = u0y
        / ((a11 * u0x + a12 * u0y - px + tx).powi(2) + (a21 * u0x + a22 * u0y - py + ty).powi(2))
            .sqrt()
        - u0y * (a11 * u0x + a12 * u0y - px + tx).powi(2)
            / ((a11 * u0x + a12 * u0y - px + tx).powi(2)
                + (a21 * u0x + a22 * u0y - py + ty).powi(2))
            .powf(3_f32 / 2.0);
    out1
}

pub fn hess_edge_case1_3_1(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    px: f32,
    py: f32,
    tx: f32,
    ty: f32,
    u0x: f32,
    u0y: f32,
) -> f32 {
    let out1 = -u0y * (a11 * u0x + a12 * u0y - px + tx) * (a21 * u0x + a22 * u0y - py + ty)
        / ((a11 * u0x + a12 * u0y - px + tx).powi(2) + (a21 * u0x + a22 * u0y - py + ty).powi(2))
            .powf(3_f32 / 2.0);
    out1
}

pub fn hess_edge_case1_3_2(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    px: f32,
    py: f32,
    tx: f32,
    ty: f32,
    u0x: f32,
    u0y: f32,
) -> f32 {
    let out1 = u0x * u0y
        / ((a11 * u0x + a12 * u0y - px + tx).powi(2) + (a21 * u0x + a22 * u0y - py + ty).powi(2))
            .sqrt()
        - u0x * u0y * (a11 * u0x + a12 * u0y - px + tx).powi(2)
            / ((a11 * u0x + a12 * u0y - px + tx).powi(2)
                + (a21 * u0x + a22 * u0y - py + ty).powi(2))
            .powf(3_f32 / 2.0);
    out1
}

pub fn hess_edge_case1_3_3(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    px: f32,
    py: f32,
    tx: f32,
    ty: f32,
    u0x: f32,
    u0y: f32,
) -> f32 {
    let out1 = u0y.powi(2)
        / ((a11 * u0x + a12 * u0y - px + tx).powi(2) + (a21 * u0x + a22 * u0y - py + ty).powi(2))
            .sqrt()
        - u0y.powi(2) * (a11 * u0x + a12 * u0y - px + tx).powi(2)
            / ((a11 * u0x + a12 * u0y - px + tx).powi(2)
                + (a21 * u0x + a22 * u0y - py + ty).powi(2))
            .powf(3_f32 / 2.0);
    out1
}

pub fn hess_edge_case1_3_4(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    px: f32,
    py: f32,
    tx: f32,
    ty: f32,
    u0x: f32,
    u0y: f32,
) -> f32 {
    let out1 = -u0x * u0y * (a11 * u0x + a12 * u0y - px + tx) * (a21 * u0x + a22 * u0y - py + ty)
        / ((a11 * u0x + a12 * u0y - px + tx).powi(2) + (a21 * u0x + a22 * u0y - py + ty).powi(2))
            .powf(3_f32 / 2.0);
    out1
}

pub fn hess_edge_case1_3_5(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    px: f32,
    py: f32,
    tx: f32,
    ty: f32,
    u0x: f32,
    u0y: f32,
) -> f32 {
    let out1 = -u0y.powi(2) * (a11 * u0x + a12 * u0y - px + tx) * (a21 * u0x + a22 * u0y - py + ty)
        / ((a11 * u0x + a12 * u0y - px + tx).powi(2) + (a21 * u0x + a22 * u0y - py + ty).powi(2))
            .powf(3_f32 / 2.0);
    out1
}

pub fn hess_edge_case1_4_0(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    px: f32,
    py: f32,
    tx: f32,
    ty: f32,
    u0x: f32,
    u0y: f32,
) -> f32 {
    let out1 = -u0x * (a11 * u0x + a12 * u0y - px + tx) * (a21 * u0x + a22 * u0y - py + ty)
        / ((a11 * u0x + a12 * u0y - px + tx).powi(2) + (a21 * u0x + a22 * u0y - py + ty).powi(2))
            .powf(3_f32 / 2.0);
    out1
}

pub fn hess_edge_case1_4_1(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    px: f32,
    py: f32,
    tx: f32,
    ty: f32,
    u0x: f32,
    u0y: f32,
) -> f32 {
    let out1 = u0x
        / ((a11 * u0x + a12 * u0y - px + tx).powi(2) + (a21 * u0x + a22 * u0y - py + ty).powi(2))
            .sqrt()
        - u0x * (a21 * u0x + a22 * u0y - py + ty).powi(2)
            / ((a11 * u0x + a12 * u0y - px + tx).powi(2)
                + (a21 * u0x + a22 * u0y - py + ty).powi(2))
            .powf(3_f32 / 2.0);
    out1
}

pub fn hess_edge_case1_4_2(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    px: f32,
    py: f32,
    tx: f32,
    ty: f32,
    u0x: f32,
    u0y: f32,
) -> f32 {
    let out1 = -u0x.powi(2) * (a11 * u0x + a12 * u0y - px + tx) * (a21 * u0x + a22 * u0y - py + ty)
        / ((a11 * u0x + a12 * u0y - px + tx).powi(2) + (a21 * u0x + a22 * u0y - py + ty).powi(2))
            .powf(3_f32 / 2.0);
    out1
}

pub fn hess_edge_case1_4_3(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    px: f32,
    py: f32,
    tx: f32,
    ty: f32,
    u0x: f32,
    u0y: f32,
) -> f32 {
    let out1 = -u0x * u0y * (a11 * u0x + a12 * u0y - px + tx) * (a21 * u0x + a22 * u0y - py + ty)
        / ((a11 * u0x + a12 * u0y - px + tx).powi(2) + (a21 * u0x + a22 * u0y - py + ty).powi(2))
            .powf(3_f32 / 2.0);
    out1
}

pub fn hess_edge_case1_4_4(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    px: f32,
    py: f32,
    tx: f32,
    ty: f32,
    u0x: f32,
    u0y: f32,
) -> f32 {
    let out1 = u0x.powi(2)
        / ((a11 * u0x + a12 * u0y - px + tx).powi(2) + (a21 * u0x + a22 * u0y - py + ty).powi(2))
            .sqrt()
        - u0x.powi(2) * (a21 * u0x + a22 * u0y - py + ty).powi(2)
            / ((a11 * u0x + a12 * u0y - px + tx).powi(2)
                + (a21 * u0x + a22 * u0y - py + ty).powi(2))
            .powf(3_f32 / 2.0);
    out1
}

pub fn hess_edge_case1_4_5(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    px: f32,
    py: f32,
    tx: f32,
    ty: f32,
    u0x: f32,
    u0y: f32,
) -> f32 {
    let out1 = u0x * u0y
        / ((a11 * u0x + a12 * u0y - px + tx).powi(2) + (a21 * u0x + a22 * u0y - py + ty).powi(2))
            .sqrt()
        - u0x * u0y * (a21 * u0x + a22 * u0y - py + ty).powi(2)
            / ((a11 * u0x + a12 * u0y - px + tx).powi(2)
                + (a21 * u0x + a22 * u0y - py + ty).powi(2))
            .powf(3_f32 / 2.0);
    out1
}

pub fn hess_edge_case1_5_0(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    px: f32,
    py: f32,
    tx: f32,
    ty: f32,
    u0x: f32,
    u0y: f32,
) -> f32 {
    let out1 = -u0y * (a11 * u0x + a12 * u0y - px + tx) * (a21 * u0x + a22 * u0y - py + ty)
        / ((a11 * u0x + a12 * u0y - px + tx).powi(2) + (a21 * u0x + a22 * u0y - py + ty).powi(2))
            .powf(3_f32 / 2.0);
    out1
}

pub fn hess_edge_case1_5_1(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    px: f32,
    py: f32,
    tx: f32,
    ty: f32,
    u0x: f32,
    u0y: f32,
) -> f32 {
    let out1 = u0y
        / ((a11 * u0x + a12 * u0y - px + tx).powi(2) + (a21 * u0x + a22 * u0y - py + ty).powi(2))
            .sqrt()
        - u0y * (a21 * u0x + a22 * u0y - py + ty).powi(2)
            / ((a11 * u0x + a12 * u0y - px + tx).powi(2)
                + (a21 * u0x + a22 * u0y - py + ty).powi(2))
            .powf(3_f32 / 2.0);
    out1
}

pub fn hess_edge_case1_5_2(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    px: f32,
    py: f32,
    tx: f32,
    ty: f32,
    u0x: f32,
    u0y: f32,
) -> f32 {
    let out1 = -u0x * u0y * (a11 * u0x + a12 * u0y - px + tx) * (a21 * u0x + a22 * u0y - py + ty)
        / ((a11 * u0x + a12 * u0y - px + tx).powi(2) + (a21 * u0x + a22 * u0y - py + ty).powi(2))
            .powf(3_f32 / 2.0);
    out1
}

pub fn hess_edge_case1_5_3(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    px: f32,
    py: f32,
    tx: f32,
    ty: f32,
    u0x: f32,
    u0y: f32,
) -> f32 {
    let out1 = -u0y.powi(2) * (a11 * u0x + a12 * u0y - px + tx) * (a21 * u0x + a22 * u0y - py + ty)
        / ((a11 * u0x + a12 * u0y - px + tx).powi(2) + (a21 * u0x + a22 * u0y - py + ty).powi(2))
            .powf(3_f32 / 2.0);
    out1
}

pub fn hess_edge_case1_5_4(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    px: f32,
    py: f32,
    tx: f32,
    ty: f32,
    u0x: f32,
    u0y: f32,
) -> f32 {
    let out1 = u0x * u0y
        / ((a11 * u0x + a12 * u0y - px + tx).powi(2) + (a21 * u0x + a22 * u0y - py + ty).powi(2))
            .sqrt()
        - u0x * u0y * (a21 * u0x + a22 * u0y - py + ty).powi(2)
            / ((a11 * u0x + a12 * u0y - px + tx).powi(2)
                + (a21 * u0x + a22 * u0y - py + ty).powi(2))
            .powf(3_f32 / 2.0);
    out1
}

pub fn hess_edge_case1_5_5(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    px: f32,
    py: f32,
    tx: f32,
    ty: f32,
    u0x: f32,
    u0y: f32,
) -> f32 {
    let out1 = u0y.powi(2)
        / ((a11 * u0x + a12 * u0y - px + tx).powi(2) + (a21 * u0x + a22 * u0y - py + ty).powi(2))
            .sqrt()
        - u0y.powi(2) * (a21 * u0x + a22 * u0y - py + ty).powi(2)
            / ((a11 * u0x + a12 * u0y - px + tx).powi(2)
                + (a21 * u0x + a22 * u0y - py + ty).powi(2))
            .powf(3_f32 / 2.0);
    out1
}
