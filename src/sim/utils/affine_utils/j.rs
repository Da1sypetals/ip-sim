use crate::sim::utils::types::Mat2x6;

/// Assume positive contribution
pub fn j_per_triangle(x1: glm::Vec2, x2: glm::Vec2) -> Mat2x6 {
    let mut res = Mat2x6::zeros();

    for index in [(0, 0), (1, 1)] {
        res[index] = 1f32;
    }

    for index in [(0, 2), (1, 4)] {
        res[index] = (x1.x + x2.x) / 3f32;
    }

    for index in [(0, 3), (1, 5)] {
        res[index] = (x1.y + x2.y) / 3f32;
    }

    res * (x1.x * x2.y - x2.x * x1.y).abs() * 0.5f32
}
