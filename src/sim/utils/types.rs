use nalgebra::{ArrayStorage, U2, U6};

pub type Mat6x6 = nalgebra::Matrix<f32, U6, U6, ArrayStorage<f32, 6, 6>>;
pub type Mat2x6 = nalgebra::Matrix<f32, U2, U6, ArrayStorage<f32, 2, 6>>;
pub type Mat6x2 = nalgebra::Matrix<f32, U6, U2, ArrayStorage<f32, 6, 2>>;
pub type Vec6 = glm::TVec<f32, 6>;
