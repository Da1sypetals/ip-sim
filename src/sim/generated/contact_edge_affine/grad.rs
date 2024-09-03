/*
 *                      Code generated with SymPy 1.13.2
 *
 *              See http://www.sympy.org/ for more information.
 *
 *                       This file is part of 'project'
 */

pub fn grad_edge_case1_0(
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
    let out1 = (a11 * u0x + a12 * u0y - px + tx)
        / ((a11 * u0x + a12 * u0y - px + tx).powi(2) + (a21 * u0x + a22 * u0y - py + ty).powi(2))
            .sqrt();
    out1
}

pub fn grad_edge_case1_1(
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
    let out1 = (a21 * u0x + a22 * u0y - py + ty)
        / ((a11 * u0x + a12 * u0y - px + tx).powi(2) + (a21 * u0x + a22 * u0y - py + ty).powi(2))
            .sqrt();
    out1
}

pub fn grad_edge_case1_2(
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
    let out1 = u0x * (a11 * u0x + a12 * u0y - px + tx)
        / ((a11 * u0x + a12 * u0y - px + tx).powi(2) + (a21 * u0x + a22 * u0y - py + ty).powi(2))
            .sqrt();
    out1
}

pub fn grad_edge_case1_3(
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
    let out1 = u0y * (a11 * u0x + a12 * u0y - px + tx)
        / ((a11 * u0x + a12 * u0y - px + tx).powi(2) + (a21 * u0x + a22 * u0y - py + ty).powi(2))
            .sqrt();
    out1
}

pub fn grad_edge_case1_4(
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
    let out1 = u0x * (a21 * u0x + a22 * u0y - py + ty)
        / ((a11 * u0x + a12 * u0y - px + tx).powi(2) + (a21 * u0x + a22 * u0y - py + ty).powi(2))
            .sqrt();
    out1
}

pub fn grad_edge_case1_5(
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
    let out1 = u0y * (a21 * u0x + a22 * u0y - py + ty)
        / ((a11 * u0x + a12 * u0y - px + tx).powi(2) + (a21 * u0x + a22 * u0y - py + ty).powi(2))
            .sqrt();
    out1
}

/*
 *                      Code generated with SymPy 1.13.2
 *
 *              See http://www.sympy.org/ for more information.
 *
 *                       This file is part of 'project'
 */

pub fn grad_edge_case2_0(
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
    let out1 = (a11 * v0x + a12 * v0y - px + tx)
        / ((a11 * v0x + a12 * v0y - px + tx).powi(2) + (a21 * v0x + a22 * v0y - py + ty).powi(2))
            .sqrt();
    out1
}

pub fn grad_edge_case2_1(
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
    let out1 = (a21 * v0x + a22 * v0y - py + ty)
        / ((a11 * v0x + a12 * v0y - px + tx).powi(2) + (a21 * v0x + a22 * v0y - py + ty).powi(2))
            .sqrt();
    out1
}

pub fn grad_edge_case2_2(
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
    let out1 = v0x * (a11 * v0x + a12 * v0y - px + tx)
        / ((a11 * v0x + a12 * v0y - px + tx).powi(2) + (a21 * v0x + a22 * v0y - py + ty).powi(2))
            .sqrt();
    out1
}

pub fn grad_edge_case2_3(
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
    let out1 = v0y * (a11 * v0x + a12 * v0y - px + tx)
        / ((a11 * v0x + a12 * v0y - px + tx).powi(2) + (a21 * v0x + a22 * v0y - py + ty).powi(2))
            .sqrt();
    out1
}

pub fn grad_edge_case2_4(
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
    let out1 = v0x * (a21 * v0x + a22 * v0y - py + ty)
        / ((a11 * v0x + a12 * v0y - px + tx).powi(2) + (a21 * v0x + a22 * v0y - py + ty).powi(2))
            .sqrt();
    out1
}

pub fn grad_edge_case2_5(
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
    let out1 = v0y * (a21 * v0x + a22 * v0y - py + ty)
        / ((a11 * v0x + a12 * v0y - px + tx).powi(2) + (a21 * v0x + a22 * v0y - py + ty).powi(2))
            .sqrt();
    out1
}

/*
 *                      Code generated with SymPy 1.13.2
 *
 *              See http://www.sympy.org/ for more information.
 *
 *                       This file is part of 'project'
 */

pub fn grad_edge_case3_0(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    u0x: f32,
    u0y: f32,
    v0x: f32,
    v0y: f32,
) -> f32 {
    let out1 = (-a21 * u0x + a21 * v0x - a22 * u0y + a22 * v0y)
        / ((a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y).powi(2)
            + (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y).powi(2))
        .sqrt();
    out1
}

pub fn grad_edge_case3_1(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    u0x: f32,
    u0y: f32,
    v0x: f32,
    v0y: f32,
) -> f32 {
    let out1 = (a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y)
        / ((a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y).powi(2)
            + (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y).powi(2))
        .sqrt();
    out1
}

pub fn grad_edge_case3_2(
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
        * ((-a11 * u0x + a11 * v0x - a12 * u0y + a12 * v0y) * (-a21 * u0x - a22 * u0y + py - ty)
            + (-a11 * u0x - a12 * u0y + px - tx) * (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y))
        * (a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y)
        / ((a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y).powi(2)
            + (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y).powi(2))
        .powf(3_f32 / 2.0)
        + (-u0x * (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y)
            + (-u0x + v0x) * (-a21 * u0x - a22 * u0y + py - ty))
            / ((a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y).powi(2)
                + (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y).powi(2))
            .sqrt();
    out1
}

pub fn grad_edge_case3_3(
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
        * (2f32 * u0y - 2f32 * v0y)
        * ((-a11 * u0x + a11 * v0x - a12 * u0y + a12 * v0y) * (-a21 * u0x - a22 * u0y + py - ty)
            + (-a11 * u0x - a12 * u0y + px - tx) * (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y))
        * (a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y)
        / ((a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y).powi(2)
            + (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y).powi(2))
        .powf(3_f32 / 2.0)
        + (-u0y * (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y)
            + (-u0y + v0y) * (-a21 * u0x - a22 * u0y + py - ty))
            / ((a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y).powi(2)
                + (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y).powi(2))
            .sqrt();
    out1
}

pub fn grad_edge_case3_4(
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
        * ((-a11 * u0x + a11 * v0x - a12 * u0y + a12 * v0y) * (-a21 * u0x - a22 * u0y + py - ty)
            + (-a11 * u0x - a12 * u0y + px - tx) * (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y))
        * (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y)
        / ((a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y).powi(2)
            + (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y).powi(2))
        .powf(3_f32 / 2.0)
        + (-u0x * (-a11 * u0x + a11 * v0x - a12 * u0y + a12 * v0y)
            + (u0x - v0x) * (-a11 * u0x - a12 * u0y + px - tx))
            / ((a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y).powi(2)
                + (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y).powi(2))
            .sqrt();
    out1
}

pub fn grad_edge_case3_5(
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
        * (2f32 * u0y - 2f32 * v0y)
        * ((-a11 * u0x + a11 * v0x - a12 * u0y + a12 * v0y) * (-a21 * u0x - a22 * u0y + py - ty)
            + (-a11 * u0x - a12 * u0y + px - tx) * (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y))
        * (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y)
        / ((a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y).powi(2)
            + (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y).powi(2))
        .powf(3_f32 / 2.0)
        + (-u0y * (-a11 * u0x + a11 * v0x - a12 * u0y + a12 * v0y)
            + (u0y - v0y) * (-a11 * u0x - a12 * u0y + px - tx))
            / ((a11 * u0x - a11 * v0x + a12 * u0y - a12 * v0y).powi(2)
                + (a21 * u0x - a21 * v0x + a22 * u0y - a22 * v0y).powi(2))
            .sqrt();
    out1
}
