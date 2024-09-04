// use faer::Col;

// use super::{body::body::Body, utils::hess::Hess};
// use sim::body::affinebody::AffineBody;
// use sim::body::body::GenericBody;
// use sim::body::springsbody::SpringsBody;

// use sim::body::affinebody::AffineBody;
// use sim::body::body::GenericBody;
// use sim::body::springsbody::SpringsBody;

// use sim::body::affinebody::AffineBody;
// use sim::body::body::GenericBody;
// use sim::body::springsbody::SpringsBody;

// use sim::body::affinebody::AffineBody;
// use sim::body::body::GenericBody;
// use sim::body::springsbody::SpringsBody;
// use sim::body::springsbody::SpringsBody;

// pub struct ObjFrame {
//     pub bodies: Vec<Body>,
// }

// impl ObjFrame {
//     pub fn read(&mut self, dof: &Col<f32>) {
//         for body in &mut self.bodies {
//             match body {
//                 Body::Affine() => todo!(),
//                 Body::Soft() => todo!(),
//                 Body::Springs(body, offset) => {
//                     for i in 0..body.ndof {
//                         let idof = i + *offset;
//                         body.x[i] = dof[idof];
//                     }
//                 }
//             }
//         }
//     }

//     pub fn write(&self, dof: &mut Col<f32>) {
//         for body in &self.bodies {
//             match body {
//                 Body::Affine() => todo!(),
//                 Body::Soft() => todo!(),
//                 Body::Springs(body, offset) => {
//                     for i in 0..body.ndof {
//                         let idof = i + offset;
//                         dof[idof] = body.x[i];
//                     }
//                 }
//             }
//         }
//     }
// }
