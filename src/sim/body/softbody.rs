// use super::body::Ip;
// use crate::sim::utils::hess::Hess;
// use faer::Col;

// pub struct SoftBody {
//     pub x: Col<f32>,
//     pub xprev: Col<f32>,
//     pub v: Col<f32>,
//     pub tris: Vec<(usize, usize, usize)>,

//     // rest pose
//     pub d0: Vec<glm::Mat2>,
//     // rest volume
//     pub v0: Vec<f32>,

//     // parameters
//     pub mu: f32,
//     pub lambda: f32,
// }

// impl SoftBody {
//     pub fn init(&mut self) {
//         // temporary impl
//         todo!()
//     }
//     pub fn pose_mat(&self, itri: usize) -> glm::Mat2 {
//         let (i0, i1, i2) = self.tris[itri];
//         let x10 = self.x_at(i0) - self.x_at(i1);
//         let x20 = self.x_at(i0) - self.x_at(i2);

//         glm::Mat2::from_columns(&[x10, x20])
//     }
//     pub fn x_at(&self, iv: usize) -> glm::Vec2 {
//         glm::vec2(self.x[iv * 2], self.x[iv * 2 + 1])
//     }
//     pub fn v_at(&self, iv: usize) -> glm::Vec2 {
//         glm::vec2(self.v[iv * 2], self.v[iv * 2 + 1])
//     }
// }

// pub struct SoftbodyIp;

// impl Ip for SoftbodyIp {
//     type BodyT = SoftBody;

//     fn prepare(&self, cfg: &Self::BodyT) {
//         todo!()
//     }

//     fn value(&self, cfg: &Self::BodyT) -> f32 {
//         let mut energy = 0f32;
//         for (itri, (i0, i1, i2)) in cfg.tris.iter().enumerate() {
//             let d = cfg.pose_mat(itri);
//             let d0 = cfg.d0[itri];
//             let F = d * d0.try_inverse().expect("Rest pose not invertible!");
//             let C = F.transpose() * F;
//             let I_c = C.trace();
//             let J = F.determinant();
//             let logJ = J.ln();
//             const dim: f32 = 2f32;

//             energy +=
//                 cfg.mu * 0.5f32 * (I_c - dim) - cfg.mu * logJ + cfg.lambda * 0.5f32 * logJ * logJ;
//         }
//         energy
//     }

//     fn grad(&self, cfg: &Self::BodyT, result: &mut Col<f32>) {
//         // 1st-piola
//         for (itri, (i0, i1, i2)) in cfg.tris.iter().enumerate() {
//             let d = cfg.pose_mat(itri);
//             let d0 = cfg.d0[itri];
//             let d0_inv = d0.try_inverse().expect("Rest pose not invertible!");
//             let F = d * d0_inv;
//             let Ft_inv = F.transpose().try_inverse().expect("F not invertible!");
//             let J = F.determinant();

//             // piola
//             let piola = cfg.mu * (F - Ft_inv) + cfg.lambda * J.ln() * Ft_inv;

//             // force(-grad)
//             // 0.5 for 2-dim
//             let grads = 0.5f32 / d0_inv.determinant() * piola * d0_inv.transpose();
//             let g0 = -grads.column(0) - grads.column(1);
//             let g1 = grads.column(0);
//             let g2 = grads.column(1);

//             {
//                 // 0
//                 result[i0 * 2] += g0.x;
//                 result[i0 * 2 + 1] += g0.y;

//                 // 1
//                 result[i1 * 2] += g1.x;
//                 result[i1 * 2 + 1] += g1.y;

//                 // 2
//                 result[i2 * 2] += g2.x;
//                 result[i2 * 2 + 1] += g2.y;
//             }
//         }
//     }

//     fn hess(&self, cfg: &Self::BodyT, result: &mut Hess) {
//         todo!()
//     }
// }
