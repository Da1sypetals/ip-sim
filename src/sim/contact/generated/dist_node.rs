/*
// pub fn d_grad_p_node(p: glm::Vec2, e1: glm::Vec2, e2: glm::Vec2) {
//     let a = e1;
//     let b = e2;

//     let ab = b - a;
//     let ap = p - a;
//     let bp = p - b;

//     let mut g_edge_a = glm::vec2(0.0, 0.0);
//     let mut g_edge_b = glm::vec2(0.0, 0.0);
//     let mut g_point = glm::vec2(0.0, 0.0);

//     // Check if the point is closest to the start of the edge
//     if ab.dot(&ap) <= 0.0 {
//         let ap_norm = ap.norm();
//         g_point = ap / ap_norm;
//         g_edge_a = -ap / ap_norm;
//     }
//     // Check if the point is closest to the end of the edge
//     else if ab.dot(&bp) >= 0.0 {
//         let bp_norm = bp.norm();
//         g_point = bp / bp_norm;
//         g_edge_b = -bp / bp_norm;
//     }
//     // The point is closest to the edge itself
//     else {
//         let ab_norm = ab.norm();
//         let ab_norm_3 = ab_norm.powi(3);

//         let cross = ab.x * ap.y - ap.x * ab.y;

//         g_point = glm::vec2(
//             -ab.y * cross.signum() / ab_norm,
//             ab.x * cross.signum() / ab_norm,
//         );
//         g_edge_a = glm::vec2(
//             ab.x * cross.abs() / ab_norm_3 - bp.y * cross.signum() / ab_norm,
//             ab.y * cross.abs() / ab_norm_3 + bp.x * cross.signum() / ab_norm,
//         );
//         // todo: g_edge_b
//         g_edge_b = glm::vec2(
//             -ab.x * cross.abs() / ab_norm_3 + ap.y * cross.signum() / ab_norm,
//             -ab.y * cross.abs() / ab_norm_3 - ap.x * cross.signum() / ab_norm,
//         );
//     }
// }
 */

pub fn d_grad_p_node(p: glm::Vec2, e0: glm::Vec2, e1: glm::Vec2) -> glm::Vec2 {
    let a = e0;
    let b = e1;

    let ab = b - a;
    let ap = p - a;
    let bp = p - b;

    let mut g_point = glm::vec2(0.0, 0.0);

    // Check if the point is closest to the start of the edge
    if ab.dot(&ap) <= 0.0 {
        let ap_norm = ap.norm();
        g_point = ap / ap_norm;
    }
    // Check if the point is closest to the end of the edge
    else if ab.dot(&bp) >= 0.0 {
        let bp_norm = bp.norm();
        g_point = bp / bp_norm;
    }
    // The point is closest to the edge itself
    else {
        let ab_norm = ab.norm();
        // let ab_norm_3 = ab_norm.powi(3);

        let cross = ab.x * ap.y - ap.x * ab.y;

        g_point = glm::vec2(
            -ab.y * cross.signum() / ab_norm,
            ab.x * cross.signum() / ab_norm,
        );
    }
    g_point
}

pub fn d_grad_e0_node(p: glm::Vec2, e1: glm::Vec2, e2: glm::Vec2) -> glm::Vec2 {
    let a = e1;
    let b = e2;

    let ab = b - a;
    let ap = p - a;
    let bp = p - b;

    let mut g_edge_a = glm::vec2(0.0, 0.0);

    // Check if the point is closest to the start of the edge
    if ab.dot(&ap) <= 0.0 {
        let ap_norm = ap.norm();
        g_edge_a = -ap / ap_norm;
    }
    // Check if the point is closest to the end of the edge
    else if ab.dot(&bp) >= 0.0 {
        // let bp_norm = bp.norm();
    }
    // The point is closest to the edge itself
    else {
        let ab_norm = ab.norm();
        let ab_norm_3 = ab_norm.powi(3);

        let cross = ab.x * ap.y - ap.x * ab.y;

        g_edge_a = glm::vec2(
            ab.x * cross.abs() / ab_norm_3 - bp.y * cross.signum() / ab_norm,
            ab.y * cross.abs() / ab_norm_3 + bp.x * cross.signum() / ab_norm,
        );
    }
    g_edge_a
}

pub fn d_grad_e1_node(p: glm::Vec2, e1: glm::Vec2, e2: glm::Vec2) -> glm::Vec2 {
    let a = e1;
    let b = e2;

    let ab = b - a;
    let ap = p - a;
    let bp = p - b;

    let mut g_edge_b = glm::vec2(0.0, 0.0);

    // Check if the point is closest to the start of the edge
    if ab.dot(&ap) <= 0.0 {
        // let ap_norm = ap.norm();
    }
    // Check if the point is closest to the end of the edge
    else if ab.dot(&bp) >= 0.0 {
        let bp_norm = bp.norm();
        g_edge_b = -bp / bp_norm;
    }
    // The point is closest to the edge itself
    else {
        let ab_norm = ab.norm();
        let ab_norm_3 = ab_norm.powi(3);

        let cross = ab.x * ap.y - ap.x * ab.y;

        // todo: g_edge_b
        g_edge_b = glm::vec2(
            -ab.x * cross.abs() / ab_norm_3 + ap.y * cross.signum() / ab_norm,
            -ab.y * cross.abs() / ab_norm_3 - ap.x * cross.signum() / ab_norm,
        );
    }
    g_edge_b
}

/// when point is closer to e1
pub fn d_grad_e1_case1_affine(
    a11: f64,
    a12: f64,
    a21: f64,
    a22: f64,
    px: f64,
    py: f64,
    tx: f64,
    ty: f64,
    x1x: f64,
    x1y: f64,
) -> [f64; 6] {
    let out1 = [
        (a11 * x1x + a12 * x1y - px + tx)
            / ((a11 * x1x + a12 * x1y - px + tx).powi(2)
                + (a21 * x1x + a22 * x1y - py + ty).powi(2))
            .sqrt(),
        (a21 * x1x + a22 * x1y - py + ty)
            / ((a11 * x1x + a12 * x1y - px + tx).powi(2)
                + (a21 * x1x + a22 * x1y - py + ty).powi(2))
            .sqrt(),
        x1x * (a11 * x1x + a12 * x1y - px + tx)
            / ((a11 * x1x + a12 * x1y - px + tx).powi(2)
                + (a21 * x1x + a22 * x1y - py + ty).powi(2))
            .sqrt(),
        x1y * (a11 * x1x + a12 * x1y - px + tx)
            / ((a11 * x1x + a12 * x1y - px + tx).powi(2)
                + (a21 * x1x + a22 * x1y - py + ty).powi(2))
            .sqrt(),
        x1x * (a21 * x1x + a22 * x1y - py + ty)
            / ((a11 * x1x + a12 * x1y - px + tx).powi(2)
                + (a21 * x1x + a22 * x1y - py + ty).powi(2))
            .sqrt(),
        x1y * (a21 * x1x + a22 * x1y - py + ty)
            / ((a11 * x1x + a12 * x1y - px + tx).powi(2)
                + (a21 * x1x + a22 * x1y - py + ty).powi(2))
            .sqrt(),
    ];
    out1
}
