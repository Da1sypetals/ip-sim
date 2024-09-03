/*
 *                      Code generated with SymPy 1.13.2
 *
 *              See http://www.sympy.org/ for more information.
 *
 *                       This file is part of 'project'
 */

use crate::sim::utils::types::Mat6x6;

pub fn hess_case2(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    px: f32,
    py: f32,
    tx: f32,
    ty: f32,
    vx: f32,
    vy: f32,
) -> Mat6x6 {
    Mat6x6::new(
        hess_case2_0_0(a11, a12, a21, a22, px, py, tx, ty, vx, vy),
        hess_case2_0_1(a11, a12, a21, a22, px, py, tx, ty, vx, vy),
        hess_case2_0_2(a11, a12, a21, a22, px, py, tx, ty, vx, vy),
        hess_case2_0_3(a11, a12, a21, a22, px, py, tx, ty, vx, vy),
        hess_case2_0_4(a11, a12, a21, a22, px, py, tx, ty, vx, vy),
        hess_case2_0_5(a11, a12, a21, a22, px, py, tx, ty, vx, vy),
        hess_case2_1_0(a11, a12, a21, a22, px, py, tx, ty, vx, vy),
        hess_case2_1_1(a11, a12, a21, a22, px, py, tx, ty, vx, vy),
        hess_case2_1_2(a11, a12, a21, a22, px, py, tx, ty, vx, vy),
        hess_case2_1_3(a11, a12, a21, a22, px, py, tx, ty, vx, vy),
        hess_case2_1_4(a11, a12, a21, a22, px, py, tx, ty, vx, vy),
        hess_case2_1_5(a11, a12, a21, a22, px, py, tx, ty, vx, vy),
        hess_case2_2_0(a11, a12, a21, a22, px, py, tx, ty, vx, vy),
        hess_case2_2_1(a11, a12, a21, a22, px, py, tx, ty, vx, vy),
        hess_case2_2_2(a11, a12, a21, a22, px, py, tx, ty, vx, vy),
        hess_case2_2_3(a11, a12, a21, a22, px, py, tx, ty, vx, vy),
        hess_case2_2_4(a11, a12, a21, a22, px, py, tx, ty, vx, vy),
        hess_case2_2_5(a11, a12, a21, a22, px, py, tx, ty, vx, vy),
        hess_case2_3_0(a11, a12, a21, a22, px, py, tx, ty, vx, vy),
        hess_case2_3_1(a11, a12, a21, a22, px, py, tx, ty, vx, vy),
        hess_case2_3_2(a11, a12, a21, a22, px, py, tx, ty, vx, vy),
        hess_case2_3_3(a11, a12, a21, a22, px, py, tx, ty, vx, vy),
        hess_case2_3_4(a11, a12, a21, a22, px, py, tx, ty, vx, vy),
        hess_case2_3_5(a11, a12, a21, a22, px, py, tx, ty, vx, vy),
        hess_case2_4_0(a11, a12, a21, a22, px, py, tx, ty, vx, vy),
        hess_case2_4_1(a11, a12, a21, a22, px, py, tx, ty, vx, vy),
        hess_case2_4_2(a11, a12, a21, a22, px, py, tx, ty, vx, vy),
        hess_case2_4_3(a11, a12, a21, a22, px, py, tx, ty, vx, vy),
        hess_case2_4_4(a11, a12, a21, a22, px, py, tx, ty, vx, vy),
        hess_case2_4_5(a11, a12, a21, a22, px, py, tx, ty, vx, vy),
        hess_case2_5_0(a11, a12, a21, a22, px, py, tx, ty, vx, vy),
        hess_case2_5_1(a11, a12, a21, a22, px, py, tx, ty, vx, vy),
        hess_case2_5_2(a11, a12, a21, a22, px, py, tx, ty, vx, vy),
        hess_case2_5_3(a11, a12, a21, a22, px, py, tx, ty, vx, vy),
        hess_case2_5_4(a11, a12, a21, a22, px, py, tx, ty, vx, vy),
        hess_case2_5_5(a11, a12, a21, a22, px, py, tx, ty, vx, vy),
    )
}

fn hess_case2_0_0(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    px: f32,
    py: f32,
    tx: f32,
    ty: f32,
    vx: f32,
    vy: f32,
) -> f32 {
    let out1 = ((a11 * px + a12 * py + tx - vx).powi(2) + (a21 * px + a22 * py + ty - vy).powi(2))
        .sqrt()
        .recip()
        + (-a11 * px - a12 * py - tx + vx) * (a11 * px + a12 * py + tx - vx)
            / ((a11 * px + a12 * py + tx - vx).powi(2) + (a21 * px + a22 * py + ty - vy).powi(2))
                .powf(3_f32 / 2.0);
    out1
}

fn hess_case2_0_1(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    px: f32,
    py: f32,
    tx: f32,
    ty: f32,
    vx: f32,
    vy: f32,
) -> f32 {
    let out1 = (a11 * px + a12 * py + tx - vx) * (-a21 * px - a22 * py - ty + vy)
        / ((a11 * px + a12 * py + tx - vx).powi(2) + (a21 * px + a22 * py + ty - vy).powi(2))
            .powf(3_f32 / 2.0);
    out1
}

fn hess_case2_0_2(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    px: f32,
    py: f32,
    tx: f32,
    ty: f32,
    vx: f32,
    vy: f32,
) -> f32 {
    let out1 = px
        / ((a11 * px + a12 * py + tx - vx).powi(2) + (a21 * px + a22 * py + ty - vy).powi(2))
            .sqrt()
        - px * (a11 * px + a12 * py + tx - vx).powi(2)
            / ((a11 * px + a12 * py + tx - vx).powi(2) + (a21 * px + a22 * py + ty - vy).powi(2))
                .powf(3_f32 / 2.0);
    out1
}

fn hess_case2_0_3(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    px: f32,
    py: f32,
    tx: f32,
    ty: f32,
    vx: f32,
    vy: f32,
) -> f32 {
    let out1 = py
        / ((a11 * px + a12 * py + tx - vx).powi(2) + (a21 * px + a22 * py + ty - vy).powi(2))
            .sqrt()
        - py * (a11 * px + a12 * py + tx - vx).powi(2)
            / ((a11 * px + a12 * py + tx - vx).powi(2) + (a21 * px + a22 * py + ty - vy).powi(2))
                .powf(3_f32 / 2.0);
    out1
}

fn hess_case2_0_4(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    px: f32,
    py: f32,
    tx: f32,
    ty: f32,
    vx: f32,
    vy: f32,
) -> f32 {
    let out1 = -px * (a11 * px + a12 * py + tx - vx) * (a21 * px + a22 * py + ty - vy)
        / ((a11 * px + a12 * py + tx - vx).powi(2) + (a21 * px + a22 * py + ty - vy).powi(2))
            .powf(3_f32 / 2.0);
    out1
}

fn hess_case2_0_5(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    px: f32,
    py: f32,
    tx: f32,
    ty: f32,
    vx: f32,
    vy: f32,
) -> f32 {
    let out1 = -py * (a11 * px + a12 * py + tx - vx) * (a21 * px + a22 * py + ty - vy)
        / ((a11 * px + a12 * py + tx - vx).powi(2) + (a21 * px + a22 * py + ty - vy).powi(2))
            .powf(3_f32 / 2.0);
    out1
}

fn hess_case2_1_0(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    px: f32,
    py: f32,
    tx: f32,
    ty: f32,
    vx: f32,
    vy: f32,
) -> f32 {
    let out1 = (-a11 * px - a12 * py - tx + vx) * (a21 * px + a22 * py + ty - vy)
        / ((a11 * px + a12 * py + tx - vx).powi(2) + (a21 * px + a22 * py + ty - vy).powi(2))
            .powf(3_f32 / 2.0);
    out1
}

fn hess_case2_1_1(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    px: f32,
    py: f32,
    tx: f32,
    ty: f32,
    vx: f32,
    vy: f32,
) -> f32 {
    let out1 = ((a11 * px + a12 * py + tx - vx).powi(2) + (a21 * px + a22 * py + ty - vy).powi(2))
        .sqrt()
        .recip()
        + (-a21 * px - a22 * py - ty + vy) * (a21 * px + a22 * py + ty - vy)
            / ((a11 * px + a12 * py + tx - vx).powi(2) + (a21 * px + a22 * py + ty - vy).powi(2))
                .powf(3_f32 / 2.0);
    out1
}

fn hess_case2_1_2(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    px: f32,
    py: f32,
    tx: f32,
    ty: f32,
    vx: f32,
    vy: f32,
) -> f32 {
    let out1 = -px * (a11 * px + a12 * py + tx - vx) * (a21 * px + a22 * py + ty - vy)
        / ((a11 * px + a12 * py + tx - vx).powi(2) + (a21 * px + a22 * py + ty - vy).powi(2))
            .powf(3_f32 / 2.0);
    out1
}

fn hess_case2_1_3(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    px: f32,
    py: f32,
    tx: f32,
    ty: f32,
    vx: f32,
    vy: f32,
) -> f32 {
    let out1 = -py * (a11 * px + a12 * py + tx - vx) * (a21 * px + a22 * py + ty - vy)
        / ((a11 * px + a12 * py + tx - vx).powi(2) + (a21 * px + a22 * py + ty - vy).powi(2))
            .powf(3_f32 / 2.0);
    out1
}

fn hess_case2_1_4(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    px: f32,
    py: f32,
    tx: f32,
    ty: f32,
    vx: f32,
    vy: f32,
) -> f32 {
    let out1 = px
        / ((a11 * px + a12 * py + tx - vx).powi(2) + (a21 * px + a22 * py + ty - vy).powi(2))
            .sqrt()
        - px * (a21 * px + a22 * py + ty - vy).powi(2)
            / ((a11 * px + a12 * py + tx - vx).powi(2) + (a21 * px + a22 * py + ty - vy).powi(2))
                .powf(3_f32 / 2.0);
    out1
}

fn hess_case2_1_5(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    px: f32,
    py: f32,
    tx: f32,
    ty: f32,
    vx: f32,
    vy: f32,
) -> f32 {
    let out1 = py
        / ((a11 * px + a12 * py + tx - vx).powi(2) + (a21 * px + a22 * py + ty - vy).powi(2))
            .sqrt()
        - py * (a21 * px + a22 * py + ty - vy).powi(2)
            / ((a11 * px + a12 * py + tx - vx).powi(2) + (a21 * px + a22 * py + ty - vy).powi(2))
                .powf(3_f32 / 2.0);
    out1
}

fn hess_case2_2_0(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    px: f32,
    py: f32,
    tx: f32,
    ty: f32,
    vx: f32,
    vy: f32,
) -> f32 {
    let out1 = px
        / ((a11 * px + a12 * py + tx - vx).powi(2) + (a21 * px + a22 * py + ty - vy).powi(2))
            .sqrt()
        + px * (-a11 * px - a12 * py - tx + vx) * (a11 * px + a12 * py + tx - vx)
            / ((a11 * px + a12 * py + tx - vx).powi(2) + (a21 * px + a22 * py + ty - vy).powi(2))
                .powf(3_f32 / 2.0);
    out1
}

fn hess_case2_2_1(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    px: f32,
    py: f32,
    tx: f32,
    ty: f32,
    vx: f32,
    vy: f32,
) -> f32 {
    let out1 = px * (a11 * px + a12 * py + tx - vx) * (-a21 * px - a22 * py - ty + vy)
        / ((a11 * px + a12 * py + tx - vx).powi(2) + (a21 * px + a22 * py + ty - vy).powi(2))
            .powf(3_f32 / 2.0);
    out1
}

fn hess_case2_2_2(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    px: f32,
    py: f32,
    tx: f32,
    ty: f32,
    vx: f32,
    vy: f32,
) -> f32 {
    let out1 = px.powi(2)
        / ((a11 * px + a12 * py + tx - vx).powi(2) + (a21 * px + a22 * py + ty - vy).powi(2))
            .sqrt()
        - px.powi(2) * (a11 * px + a12 * py + tx - vx).powi(2)
            / ((a11 * px + a12 * py + tx - vx).powi(2) + (a21 * px + a22 * py + ty - vy).powi(2))
                .powf(3_f32 / 2.0);
    out1
}

fn hess_case2_2_3(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    px: f32,
    py: f32,
    tx: f32,
    ty: f32,
    vx: f32,
    vy: f32,
) -> f32 {
    let out1 = px * py
        / ((a11 * px + a12 * py + tx - vx).powi(2) + (a21 * px + a22 * py + ty - vy).powi(2))
            .sqrt()
        - px * py * (a11 * px + a12 * py + tx - vx).powi(2)
            / ((a11 * px + a12 * py + tx - vx).powi(2) + (a21 * px + a22 * py + ty - vy).powi(2))
                .powf(3_f32 / 2.0);
    out1
}

fn hess_case2_2_4(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    px: f32,
    py: f32,
    tx: f32,
    ty: f32,
    vx: f32,
    vy: f32,
) -> f32 {
    let out1 = -px.powi(2) * (a11 * px + a12 * py + tx - vx) * (a21 * px + a22 * py + ty - vy)
        / ((a11 * px + a12 * py + tx - vx).powi(2) + (a21 * px + a22 * py + ty - vy).powi(2))
            .powf(3_f32 / 2.0);
    out1
}

fn hess_case2_2_5(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    px: f32,
    py: f32,
    tx: f32,
    ty: f32,
    vx: f32,
    vy: f32,
) -> f32 {
    let out1 = -px * py * (a11 * px + a12 * py + tx - vx) * (a21 * px + a22 * py + ty - vy)
        / ((a11 * px + a12 * py + tx - vx).powi(2) + (a21 * px + a22 * py + ty - vy).powi(2))
            .powf(3_f32 / 2.0);
    out1
}

fn hess_case2_3_0(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    px: f32,
    py: f32,
    tx: f32,
    ty: f32,
    vx: f32,
    vy: f32,
) -> f32 {
    let out1 = py
        / ((a11 * px + a12 * py + tx - vx).powi(2) + (a21 * px + a22 * py + ty - vy).powi(2))
            .sqrt()
        + py * (-a11 * px - a12 * py - tx + vx) * (a11 * px + a12 * py + tx - vx)
            / ((a11 * px + a12 * py + tx - vx).powi(2) + (a21 * px + a22 * py + ty - vy).powi(2))
                .powf(3_f32 / 2.0);
    out1
}

fn hess_case2_3_1(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    px: f32,
    py: f32,
    tx: f32,
    ty: f32,
    vx: f32,
    vy: f32,
) -> f32 {
    let out1 = py * (a11 * px + a12 * py + tx - vx) * (-a21 * px - a22 * py - ty + vy)
        / ((a11 * px + a12 * py + tx - vx).powi(2) + (a21 * px + a22 * py + ty - vy).powi(2))
            .powf(3_f32 / 2.0);
    out1
}

fn hess_case2_3_2(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    px: f32,
    py: f32,
    tx: f32,
    ty: f32,
    vx: f32,
    vy: f32,
) -> f32 {
    let out1 = px * py
        / ((a11 * px + a12 * py + tx - vx).powi(2) + (a21 * px + a22 * py + ty - vy).powi(2))
            .sqrt()
        - px * py * (a11 * px + a12 * py + tx - vx).powi(2)
            / ((a11 * px + a12 * py + tx - vx).powi(2) + (a21 * px + a22 * py + ty - vy).powi(2))
                .powf(3_f32 / 2.0);
    out1
}

fn hess_case2_3_3(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    px: f32,
    py: f32,
    tx: f32,
    ty: f32,
    vx: f32,
    vy: f32,
) -> f32 {
    let out1 = py.powi(2)
        / ((a11 * px + a12 * py + tx - vx).powi(2) + (a21 * px + a22 * py + ty - vy).powi(2))
            .sqrt()
        - py.powi(2) * (a11 * px + a12 * py + tx - vx).powi(2)
            / ((a11 * px + a12 * py + tx - vx).powi(2) + (a21 * px + a22 * py + ty - vy).powi(2))
                .powf(3_f32 / 2.0);
    out1
}

fn hess_case2_3_4(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    px: f32,
    py: f32,
    tx: f32,
    ty: f32,
    vx: f32,
    vy: f32,
) -> f32 {
    let out1 = -px * py * (a11 * px + a12 * py + tx - vx) * (a21 * px + a22 * py + ty - vy)
        / ((a11 * px + a12 * py + tx - vx).powi(2) + (a21 * px + a22 * py + ty - vy).powi(2))
            .powf(3_f32 / 2.0);
    out1
}

fn hess_case2_3_5(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    px: f32,
    py: f32,
    tx: f32,
    ty: f32,
    vx: f32,
    vy: f32,
) -> f32 {
    let out1 = -py.powi(2) * (a11 * px + a12 * py + tx - vx) * (a21 * px + a22 * py + ty - vy)
        / ((a11 * px + a12 * py + tx - vx).powi(2) + (a21 * px + a22 * py + ty - vy).powi(2))
            .powf(3_f32 / 2.0);
    out1
}

fn hess_case2_4_0(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    px: f32,
    py: f32,
    tx: f32,
    ty: f32,
    vx: f32,
    vy: f32,
) -> f32 {
    let out1 = px * (-a11 * px - a12 * py - tx + vx) * (a21 * px + a22 * py + ty - vy)
        / ((a11 * px + a12 * py + tx - vx).powi(2) + (a21 * px + a22 * py + ty - vy).powi(2))
            .powf(3_f32 / 2.0);
    out1
}

fn hess_case2_4_1(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    px: f32,
    py: f32,
    tx: f32,
    ty: f32,
    vx: f32,
    vy: f32,
) -> f32 {
    let out1 = px
        / ((a11 * px + a12 * py + tx - vx).powi(2) + (a21 * px + a22 * py + ty - vy).powi(2))
            .sqrt()
        + px * (-a21 * px - a22 * py - ty + vy) * (a21 * px + a22 * py + ty - vy)
            / ((a11 * px + a12 * py + tx - vx).powi(2) + (a21 * px + a22 * py + ty - vy).powi(2))
                .powf(3_f32 / 2.0);
    out1
}

fn hess_case2_4_2(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    px: f32,
    py: f32,
    tx: f32,
    ty: f32,
    vx: f32,
    vy: f32,
) -> f32 {
    let out1 = -px.powi(2) * (a11 * px + a12 * py + tx - vx) * (a21 * px + a22 * py + ty - vy)
        / ((a11 * px + a12 * py + tx - vx).powi(2) + (a21 * px + a22 * py + ty - vy).powi(2))
            .powf(3_f32 / 2.0);
    out1
}

fn hess_case2_4_3(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    px: f32,
    py: f32,
    tx: f32,
    ty: f32,
    vx: f32,
    vy: f32,
) -> f32 {
    let out1 = -px * py * (a11 * px + a12 * py + tx - vx) * (a21 * px + a22 * py + ty - vy)
        / ((a11 * px + a12 * py + tx - vx).powi(2) + (a21 * px + a22 * py + ty - vy).powi(2))
            .powf(3_f32 / 2.0);
    out1
}

fn hess_case2_4_4(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    px: f32,
    py: f32,
    tx: f32,
    ty: f32,
    vx: f32,
    vy: f32,
) -> f32 {
    let out1 = px.powi(2)
        / ((a11 * px + a12 * py + tx - vx).powi(2) + (a21 * px + a22 * py + ty - vy).powi(2))
            .sqrt()
        - px.powi(2) * (a21 * px + a22 * py + ty - vy).powi(2)
            / ((a11 * px + a12 * py + tx - vx).powi(2) + (a21 * px + a22 * py + ty - vy).powi(2))
                .powf(3_f32 / 2.0);
    out1
}

fn hess_case2_4_5(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    px: f32,
    py: f32,
    tx: f32,
    ty: f32,
    vx: f32,
    vy: f32,
) -> f32 {
    let out1 = px * py
        / ((a11 * px + a12 * py + tx - vx).powi(2) + (a21 * px + a22 * py + ty - vy).powi(2))
            .sqrt()
        - px * py * (a21 * px + a22 * py + ty - vy).powi(2)
            / ((a11 * px + a12 * py + tx - vx).powi(2) + (a21 * px + a22 * py + ty - vy).powi(2))
                .powf(3_f32 / 2.0);
    out1
}

fn hess_case2_5_0(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    px: f32,
    py: f32,
    tx: f32,
    ty: f32,
    vx: f32,
    vy: f32,
) -> f32 {
    let out1 = py * (-a11 * px - a12 * py - tx + vx) * (a21 * px + a22 * py + ty - vy)
        / ((a11 * px + a12 * py + tx - vx).powi(2) + (a21 * px + a22 * py + ty - vy).powi(2))
            .powf(3_f32 / 2.0);
    out1
}

fn hess_case2_5_1(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    px: f32,
    py: f32,
    tx: f32,
    ty: f32,
    vx: f32,
    vy: f32,
) -> f32 {
    let out1 = py
        / ((a11 * px + a12 * py + tx - vx).powi(2) + (a21 * px + a22 * py + ty - vy).powi(2))
            .sqrt()
        + py * (-a21 * px - a22 * py - ty + vy) * (a21 * px + a22 * py + ty - vy)
            / ((a11 * px + a12 * py + tx - vx).powi(2) + (a21 * px + a22 * py + ty - vy).powi(2))
                .powf(3_f32 / 2.0);
    out1
}

fn hess_case2_5_2(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    px: f32,
    py: f32,
    tx: f32,
    ty: f32,
    vx: f32,
    vy: f32,
) -> f32 {
    let out1 = -px * py * (a11 * px + a12 * py + tx - vx) * (a21 * px + a22 * py + ty - vy)
        / ((a11 * px + a12 * py + tx - vx).powi(2) + (a21 * px + a22 * py + ty - vy).powi(2))
            .powf(3_f32 / 2.0);
    out1
}

fn hess_case2_5_3(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    px: f32,
    py: f32,
    tx: f32,
    ty: f32,
    vx: f32,
    vy: f32,
) -> f32 {
    let out1 = -py.powi(2) * (a11 * px + a12 * py + tx - vx) * (a21 * px + a22 * py + ty - vy)
        / ((a11 * px + a12 * py + tx - vx).powi(2) + (a21 * px + a22 * py + ty - vy).powi(2))
            .powf(3_f32 / 2.0);
    out1
}

fn hess_case2_5_4(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    px: f32,
    py: f32,
    tx: f32,
    ty: f32,
    vx: f32,
    vy: f32,
) -> f32 {
    let out1 = px * py
        / ((a11 * px + a12 * py + tx - vx).powi(2) + (a21 * px + a22 * py + ty - vy).powi(2))
            .sqrt()
        - px * py * (a21 * px + a22 * py + ty - vy).powi(2)
            / ((a11 * px + a12 * py + tx - vx).powi(2) + (a21 * px + a22 * py + ty - vy).powi(2))
                .powf(3_f32 / 2.0);
    out1
}

fn hess_case2_5_5(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    px: f32,
    py: f32,
    tx: f32,
    ty: f32,
    vx: f32,
    vy: f32,
) -> f32 {
    let out1 = py.powi(2)
        / ((a11 * px + a12 * py + tx - vx).powi(2) + (a21 * px + a22 * py + ty - vy).powi(2))
            .sqrt()
        - py.powi(2) * (a21 * px + a22 * py + ty - vy).powi(2)
            / ((a11 * px + a12 * py + tx - vx).powi(2) + (a21 * px + a22 * py + ty - vy).powi(2))
                .powf(3_f32 / 2.0);
    out1
}
