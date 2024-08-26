use crate::sim::body::springsbody::SpringsBody;
use faer::Col;

use super::affine_contact::{ContactElem, ContactNode};

pub struct Boundary;
impl Boundary {
    // Left Right; Bottom Top
    pub fn left_bot() -> glm::Vec2 {
        return glm::vec2(-1., -1.);
    }

    pub fn left_top() -> glm::Vec2 {
        return glm::vec2(-1., 1.);
    }

    pub fn right_bot() -> glm::Vec2 {
        return glm::vec2(1., -1.);
    }

    pub fn right_top() -> glm::Vec2 {
        return glm::vec2(1., 1.);
    }
    pub fn points() -> Vec<glm::Vec2> {
        vec![
            Boundary::left_bot(),
            Boundary::left_top(),
            Boundary::right_bot(),
            Boundary::right_top(),
        ]
    }

    pub fn bot() -> (glm::Vec2, glm::Vec2) {
        return (Self::left_bot(), Self::right_bot());
    }

    pub fn top() -> (glm::Vec2, glm::Vec2) {
        return (Self::left_top(), Self::right_top());
    }

    pub fn left() -> (glm::Vec2, glm::Vec2) {
        return (Self::left_bot(), Self::left_top());
    }

    pub fn right() -> (glm::Vec2, glm::Vec2) {
        return (Self::right_bot(), Self::right_top());
    }

    pub fn edges() -> Vec<(glm::Vec2, glm::Vec2)> {
        vec![
            Boundary::bot(),
            Boundary::top(),
            Boundary::left(),
            Boundary::right(),
        ]
    }

    // to avoid some numerical issues
    pub fn edges_extended() -> Vec<(glm::Vec2, glm::Vec2)> {
        vec![
            (
                glm::vec2(Boundary::left_bot().x * 1.1, Boundary::left_bot().y),
                glm::vec2(Boundary::right_bot().x * 1.1, Boundary::right_bot().y),
            ),
            (
                glm::vec2(Boundary::left_top().x * 1.1, Boundary::left_top().y),
                glm::vec2(Boundary::right_top().x * 1.1, Boundary::right_top().y),
            ),
            (
                glm::vec2(Boundary::left_bot().x, Boundary::left_bot().y * 1.1),
                glm::vec2(Boundary::left_top().x, Boundary::left_top().y * 1.1),
            ),
            (
                glm::vec2(Boundary::right_bot().x, Boundary::right_bot().y * 1.1),
                glm::vec2(Boundary::right_top().x, Boundary::right_top().y * 1.1),
            ),
        ]
    }

    pub fn collect_contact_pairs_springbody_with_boundary(
        spbody: &SpringsBody,
        offset: usize,
        dof: &Col<f32>,
        dhat: f32,
        pairs: &mut Vec<ContactElem>,
    ) {
        for edge in Boundary::edges_extended() {
            for inode in 0..spbody.ndof / 2 {
                let (ix, iy) = (offset + inode * 2, offset + inode * 2 + 1);
                let point = glm::vec2(dof[ix], dof[iy]);
                let pair = ContactElem {
                    p: ContactNode::Node {
                        p: point,
                        index: (ix, iy),
                    },
                    e: (ContactNode::Static(edge.0), ContactNode::Static(edge.1)),
                };
                if pair.distance() < dhat {
                    pairs.push(pair);
                }
            }
        }
    }
}
