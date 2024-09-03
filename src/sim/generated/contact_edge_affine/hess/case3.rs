/*
 *                      Code generated with SymPy 1.13.2
 *
 *              See http://www.sympy.org/ for more information.
 *
 *                       This file is part of 'project'
 */

use crate::sim::utils::types::Mat6x6;

pub fn hess_edge_case3(
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
    v0x: f32,
    v0y: f32,
) -> Mat6x6 {
    Mat6x6::new(
        hess_edge_case3_0_0(),
        hess_edge_case3_0_1(),
        hess_edge_case3_0_2(a11, a12, a21, a22, u0x, u0y, v0x, v0y),
        hess_edge_case3_0_3(a11, a12, a21, a22, u0x, u0y, v0x, v0y),
        hess_edge_case3_0_4(a11, a12, a21, a22, u0x, u0y, v0x, v0y),
        hess_edge_case3_0_5(a11, a12, a21, a22, u0x, u0y, v0x, v0y),
        hess_edge_case3_1_0(),
        hess_edge_case3_1_1(),
        hess_edge_case3_1_2(a11, a12, a21, a22, u0x, u0y, v0x, v0y),
        hess_edge_case3_1_3(a11, a12, a21, a22, u0x, u0y, v0x, v0y),
        hess_edge_case3_1_4(a11, a12, a21, a22, u0x, u0y, v0x, v0y),
        hess_edge_case3_1_5(a11, a12, a21, a22, u0x, u0y, v0x, v0y),
        hess_edge_case3_2_0(a11, a12, a21, a22, u0x, u0y, v0x, v0y),
        hess_edge_case3_2_1(a11, a12, a21, a22, u0x, u0y, v0x, v0y),
        hess_edge_case3_2_2(a11, a12, a21, a22, px, py, tx, ty, u0x, u0y, v0x, v0y),
        hess_edge_case3_2_3(a11, a12, a21, a22, px, py, tx, ty, u0x, u0y, v0x, v0y),
        hess_edge_case3_2_4(a11, a12, a21, a22, px, py, tx, ty, u0x, u0y, v0x, v0y),
        hess_edge_case3_2_5(a11, a12, a21, a22, px, py, tx, ty, u0x, u0y, v0x, v0y),
        hess_edge_case3_3_0(a11, a12, a21, a22, u0x, u0y, v0x, v0y),
        hess_edge_case3_3_1(a11, a12, a21, a22, u0x, u0y, v0x, v0y),
        hess_edge_case3_3_2(a11, a12, a21, a22, px, py, tx, ty, u0x, u0y, v0x, v0y),
        hess_edge_case3_3_3(a11, a12, a21, a22, px, py, tx, ty, u0x, u0y, v0x, v0y),
        hess_edge_case3_3_4(a11, a12, a21, a22, px, py, tx, ty, u0x, u0y, v0x, v0y),
        hess_edge_case3_3_5(a11, a12, a21, a22, px, py, tx, ty, u0x, u0y, v0x, v0y),
        hess_edge_case3_4_0(a11, a12, a21, a22, u0x, u0y, v0x, v0y),
        hess_edge_case3_4_1(a11, a12, a21, a22, u0x, u0y, v0x, v0y),
        hess_edge_case3_4_2(a11, a12, a21, a22, px, py, tx, ty, u0x, u0y, v0x, v0y),
        hess_edge_case3_4_3(a11, a12, a21, a22, px, py, tx, ty, u0x, u0y, v0x, v0y),
        hess_edge_case3_4_4(a11, a12, a21, a22, px, py, tx, ty, u0x, u0y, v0x, v0y),
        hess_edge_case3_4_5(a11, a12, a21, a22, px, py, tx, ty, u0x, u0y, v0x, v0y),
        hess_edge_case3_5_0(a11, a12, a21, a22, u0x, u0y, v0x, v0y),
        hess_edge_case3_5_1(a11, a12, a21, a22, u0x, u0y, v0x, v0y),
        hess_edge_case3_5_2(a11, a12, a21, a22, px, py, tx, ty, u0x, u0y, v0x, v0y),
        hess_edge_case3_5_3(a11, a12, a21, a22, px, py, tx, ty, u0x, u0y, v0x, v0y),
        hess_edge_case3_5_4(a11, a12, a21, a22, px, py, tx, ty, u0x, u0y, v0x, v0y),
        hess_edge_case3_5_5(a11, a12, a21, a22, px, py, tx, ty, u0x, u0y, v0x, v0y),
    )
}

pub fn hess_edge_case3_0_0() -> f32 {
    let out1 = 0.0;
    out1
}

pub fn hess_edge_case3_0_1() -> f32 {
    let out1 = 0.0;
    out1
}

pub fn hess_edge_case3_0_2(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    u0x: f32,
    u0y: f32,
    v0x: f32,
    v0y: f32,
) -> f32 {
    let out1 = -1_f32 / 2.0
        * (2f32 * u0x - 2f32 * v0x)
        * (a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y)
        * (-a21 * u0x + a21 * v0x - a22 * u0y + a22 * v0y)
        / ((a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y).powi(2)
            + (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y).powi(2))
        .powf(3_f32 / 2.0);
    out1
}

pub fn hess_edge_case3_0_3(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    u0x: f32,
    u0y: f32,
    v0x: f32,
    v0y: f32,
) -> f32 {
    let out1 = -1_f32 / 2.0
        * (2f32 * u0y - 2f32 * v0y)
        * (a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y)
        * (-a21 * u0x + a21 * v0x - a22 * u0y + a22 * v0y)
        / ((a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y).powi(2)
            + (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y).powi(2))
        .powf(3_f32 / 2.0);
    out1
}

pub fn hess_edge_case3_0_4(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    u0x: f32,
    u0y: f32,
    v0x: f32,
    v0y: f32,
) -> f32 {
    let out1 = (-u0x + v0x)
        / ((a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y).powi(2)
            + (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y).powi(2))
        .sqrt()
        - 1_f32 / 2.0
            * (2f32 * u0x - 2f32 * v0x)
            * (-a21 * u0x + a21 * v0x - a22 * u0y + a22 * v0y)
            * (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y)
            / ((a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y).powi(2)
                + (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y).powi(2))
            .powf(3_f32 / 2.0);
    out1
}

pub fn hess_edge_case3_0_5(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    u0x: f32,
    u0y: f32,
    v0x: f32,
    v0y: f32,
) -> f32 {
    let out1 = (-u0y + v0y)
        / ((a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y).powi(2)
            + (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y).powi(2))
        .sqrt()
        - 1_f32 / 2.0
            * (2f32 * u0y - 2f32 * v0y)
            * (-a21 * u0x + a21 * v0x - a22 * u0y + a22 * v0y)
            * (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y)
            / ((a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y).powi(2)
                + (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y).powi(2))
            .powf(3_f32 / 2.0);
    out1
}

pub fn hess_edge_case3_1_0() -> f32 {
    let out1 = 0.0;
    out1
}

pub fn hess_edge_case3_1_1() -> f32 {
    let out1 = 0.0;
    out1
}

pub fn hess_edge_case3_1_2(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    u0x: f32,
    u0y: f32,
    v0x: f32,
    v0y: f32,
) -> f32 {
    let out1 = (u0x - v0x)
        / ((a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y).powi(2)
            + (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y).powi(2))
        .sqrt()
        - 1_f32 / 2.0
            * (2f32 * u0x - 2f32 * v0x)
            * (a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y).powi(2)
            / ((a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y).powi(2)
                + (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y).powi(2))
            .powf(3_f32 / 2.0);
    out1
}

pub fn hess_edge_case3_1_3(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    u0x: f32,
    u0y: f32,
    v0x: f32,
    v0y: f32,
) -> f32 {
    let out1 = (u0y - v0y)
        / ((a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y).powi(2)
            + (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y).powi(2))
        .sqrt()
        - 1_f32 / 2.0
            * (2f32 * u0y - 2f32 * v0y)
            * (a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y).powi(2)
            / ((a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y).powi(2)
                + (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y).powi(2))
            .powf(3_f32 / 2.0);
    out1
}

pub fn hess_edge_case3_1_4(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    u0x: f32,
    u0y: f32,
    v0x: f32,
    v0y: f32,
) -> f32 {
    let out1 = -1_f32 / 2.0
        * (2f32 * u0x - 2f32 * v0x)
        * (a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y)
        * (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y)
        / ((a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y).powi(2)
            + (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y).powi(2))
        .powf(3_f32 / 2.0);
    out1
}

pub fn hess_edge_case3_1_5(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    u0x: f32,
    u0y: f32,
    v0x: f32,
    v0y: f32,
) -> f32 {
    let out1 = -1_f32 / 2.0
        * (2f32 * u0y - 2f32 * v0y)
        * (a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y)
        * (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y)
        / ((a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y).powi(2)
            + (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y).powi(2))
        .powf(3_f32 / 2.0);
    out1
}

pub fn hess_edge_case3_2_0(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    u0x: f32,
    u0y: f32,
    v0x: f32,
    v0y: f32,
) -> f32 {
    let out1 = -1_f32 / 2.0
        * (2f32 * u0x - 2f32 * v0x)
        * (a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y)
        * (-a21 * u0x + a21 * v0x - a22 * u0y + a22 * v0y)
        / ((a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y).powi(2)
            + (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y).powi(2))
        .powf(3_f32 / 2.0);
    out1
}

pub fn hess_edge_case3_2_1(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    u0x: f32,
    u0y: f32,
    v0x: f32,
    v0y: f32,
) -> f32 {
    let out1 = (u0x - v0x)
        / ((a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y).powi(2)
            + (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y).powi(2))
        .sqrt()
        - 1_f32 / 2.0
            * (2f32 * u0x - 2f32 * v0x)
            * (a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y).powi(2)
            / ((a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y).powi(2)
                + (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y).powi(2))
            .powf(3_f32 / 2.0);
    out1
}

pub fn hess_edge_case3_2_2(
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
    v0x: f32,
    v0y: f32,
) -> f32 {
    let out1 = -1_f32 / 2.0
        * (u0x - v0x)
        * (2f32 * u0x - 2f32 * v0x)
        * ((-a11 * u0x + a11 * v0x - a12 * u0y + a12 * v0y) * (-a21 * u0x - a22 * u0y + py - ty)
            + (-a11 * u0x - a12 * u0y + px - tx) * (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y))
        / ((a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y).powi(2)
            + (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y).powi(2))
        .powf(3_f32 / 2.0)
        + (3_f32 / 4.0)
            * (2f32 * u0x - 2f32 * v0x).powi(2)
            * ((-a11 * u0x + a11 * v0x - a12 * u0y + a12 * v0y)
                * (-a21 * u0x - a22 * u0y + py - ty)
                + (-a11 * u0x - a12 * u0y + px - tx)
                    * (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y))
            * (a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y).powi(2)
            / ((a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y).powi(2)
                + (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y).powi(2))
            .powf(5_f32 / 2.0)
        - (2f32 * u0x - 2f32 * v0x)
            * (-u0x * (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y)
                + (-u0x + v0x) * (-a21 * u0x - a22 * u0y + py - ty))
            * (a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y)
            / ((a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y).powi(2)
                + (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y).powi(2))
            .powf(3_f32 / 2.0);
    out1
}

pub fn hess_edge_case3_2_3(
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
    v0x: f32,
    v0y: f32,
) -> f32 {
    let out1 = -1_f32 / 2.0
        * (u0x - v0x)
        * (2f32 * u0y - 2f32 * v0y)
        * ((-a11 * u0x + a11 * v0x - a12 * u0y + a12 * v0y) * (-a21 * u0x - a22 * u0y + py - ty)
            + (-a11 * u0x - a12 * u0y + px - tx) * (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y))
        / ((a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y).powi(2)
            + (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y).powi(2))
        .powf(3_f32 / 2.0)
        + (3_f32 / 4.0)
            * (2f32 * u0x - 2f32 * v0x)
            * (2f32 * u0y - 2f32 * v0y)
            * ((-a11 * u0x + a11 * v0x - a12 * u0y + a12 * v0y)
                * (-a21 * u0x - a22 * u0y + py - ty)
                + (-a11 * u0x - a12 * u0y + px - tx)
                    * (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y))
            * (a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y).powi(2)
            / ((a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y).powi(2)
                + (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y).powi(2))
            .powf(5_f32 / 2.0)
        - 1_f32 / 2.0
            * (2f32 * u0x - 2f32 * v0x)
            * (-u0y * (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y)
                + (-u0y + v0y) * (-a21 * u0x - a22 * u0y + py - ty))
            * (a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y)
            / ((a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y).powi(2)
                + (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y).powi(2))
            .powf(3_f32 / 2.0)
        - 1_f32 / 2.0
            * (2f32 * u0y - 2f32 * v0y)
            * (-u0x * (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y)
                + (-u0x + v0x) * (-a21 * u0x - a22 * u0y + py - ty))
            * (a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y)
            / ((a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y).powi(2)
                + (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y).powi(2))
            .powf(3_f32 / 2.0);
    out1
}

pub fn hess_edge_case3_2_4(
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
    v0x: f32,
    v0y: f32,
) -> f32 {
    let out1 = (3_f32 / 4.0)
        * (2f32 * u0x - 2f32 * v0x).powi(2)
        * ((-a11 * u0x + a11 * v0x - a12 * u0y + a12 * v0y) * (-a21 * u0x - a22 * u0y + py - ty)
            + (-a11 * u0x - a12 * u0y + px - tx) * (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y))
        * (a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y)
        * (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y)
        / ((a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y).powi(2)
            + (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y).powi(2))
        .powf(5_f32 / 2.0)
        - 1_f32 / 2.0
            * (2f32 * u0x - 2f32 * v0x)
            * (-u0x * (-a11 * u0x + a11 * v0x - a12 * u0y + a12 * v0y)
                + (u0x - v0x) * (-a11 * u0x - a12 * u0y + px - tx))
            * (a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y)
            / ((a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y).powi(2)
                + (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y).powi(2))
            .powf(3_f32 / 2.0)
        - 1_f32 / 2.0
            * (2f32 * u0x - 2f32 * v0x)
            * (-u0x * (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y)
                + (-u0x + v0x) * (-a21 * u0x - a22 * u0y + py - ty))
            * (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y)
            / ((a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y).powi(2)
                + (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y).powi(2))
            .powf(3_f32 / 2.0)
        + (-u0x * (-u0x + v0x) - u0x * (u0x - v0x))
            / ((a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y).powi(2)
                + (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y).powi(2))
            .sqrt();
    out1
}

pub fn hess_edge_case3_2_5(
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
    v0x: f32,
    v0y: f32,
) -> f32 {
    let out1 = (3_f32 / 4.0)
        * (2f32 * u0x - 2f32 * v0x)
        * (2f32 * u0y - 2f32 * v0y)
        * ((-a11 * u0x + a11 * v0x - a12 * u0y + a12 * v0y) * (-a21 * u0x - a22 * u0y + py - ty)
            + (-a11 * u0x - a12 * u0y + px - tx) * (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y))
        * (a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y)
        * (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y)
        / ((a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y).powi(2)
            + (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y).powi(2))
        .powf(5_f32 / 2.0)
        - 1_f32 / 2.0
            * (2f32 * u0x - 2f32 * v0x)
            * (-u0y * (-a11 * u0x + a11 * v0x - a12 * u0y + a12 * v0y)
                + (u0y - v0y) * (-a11 * u0x - a12 * u0y + px - tx))
            * (a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y)
            / ((a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y).powi(2)
                + (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y).powi(2))
            .powf(3_f32 / 2.0)
        - 1_f32 / 2.0
            * (2f32 * u0y - 2f32 * v0y)
            * (-u0x * (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y)
                + (-u0x + v0x) * (-a21 * u0x - a22 * u0y + py - ty))
            * (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y)
            / ((a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y).powi(2)
                + (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y).powi(2))
            .powf(3_f32 / 2.0)
        + (-u0x * (u0y - v0y) - u0y * (-u0x + v0x))
            / ((a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y).powi(2)
                + (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y).powi(2))
            .sqrt();
    out1
}

pub fn hess_edge_case3_3_0(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    u0x: f32,
    u0y: f32,
    v0x: f32,
    v0y: f32,
) -> f32 {
    let out1 = -1_f32 / 2.0
        * (2f32 * u0y - 2f32 * v0y)
        * (a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y)
        * (-a21 * u0x + a21 * v0x - a22 * u0y + a22 * v0y)
        / ((a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y).powi(2)
            + (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y).powi(2))
        .powf(3_f32 / 2.0);
    out1
}

pub fn hess_edge_case3_3_1(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    u0x: f32,
    u0y: f32,
    v0x: f32,
    v0y: f32,
) -> f32 {
    let out1 = (u0y - v0y)
        / ((a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y).powi(2)
            + (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y).powi(2))
        .sqrt()
        - 1_f32 / 2.0
            * (2f32 * u0y - 2f32 * v0y)
            * (a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y).powi(2)
            / ((a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y).powi(2)
                + (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y).powi(2))
            .powf(3_f32 / 2.0);
    out1
}

pub fn hess_edge_case3_3_2(
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
    v0x: f32,
    v0y: f32,
) -> f32 {
    let out1 = -1_f32 / 2.0
        * (2f32 * u0x - 2f32 * v0x)
        * (u0y - v0y)
        * ((-a11 * u0x + a11 * v0x - a12 * u0y + a12 * v0y) * (-a21 * u0x - a22 * u0y + py - ty)
            + (-a11 * u0x - a12 * u0y + px - tx) * (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y))
        / ((a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y).powi(2)
            + (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y).powi(2))
        .powf(3_f32 / 2.0)
        + (3_f32 / 4.0)
            * (2f32 * u0x - 2f32 * v0x)
            * (2f32 * u0y - 2f32 * v0y)
            * ((-a11 * u0x + a11 * v0x - a12 * u0y + a12 * v0y)
                * (-a21 * u0x - a22 * u0y + py - ty)
                + (-a11 * u0x - a12 * u0y + px - tx)
                    * (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y))
            * (a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y).powi(2)
            / ((a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y).powi(2)
                + (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y).powi(2))
            .powf(5_f32 / 2.0)
        - 1_f32 / 2.0
            * (2f32 * u0x - 2f32 * v0x)
            * (-u0y * (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y)
                + (-u0y + v0y) * (-a21 * u0x - a22 * u0y + py - ty))
            * (a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y)
            / ((a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y).powi(2)
                + (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y).powi(2))
            .powf(3_f32 / 2.0)
        - 1_f32 / 2.0
            * (2f32 * u0y - 2f32 * v0y)
            * (-u0x * (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y)
                + (-u0x + v0x) * (-a21 * u0x - a22 * u0y + py - ty))
            * (a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y)
            / ((a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y).powi(2)
                + (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y).powi(2))
            .powf(3_f32 / 2.0);
    out1
}

pub fn hess_edge_case3_3_3(
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
    v0x: f32,
    v0y: f32,
) -> f32 {
    let out1 = -1_f32 / 2.0
        * (u0y - v0y)
        * (2f32 * u0y - 2f32 * v0y)
        * ((-a11 * u0x + a11 * v0x - a12 * u0y + a12 * v0y) * (-a21 * u0x - a22 * u0y + py - ty)
            + (-a11 * u0x - a12 * u0y + px - tx) * (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y))
        / ((a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y).powi(2)
            + (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y).powi(2))
        .powf(3_f32 / 2.0)
        + (3_f32 / 4.0)
            * (2f32 * u0y - 2f32 * v0y).powi(2)
            * ((-a11 * u0x + a11 * v0x - a12 * u0y + a12 * v0y)
                * (-a21 * u0x - a22 * u0y + py - ty)
                + (-a11 * u0x - a12 * u0y + px - tx)
                    * (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y))
            * (a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y).powi(2)
            / ((a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y).powi(2)
                + (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y).powi(2))
            .powf(5_f32 / 2.0)
        - (2f32 * u0y - 2f32 * v0y)
            * (-u0y * (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y)
                + (-u0y + v0y) * (-a21 * u0x - a22 * u0y + py - ty))
            * (a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y)
            / ((a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y).powi(2)
                + (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y).powi(2))
            .powf(3_f32 / 2.0);
    out1
}

pub fn hess_edge_case3_3_4(
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
    v0x: f32,
    v0y: f32,
) -> f32 {
    let out1 = (3_f32 / 4.0)
        * (2f32 * u0x - 2f32 * v0x)
        * (2f32 * u0y - 2f32 * v0y)
        * ((-a11 * u0x + a11 * v0x - a12 * u0y + a12 * v0y) * (-a21 * u0x - a22 * u0y + py - ty)
            + (-a11 * u0x - a12 * u0y + px - tx) * (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y))
        * (a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y)
        * (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y)
        / ((a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y).powi(2)
            + (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y).powi(2))
        .powf(5_f32 / 2.0)
        - 1_f32 / 2.0
            * (2f32 * u0x - 2f32 * v0x)
            * (-u0y * (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y)
                + (-u0y + v0y) * (-a21 * u0x - a22 * u0y + py - ty))
            * (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y)
            / ((a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y).powi(2)
                + (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y).powi(2))
            .powf(3_f32 / 2.0)
        - 1_f32 / 2.0
            * (2f32 * u0y - 2f32 * v0y)
            * (-u0x * (-a11 * u0x + a11 * v0x - a12 * u0y + a12 * v0y)
                + (u0x - v0x) * (-a11 * u0x - a12 * u0y + px - tx))
            * (a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y)
            / ((a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y).powi(2)
                + (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y).powi(2))
            .powf(3_f32 / 2.0)
        + (-u0x * (-u0y + v0y) - u0y * (u0x - v0x))
            / ((a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y).powi(2)
                + (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y).powi(2))
            .sqrt();
    out1
}

pub fn hess_edge_case3_3_5(
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
    v0x: f32,
    v0y: f32,
) -> f32 {
    let out1 = (3_f32 / 4.0)
        * (2f32 * u0y - 2f32 * v0y).powi(2)
        * ((-a11 * u0x + a11 * v0x - a12 * u0y + a12 * v0y) * (-a21 * u0x - a22 * u0y + py - ty)
            + (-a11 * u0x - a12 * u0y + px - tx) * (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y))
        * (a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y)
        * (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y)
        / ((a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y).powi(2)
            + (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y).powi(2))
        .powf(5_f32 / 2.0)
        - 1_f32 / 2.0
            * (2f32 * u0y - 2f32 * v0y)
            * (-u0y * (-a11 * u0x + a11 * v0x - a12 * u0y + a12 * v0y)
                + (u0y - v0y) * (-a11 * u0x - a12 * u0y + px - tx))
            * (a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y)
            / ((a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y).powi(2)
                + (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y).powi(2))
            .powf(3_f32 / 2.0)
        - 1_f32 / 2.0
            * (2f32 * u0y - 2f32 * v0y)
            * (-u0y * (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y)
                + (-u0y + v0y) * (-a21 * u0x - a22 * u0y + py - ty))
            * (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y)
            / ((a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y).powi(2)
                + (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y).powi(2))
            .powf(3_f32 / 2.0)
        + (-u0y * (-u0y + v0y) - u0y * (u0y - v0y))
            / ((a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y).powi(2)
                + (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y).powi(2))
            .sqrt();
    out1
}

pub fn hess_edge_case3_4_0(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    u0x: f32,
    u0y: f32,
    v0x: f32,
    v0y: f32,
) -> f32 {
    let out1 = (-u0x + v0x)
        / ((a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y).powi(2)
            + (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y).powi(2))
        .sqrt()
        - 1_f32 / 2.0
            * (2f32 * u0x - 2f32 * v0x)
            * (-a21 * u0x + a21 * v0x - a22 * u0y + a22 * v0y)
            * (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y)
            / ((a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y).powi(2)
                + (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y).powi(2))
            .powf(3_f32 / 2.0);
    out1
}

pub fn hess_edge_case3_4_1(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    u0x: f32,
    u0y: f32,
    v0x: f32,
    v0y: f32,
) -> f32 {
    let out1 = -1_f32 / 2.0
        * (2f32 * u0x - 2f32 * v0x)
        * (a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y)
        * (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y)
        / ((a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y).powi(2)
            + (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y).powi(2))
        .powf(3_f32 / 2.0);
    out1
}

pub fn hess_edge_case3_4_2(
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
    v0x: f32,
    v0y: f32,
) -> f32 {
    let out1 = (3_f32 / 4.0)
        * (2f32 * u0x - 2f32 * v0x).powi(2)
        * ((-a11 * u0x + a11 * v0x - a12 * u0y + a12 * v0y) * (-a21 * u0x - a22 * u0y + py - ty)
            + (-a11 * u0x - a12 * u0y + px - tx) * (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y))
        * (a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y)
        * (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y)
        / ((a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y).powi(2)
            + (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y).powi(2))
        .powf(5_f32 / 2.0)
        - 1_f32 / 2.0
            * (2f32 * u0x - 2f32 * v0x)
            * (-u0x * (-a11 * u0x + a11 * v0x - a12 * u0y + a12 * v0y)
                + (u0x - v0x) * (-a11 * u0x - a12 * u0y + px - tx))
            * (a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y)
            / ((a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y).powi(2)
                + (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y).powi(2))
            .powf(3_f32 / 2.0)
        - 1_f32 / 2.0
            * (2f32 * u0x - 2f32 * v0x)
            * (-u0x * (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y)
                + (-u0x + v0x) * (-a21 * u0x - a22 * u0y + py - ty))
            * (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y)
            / ((a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y).powi(2)
                + (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y).powi(2))
            .powf(3_f32 / 2.0)
        + (-u0x * (-u0x + v0x) - u0x * (u0x - v0x))
            / ((a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y).powi(2)
                + (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y).powi(2))
            .sqrt();
    out1
}

pub fn hess_edge_case3_4_3(
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
    v0x: f32,
    v0y: f32,
) -> f32 {
    let out1 = (3_f32 / 4.0)
        * (2f32 * u0x - 2f32 * v0x)
        * (2f32 * u0y - 2f32 * v0y)
        * ((-a11 * u0x + a11 * v0x - a12 * u0y + a12 * v0y) * (-a21 * u0x - a22 * u0y + py - ty)
            + (-a11 * u0x - a12 * u0y + px - tx) * (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y))
        * (a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y)
        * (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y)
        / ((a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y).powi(2)
            + (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y).powi(2))
        .powf(5_f32 / 2.0)
        - 1_f32 / 2.0
            * (2f32 * u0x - 2f32 * v0x)
            * (-u0y * (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y)
                + (-u0y + v0y) * (-a21 * u0x - a22 * u0y + py - ty))
            * (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y)
            / ((a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y).powi(2)
                + (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y).powi(2))
            .powf(3_f32 / 2.0)
        - 1_f32 / 2.0
            * (2f32 * u0y - 2f32 * v0y)
            * (-u0x * (-a11 * u0x + a11 * v0x - a12 * u0y + a12 * v0y)
                + (u0x - v0x) * (-a11 * u0x - a12 * u0y + px - tx))
            * (a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y)
            / ((a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y).powi(2)
                + (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y).powi(2))
            .powf(3_f32 / 2.0)
        + (-u0x * (-u0y + v0y) - u0y * (u0x - v0x))
            / ((a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y).powi(2)
                + (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y).powi(2))
            .sqrt();
    out1
}

pub fn hess_edge_case3_4_4(
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
    v0x: f32,
    v0y: f32,
) -> f32 {
    let out1 = -1_f32 / 2.0
        * (u0x - v0x)
        * (2f32 * u0x - 2f32 * v0x)
        * ((-a11 * u0x + a11 * v0x - a12 * u0y + a12 * v0y) * (-a21 * u0x - a22 * u0y + py - ty)
            + (-a11 * u0x - a12 * u0y + px - tx) * (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y))
        / ((a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y).powi(2)
            + (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y).powi(2))
        .powf(3_f32 / 2.0)
        + (3_f32 / 4.0)
            * (2f32 * u0x - 2f32 * v0x).powi(2)
            * ((-a11 * u0x + a11 * v0x - a12 * u0y + a12 * v0y)
                * (-a21 * u0x - a22 * u0y + py - ty)
                + (-a11 * u0x - a12 * u0y + px - tx)
                    * (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y))
            * (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y).powi(2)
            / ((a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y).powi(2)
                + (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y).powi(2))
            .powf(5_f32 / 2.0)
        - (2f32 * u0x - 2f32 * v0x)
            * (-u0x * (-a11 * u0x + a11 * v0x - a12 * u0y + a12 * v0y)
                + (u0x - v0x) * (-a11 * u0x - a12 * u0y + px - tx))
            * (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y)
            / ((a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y).powi(2)
                + (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y).powi(2))
            .powf(3_f32 / 2.0);
    out1
}

pub fn hess_edge_case3_4_5(
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
    v0x: f32,
    v0y: f32,
) -> f32 {
    let out1 = -1_f32 / 2.0
        * (u0x - v0x)
        * (2f32 * u0y - 2f32 * v0y)
        * ((-a11 * u0x + a11 * v0x - a12 * u0y + a12 * v0y) * (-a21 * u0x - a22 * u0y + py - ty)
            + (-a11 * u0x - a12 * u0y + px - tx) * (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y))
        / ((a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y).powi(2)
            + (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y).powi(2))
        .powf(3_f32 / 2.0)
        + (3_f32 / 4.0)
            * (2f32 * u0x - 2f32 * v0x)
            * (2f32 * u0y - 2f32 * v0y)
            * ((-a11 * u0x + a11 * v0x - a12 * u0y + a12 * v0y)
                * (-a21 * u0x - a22 * u0y + py - ty)
                + (-a11 * u0x - a12 * u0y + px - tx)
                    * (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y))
            * (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y).powi(2)
            / ((a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y).powi(2)
                + (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y).powi(2))
            .powf(5_f32 / 2.0)
        - 1_f32 / 2.0
            * (2f32 * u0x - 2f32 * v0x)
            * (-u0y * (-a11 * u0x + a11 * v0x - a12 * u0y + a12 * v0y)
                + (u0y - v0y) * (-a11 * u0x - a12 * u0y + px - tx))
            * (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y)
            / ((a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y).powi(2)
                + (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y).powi(2))
            .powf(3_f32 / 2.0)
        - 1_f32 / 2.0
            * (2f32 * u0y - 2f32 * v0y)
            * (-u0x * (-a11 * u0x + a11 * v0x - a12 * u0y + a12 * v0y)
                + (u0x - v0x) * (-a11 * u0x - a12 * u0y + px - tx))
            * (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y)
            / ((a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y).powi(2)
                + (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y).powi(2))
            .powf(3_f32 / 2.0);
    out1
}

pub fn hess_edge_case3_5_0(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    u0x: f32,
    u0y: f32,
    v0x: f32,
    v0y: f32,
) -> f32 {
    let out1 = (-u0y + v0y)
        / ((a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y).powi(2)
            + (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y).powi(2))
        .sqrt()
        - 1_f32 / 2.0
            * (2f32 * u0y - 2f32 * v0y)
            * (-a21 * u0x + a21 * v0x - a22 * u0y + a22 * v0y)
            * (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y)
            / ((a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y).powi(2)
                + (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y).powi(2))
            .powf(3_f32 / 2.0);
    out1
}

pub fn hess_edge_case3_5_1(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    u0x: f32,
    u0y: f32,
    v0x: f32,
    v0y: f32,
) -> f32 {
    let out1 = -1_f32 / 2.0
        * (2f32 * u0y - 2f32 * v0y)
        * (a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y)
        * (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y)
        / ((a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y).powi(2)
            + (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y).powi(2))
        .powf(3_f32 / 2.0);
    out1
}

pub fn hess_edge_case3_5_2(
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
    v0x: f32,
    v0y: f32,
) -> f32 {
    let out1 = (3_f32 / 4.0)
        * (2f32 * u0x - 2f32 * v0x)
        * (2f32 * u0y - 2f32 * v0y)
        * ((-a11 * u0x + a11 * v0x - a12 * u0y + a12 * v0y) * (-a21 * u0x - a22 * u0y + py - ty)
            + (-a11 * u0x - a12 * u0y + px - tx) * (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y))
        * (a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y)
        * (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y)
        / ((a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y).powi(2)
            + (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y).powi(2))
        .powf(5_f32 / 2.0)
        - 1_f32 / 2.0
            * (2f32 * u0x - 2f32 * v0x)
            * (-u0y * (-a11 * u0x + a11 * v0x - a12 * u0y + a12 * v0y)
                + (u0y - v0y) * (-a11 * u0x - a12 * u0y + px - tx))
            * (a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y)
            / ((a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y).powi(2)
                + (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y).powi(2))
            .powf(3_f32 / 2.0)
        - 1_f32 / 2.0
            * (2f32 * u0y - 2f32 * v0y)
            * (-u0x * (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y)
                + (-u0x + v0x) * (-a21 * u0x - a22 * u0y + py - ty))
            * (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y)
            / ((a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y).powi(2)
                + (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y).powi(2))
            .powf(3_f32 / 2.0)
        + (-u0x * (u0y - v0y) - u0y * (-u0x + v0x))
            / ((a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y).powi(2)
                + (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y).powi(2))
            .sqrt();
    out1
}

pub fn hess_edge_case3_5_3(
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
    v0x: f32,
    v0y: f32,
) -> f32 {
    let out1 = (3_f32 / 4.0)
        * (2f32 * u0y - 2f32 * v0y).powi(2)
        * ((-a11 * u0x + a11 * v0x - a12 * u0y + a12 * v0y) * (-a21 * u0x - a22 * u0y + py - ty)
            + (-a11 * u0x - a12 * u0y + px - tx) * (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y))
        * (a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y)
        * (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y)
        / ((a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y).powi(2)
            + (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y).powi(2))
        .powf(5_f32 / 2.0)
        - 1_f32 / 2.0
            * (2f32 * u0y - 2f32 * v0y)
            * (-u0y * (-a11 * u0x + a11 * v0x - a12 * u0y + a12 * v0y)
                + (u0y - v0y) * (-a11 * u0x - a12 * u0y + px - tx))
            * (a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y)
            / ((a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y).powi(2)
                + (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y).powi(2))
            .powf(3_f32 / 2.0)
        - 1_f32 / 2.0
            * (2f32 * u0y - 2f32 * v0y)
            * (-u0y * (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y)
                + (-u0y + v0y) * (-a21 * u0x - a22 * u0y + py - ty))
            * (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y)
            / ((a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y).powi(2)
                + (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y).powi(2))
            .powf(3_f32 / 2.0)
        + (-u0y * (-u0y + v0y) - u0y * (u0y - v0y))
            / ((a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y).powi(2)
                + (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y).powi(2))
            .sqrt();
    out1
}

pub fn hess_edge_case3_5_4(
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
    v0x: f32,
    v0y: f32,
) -> f32 {
    let out1 = -1_f32 / 2.0
        * (2f32 * u0x - 2f32 * v0x)
        * (u0y - v0y)
        * ((-a11 * u0x + a11 * v0x - a12 * u0y + a12 * v0y) * (-a21 * u0x - a22 * u0y + py - ty)
            + (-a11 * u0x - a12 * u0y + px - tx) * (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y))
        / ((a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y).powi(2)
            + (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y).powi(2))
        .powf(3_f32 / 2.0)
        + (3_f32 / 4.0)
            * (2f32 * u0x - 2f32 * v0x)
            * (2f32 * u0y - 2f32 * v0y)
            * ((-a11 * u0x + a11 * v0x - a12 * u0y + a12 * v0y)
                * (-a21 * u0x - a22 * u0y + py - ty)
                + (-a11 * u0x - a12 * u0y + px - tx)
                    * (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y))
            * (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y).powi(2)
            / ((a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y).powi(2)
                + (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y).powi(2))
            .powf(5_f32 / 2.0)
        - 1_f32 / 2.0
            * (2f32 * u0x - 2f32 * v0x)
            * (-u0y * (-a11 * u0x + a11 * v0x - a12 * u0y + a12 * v0y)
                + (u0y - v0y) * (-a11 * u0x - a12 * u0y + px - tx))
            * (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y)
            / ((a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y).powi(2)
                + (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y).powi(2))
            .powf(3_f32 / 2.0)
        - 1_f32 / 2.0
            * (2f32 * u0y - 2f32 * v0y)
            * (-u0x * (-a11 * u0x + a11 * v0x - a12 * u0y + a12 * v0y)
                + (u0x - v0x) * (-a11 * u0x - a12 * u0y + px - tx))
            * (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y)
            / ((a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y).powi(2)
                + (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y).powi(2))
            .powf(3_f32 / 2.0);
    out1
}

pub fn hess_edge_case3_5_5(
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
    v0x: f32,
    v0y: f32,
) -> f32 {
    let out1 = -1_f32 / 2.0
        * (u0y - v0y)
        * (2f32 * u0y - 2f32 * v0y)
        * ((-a11 * u0x + a11 * v0x - a12 * u0y + a12 * v0y) * (-a21 * u0x - a22 * u0y + py - ty)
            + (-a11 * u0x - a12 * u0y + px - tx) * (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y))
        / ((a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y).powi(2)
            + (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y).powi(2))
        .powf(3_f32 / 2.0)
        + (3_f32 / 4.0)
            * (2f32 * u0y - 2f32 * v0y).powi(2)
            * ((-a11 * u0x + a11 * v0x - a12 * u0y + a12 * v0y)
                * (-a21 * u0x - a22 * u0y + py - ty)
                + (-a11 * u0x - a12 * u0y + px - tx)
                    * (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y))
            * (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y).powi(2)
            / ((a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y).powi(2)
                + (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y).powi(2))
            .powf(5_f32 / 2.0)
        - (2f32 * u0y - 2f32 * v0y)
            * (-u0y * (-a11 * u0x + a11 * v0x - a12 * u0y + a12 * v0y)
                + (u0y - v0y) * (-a11 * u0x - a12 * u0y + px - tx))
            * (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y)
            / ((a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y).powi(2)
                + (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y).powi(2))
            .powf(3_f32 / 2.0);
    out1
}
