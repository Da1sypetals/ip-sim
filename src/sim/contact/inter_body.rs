use super::affine_contact::{ContactElem, ContactNode};
use crate::sim::{
    body::body::Body,
    utils::{
        affine_utils::interop::{AffineDof, InteropCol},
        misc::dof_index,
        types::Vec6,
    },
};
use faer::Col;

pub fn collect_interbody_contact_pairs(
    b1: &Body,
    b2: &Body,
    dof: &Col<f32>,
    pairs: &mut Vec<ContactElem>,
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
                    let pair = ContactElem {
                        p: ContactNode::Node {
                            p: glm::vec2(dof[ipx], dof[ipy]),
                            index: (ipx, ipy),
                        },
                        e: (
                            ContactNode::Node {
                                p: glm::vec2(dof[ie1x], dof[ie1y]),
                                index: (ie1x, ie1y),
                            },
                            ContactNode::Node {
                                p: glm::vec2(dof[ie2x], dof[ie2y]),
                                index: (ie2x, ie2y),
                            },
                        ),
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
                    let pair = ContactElem {
                        p: ContactNode::Node {
                            p: glm::vec2(dof[ipx], dof[ipy]),
                            index: (ipx, ipy),
                        },
                        e: (
                            ContactNode::Node {
                                p: glm::vec2(dof[ie1x], dof[ie1y]),
                                index: (ie1x, ie1y),
                            },
                            ContactNode::Node {
                                p: glm::vec2(dof[ie2x], dof[ie2y]),
                                index: (ie2x, ie2y),
                            },
                        ),
                    };

                    if pair.distance() < dhat {
                        pairs.push(pair);
                    }
                }
            }
        }
    }

    if let Body::Affine(ab1, off1) = b1 {
        if let Body::Affine(ab2, off2) = b2 {
            // extract q1 and q2
            let q1 = Vec6::from_dof(dof, *off1);
            let q2 = Vec6::from_dof(dof, *off2);

            // p1 - e2
            for ip in 0..ab1.nvert {
                let p = ab1.pos(&q1, ip);
                for ((iu, iv), e) in ab2.edges_enumerate(&q2) {
                    let t2 = q2.t_vec();
                    let a2 = q2.a_mat();
                    let index2 = (*off2, *off2 + 1, *off2 + 2, *off2 + 3, *off2 + 4, *off2 + 5);
                    let pair = ContactElem {
                        p: ContactNode::Affine {
                            t: q1.t_vec(),
                            a: q1.a_mat(),
                            x0: ab1.pos_init(ip),
                            index: (*off1, *off1 + 1, *off1 + 2, *off1 + 3, *off1 + 4, *off1 + 5),
                        },
                        e: (
                            ContactNode::Affine {
                                t: t2,
                                a: a2,
                                x0: ab2.pos_init(iu),
                                index: index2,
                            },
                            ContactNode::Affine {
                                t: t2,
                                a: a2,
                                x0: ab2.pos_init(iv),
                                index: index2,
                            },
                        ),
                    };

                    if pair.distance() < dhat {
                        pairs.push(pair);
                    }
                }
            }

            // p2 - e1
            for ip in 0..ab2.nvert {
                let p = ab2.pos(&q2, ip);
                for ((iu, iv), e) in ab1.edges_enumerate(&q1) {
                    let t1 = q1.t_vec();
                    let a1 = q1.a_mat();
                    let index1 = (*off1, *off1 + 1, *off1 + 2, *off1 + 3, *off1 + 4, *off1 + 5);
                    let pair = ContactElem {
                        p: ContactNode::Affine {
                            t: q2.t_vec(),
                            a: q2.a_mat(),
                            x0: ab2.pos_init(ip),
                            index: (*off2, *off2 + 1, *off2 + 2, *off2 + 3, *off2 + 4, *off2 + 5),
                        },
                        e: (
                            ContactNode::Affine {
                                t: t1,
                                a: a1,
                                x0: ab1.pos_init(iu),
                                index: index1,
                            },
                            ContactNode::Affine {
                                t: t1,
                                a: a1,
                                x0: ab1.pos_init(iv),
                                index: index1,
                            },
                        ),
                    };

                    if pair.distance() < dhat {
                        pairs.push(pair);
                    }
                }
            }
        }
    }
}
