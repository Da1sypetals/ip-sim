// /*
//  *                      Code generated with SymPy 1.13.2
//  *
//  *              See http://www.sympy.org/ for more information.
//  *
//  *                       This file is part of 'project'
//  */
// /*
//  *
//  *   case1: point closer to `e1`
//  *   case2: point closer to `e2`
//  *   case3: point closer to edge itself
//  *
// */
// /*
// todo:

// pub fn d_grad_p_case1_affine
// pub fn d_grad_p_case2_affine
// pub fn d_grad_p_case3_affine

// pub fn d_grad_e1_case1_affine *
// pub fn d_grad_e1_case2_affine
// pub fn d_grad_e1_case3_affine

// pub fn d_grad_e2_case1_affine
// pub fn d_grad_e2_case2_affine
// pub fn d_grad_e2_case3_affine

// */
// pub fn d_grad_p_case1_affine(
//     a11: f64,
//     a12: f64,
//     a21: f64,
//     a22: f64,
//     px: f64,
//     py: f64,
//     tx: f64,
//     ty: f64,
//     x0: f64,
//     y0: f64,
// ) -> [f64; 6]

// /// when point is closer to e1
// pub fn d_grad_e1_case1_affine(
//     a11: f64,
//     a12: f64,
//     a21: f64,
//     a22: f64,
//     px: f64,
//     py: f64,
//     tx: f64,
//     ty: f64,
//     x1x: f64,
//     x1y: f64,
// ) -> [f64; 6] {
//     let out1 = [
//         (a11 * x1x + a12 * x1y - px + tx)
//             / ((a11 * x1x + a12 * x1y - px + tx).powi(2)
//                 + (a21 * x1x + a22 * x1y - py + ty).powi(2))
//             .sqrt(),
//         (a21 * x1x + a22 * x1y - py + ty)
//             / ((a11 * x1x + a12 * x1y - px + tx).powi(2)
//                 + (a21 * x1x + a22 * x1y - py + ty).powi(2))
//             .sqrt(),
//         x1x * (a11 * x1x + a12 * x1y - px + tx)
//             / ((a11 * x1x + a12 * x1y - px + tx).powi(2)
//                 + (a21 * x1x + a22 * x1y - py + ty).powi(2))
//             .sqrt(),
//         x1y * (a11 * x1x + a12 * x1y - px + tx)
//             / ((a11 * x1x + a12 * x1y - px + tx).powi(2)
//                 + (a21 * x1x + a22 * x1y - py + ty).powi(2))
//             .sqrt(),
//         x1x * (a21 * x1x + a22 * x1y - py + ty)
//             / ((a11 * x1x + a12 * x1y - px + tx).powi(2)
//                 + (a21 * x1x + a22 * x1y - py + ty).powi(2))
//             .sqrt(),
//         x1y * (a21 * x1x + a22 * x1y - py + ty)
//             / ((a11 * x1x + a12 * x1y - px + tx).powi(2)
//                 + (a21 * x1x + a22 * x1y - py + ty).powi(2))
//             .sqrt(),
//     ];
//     out1
// }
