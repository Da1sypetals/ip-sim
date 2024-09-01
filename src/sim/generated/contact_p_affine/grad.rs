/*
 *                      Code generated with SymPy 1.13.2
 *
 *              See http://www.sympy.org/ for more information.
 *
 *                       This file is part of 'project'
 */

pub fn grad_case1_0(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    px: f32,
    py: f32,
    tx: f32,
    ty: f32,
    ux: f32,
    uy: f32,
) -> f32 {
    let out1 = (a11 * px + a12 * py + tx - ux)
        / ((a11 * px + a12 * py + tx - ux).powi(2) + (a21 * px + a22 * py + ty - uy).powi(2))
            .sqrt();
    out1
}

pub fn grad_case1_1(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    px: f32,
    py: f32,
    tx: f32,
    ty: f32,
    ux: f32,
    uy: f32,
) -> f32 {
    let out1 = (a21 * px + a22 * py + ty - uy)
        / ((a11 * px + a12 * py + tx - ux).powi(2) + (a21 * px + a22 * py + ty - uy).powi(2))
            .sqrt();
    out1
}

pub fn grad_case1_2(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    px: f32,
    py: f32,
    tx: f32,
    ty: f32,
    ux: f32,
    uy: f32,
) -> f32 {
    let out1 = px * (a11 * px + a12 * py + tx - ux)
        / ((a11 * px + a12 * py + tx - ux).powi(2) + (a21 * px + a22 * py + ty - uy).powi(2))
            .sqrt();
    out1
}

pub fn grad_case1_3(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    px: f32,
    py: f32,
    tx: f32,
    ty: f32,
    ux: f32,
    uy: f32,
) -> f32 {
    let out1 = py * (a11 * px + a12 * py + tx - ux)
        / ((a11 * px + a12 * py + tx - ux).powi(2) + (a21 * px + a22 * py + ty - uy).powi(2))
            .sqrt();
    out1
}

pub fn grad_case1_4(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    px: f32,
    py: f32,
    tx: f32,
    ty: f32,
    ux: f32,
    uy: f32,
) -> f32 {
    let out1 = px * (a21 * px + a22 * py + ty - uy)
        / ((a11 * px + a12 * py + tx - ux).powi(2) + (a21 * px + a22 * py + ty - uy).powi(2))
            .sqrt();
    out1
}

pub fn grad_case1_5(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    px: f32,
    py: f32,
    tx: f32,
    ty: f32,
    ux: f32,
    uy: f32,
) -> f32 {
    let out1 = py * (a21 * px + a22 * py + ty - uy)
        / ((a11 * px + a12 * py + tx - ux).powi(2) + (a21 * px + a22 * py + ty - uy).powi(2))
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

pub fn grad_case2_0(
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
    let out1 = (a11 * px + a12 * py + tx - vx)
        / ((a11 * px + a12 * py + tx - vx).powi(2) + (a21 * px + a22 * py + ty - vy).powi(2))
            .sqrt();
    out1
}

pub fn grad_case2_1(
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
    let out1 = (a21 * px + a22 * py + ty - vy)
        / ((a11 * px + a12 * py + tx - vx).powi(2) + (a21 * px + a22 * py + ty - vy).powi(2))
            .sqrt();
    out1
}

pub fn grad_case2_2(
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
    let out1 = px * (a11 * px + a12 * py + tx - vx)
        / ((a11 * px + a12 * py + tx - vx).powi(2) + (a21 * px + a22 * py + ty - vy).powi(2))
            .sqrt();
    out1
}

pub fn grad_case2_3(
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
    let out1 = py * (a11 * px + a12 * py + tx - vx)
        / ((a11 * px + a12 * py + tx - vx).powi(2) + (a21 * px + a22 * py + ty - vy).powi(2))
            .sqrt();
    out1
}

pub fn grad_case2_4(
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
    let out1 = px * (a21 * px + a22 * py + ty - vy)
        / ((a11 * px + a12 * py + tx - vx).powi(2) + (a21 * px + a22 * py + ty - vy).powi(2))
            .sqrt();
    out1
}

pub fn grad_case2_5(
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
    let out1 = py * (a21 * px + a22 * py + ty - vy)
        / ((a11 * px + a12 * py + tx - vx).powi(2) + (a21 * px + a22 * py + ty - vy).powi(2))
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

pub fn grad_case3_0(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    px: f32,
    py: f32,
    tx: f32,
    ty: f32,
    ux: f32,
    uy: f32,
    vx: f32,
    vy: f32,
) -> f32 {
    let out1 = (uy - vy)
        * (a11 * px * uy - a11 * px * vy + a12 * py * uy - a12 * py * vy - a21 * px * ux
            + a21 * px * vx
            - a22 * py * ux
            + a22 * py * vx
            + tx * uy
            - tx * vy
            - ty * ux
            + ty * vx
            + ux * vy
            - uy * vx)
            .signum()
        / ((ux - vx).powi(2) + (uy - vy).powi(2)).sqrt();
    out1
}

pub fn grad_case3_1(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    px: f32,
    py: f32,
    tx: f32,
    ty: f32,
    ux: f32,
    uy: f32,
    vx: f32,
    vy: f32,
) -> f32 {
    let out1 = (-ux + vx)
        * (a11 * px * uy - a11 * px * vy + a12 * py * uy - a12 * py * vy - a21 * px * ux
            + a21 * px * vx
            - a22 * py * ux
            + a22 * py * vx
            + tx * uy
            - tx * vy
            - ty * ux
            + ty * vx
            + ux * vy
            - uy * vx)
            .signum()
        / ((ux - vx).powi(2) + (uy - vy).powi(2)).sqrt();
    out1
}

pub fn grad_case3_2(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    px: f32,
    py: f32,
    tx: f32,
    ty: f32,
    ux: f32,
    uy: f32,
    vx: f32,
    vy: f32,
) -> f32 {
    let out1 = (px * uy - px * vy)
        * (a11 * px * uy - a11 * px * vy + a12 * py * uy - a12 * py * vy - a21 * px * ux
            + a21 * px * vx
            - a22 * py * ux
            + a22 * py * vx
            + tx * uy
            - tx * vy
            - ty * ux
            + ty * vx
            + ux * vy
            - uy * vx)
            .signum()
        / ((ux - vx).powi(2) + (uy - vy).powi(2)).sqrt();
    out1
}

pub fn grad_case3_3(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    px: f32,
    py: f32,
    tx: f32,
    ty: f32,
    ux: f32,
    uy: f32,
    vx: f32,
    vy: f32,
) -> f32 {
    let out1 = (py * uy - py * vy)
        * (a11 * px * uy - a11 * px * vy + a12 * py * uy - a12 * py * vy - a21 * px * ux
            + a21 * px * vx
            - a22 * py * ux
            + a22 * py * vx
            + tx * uy
            - tx * vy
            - ty * ux
            + ty * vx
            + ux * vy
            - uy * vx)
            .signum()
        / ((ux - vx).powi(2) + (uy - vy).powi(2)).sqrt();
    out1
}

pub fn grad_case3_4(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    px: f32,
    py: f32,
    tx: f32,
    ty: f32,
    ux: f32,
    uy: f32,
    vx: f32,
    vy: f32,
) -> f32 {
    let out1 = (-px * ux + px * vx)
        * (a11 * px * uy - a11 * px * vy + a12 * py * uy - a12 * py * vy - a21 * px * ux
            + a21 * px * vx
            - a22 * py * ux
            + a22 * py * vx
            + tx * uy
            - tx * vy
            - ty * ux
            + ty * vx
            + ux * vy
            - uy * vx)
            .signum()
        / ((ux - vx).powi(2) + (uy - vy).powi(2)).sqrt();
    out1
}

pub fn grad_case3_5(
    a11: f32,
    a12: f32,
    a21: f32,
    a22: f32,
    px: f32,
    py: f32,
    tx: f32,
    ty: f32,
    ux: f32,
    uy: f32,
    vx: f32,
    vy: f32,
) -> f32 {
    let out1 = (-py * ux + py * vx)
        * (a11 * px * uy - a11 * px * vy + a12 * py * uy - a12 * py * vy - a21 * px * ux
            + a21 * px * vx
            - a22 * py * ux
            + a22 * py * vx
            + tx * uy
            - tx * vy
            - ty * ux
            + ty * vx
            + ux * vy
            - uy * vx)
            .signum()
        / ((ux - vx).powi(2) + (uy - vy).powi(2)).sqrt();
    out1
}
