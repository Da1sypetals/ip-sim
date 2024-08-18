// pub struct ContactPair {
//     edge: (glm::Vec2, glm::Vec2),
//     point: glm::Vec2,
// }

// pub struct ContactPairGrad((glm::Vec2, glm::Vec2), glm::Vec2);

// impl ContactPair {
//     pub fn distance(&self) -> f32 {
//         let (a, b) = self.edge;
//         let p = self.point;

//         let ab = b - a;
//         let ap = p - a;
//         let bp = p - b;

//         // Check if the point is closest to the start of the edge
//         if ab.dot(&ap) <= 0.0 {
//             return ap.norm();
//         }

//         // Check if the point is closest to the end of the edge
//         if ab.dot(&bp) >= 0.0 {
//             return bp.norm();
//         }

//         // The point is closest to the edge itself
//         let ap_proj_on_ab = ap.dot(&ab) / ab.norm_squared();
//         let proj_point = a + ap_proj_on_ab * ab;
//         (p - proj_point).norm()
//     }

//     pub fn potential(&self, dhat: f32) -> f32 {
//         let d = self.distance();
//         return -(d - dhat) * (d - dhat) * ((d - dhat).ln());
//     }

//     pub fn grad(&self, dhat: f32) -> ContactPairGrad{

//     }
// }

// pub struct ContactIp {}

// impl ContactIp {}
