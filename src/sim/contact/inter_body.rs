use super::contact::{ContactIndex, ContactPair};
use crate::sim::{body::body::Body, utils::misc::dof_index};
use faer::Col;

pub fn collect_interbody_contact_pairs(
    b1: &Body,
    b2: &Body,
    dof: &Col<f32>,
    pairs: &mut Vec<ContactPair>,
    dhat: f32,
) {
    if let Body::Springs(spb1, off1) = b1 {
        if let Body::Springs(spb2, off2) = b2 {
            // p1 collide with e2
            for ip in 0..spb1.ndof / 2 {
                for c in &spb2.constraints {
                    let (ipx, ipy) = dof_index(ip, *off1);
                    let (ie1x, ie1y) = dof_index(c.i1, *off2);
                    let (ie2x, ie2y) = dof_index(c.i2, *off2);
                    let pair = ContactPair {
                        point: glm::vec2(dof[ipx], dof[ipy]),
                        edge: (
                            glm::vec2(dof[ie1x], dof[ie1y]),
                            glm::vec2(dof[ie2x], dof[ie2y]),
                        ),
                        index: ContactIndex {
                            p: Some((ipx, ipy)),
                            e: Some(((ie1x, ie1y), (ie2x, ie2y))),
                        },
                    };

                    if pair.distance() < dhat {
                        pairs.push(pair);
                    }
                }
            }

            // p2 collide with e1
            for ip in 0..spb2.ndof / 2 {
                for c in &spb1.constraints {
                    let (ipx, ipy) = dof_index(ip, *off2);
                    let (ie1x, ie1y) = dof_index(c.i1, *off1);
                    let (ie2x, ie2y) = dof_index(c.i2, *off1);
                    let pair = ContactPair {
                        point: glm::vec2(dof[ipx], dof[ipy]),
                        edge: (
                            glm::vec2(dof[ie1x], dof[ie1y]),
                            glm::vec2(dof[ie2x], dof[ie2y]),
                        ),
                        index: ContactIndex {
                            p: Some((ipx, ipy)),
                            e: Some(((ie1x, ie1y), (ie2x, ie2y))),
                        },
                    };

                    if pair.distance() < dhat {
                        pairs.push(pair);
                    }
                }
            }
        }
    }
}
