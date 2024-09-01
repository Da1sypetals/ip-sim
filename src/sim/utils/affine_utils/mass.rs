use crate::sim::utils::types::Mat6x6;

/// Assume positive contribution, compute integration which adds to M.
pub fn mass_matrix_per_triangle(x1: glm::Vec2, x2: glm::Vec2) -> Mat6x6 {
    let mut res = Mat6x6::zeros();
    for index in [(0, 0), (1, 1)] {
        res[index] = 1f32;
    }
    for index in [(0, 2), (1, 4), (2, 0), (4, 1)] {
        res[index] = (x1.x + x2.x) / 3f32;
    }

    for index in [(0, 3), (1, 5), (3, 0), (5, 1)] {
        res[index] = (x1.y + x2.y) / 3f32;
    }

    for index in [(2, 2), (4, 4)] {
        // x^2
        let func = |v: glm::Vec2| v.x * v.x;
        res[index] = (func(x1) + func(x2) + func(x1 + x2)) / 12f32;
    }

    for index in [(3, 3), (5, 5)] {
        // x^2
        let func = |v: glm::Vec2| v.y * v.y;
        res[index] = (func(x1) + func(x2) + func(x1 + x2)) / 12f32;
    }

    for index in [(2, 3), (3, 2), (4, 5), (5, 4)] {
        // x^2
        let func = |v: glm::Vec2| v.x * v.y;
        res[index] = (func(x1) + func(x2) + func(x1 + x2)) / 12f32;
    }

    res * (x1.x * x2.y - x2.x * x1.y).abs() * 0.5f32
}
