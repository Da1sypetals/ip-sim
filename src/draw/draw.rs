use super::super::sim::body::body::Body;
use macroquad::prelude::*;

pub fn draw_point(p: &glm::Vec2) {
    let _p = (p + glm::vec2(1f32, 1f32)) / 2f32;
    let p_unnorm = glm::vec2(_p[0] * screen_width(), _p[1] * screen_height());
    // dbg!(&p_unnorm);
    draw_circle(p_unnorm[0], p_unnorm[1], 5.0, YELLOW);
}

pub fn draw_link(p1: &glm::Vec2, p2: &glm::Vec2) {
    let _p1 = (p1 + glm::vec2(1f32, 1f32)) / 2f32;
    let _p2 = (p2 + glm::vec2(1f32, 1f32)) / 2f32;
    let p1_unnorm = glm::vec2(_p1[0] * screen_width(), _p1[1] * screen_height());
    let p2_unnorm = glm::vec2(_p2[0] * screen_width(), _p2[1] * screen_height());
    draw_line(
        p1_unnorm.x,
        p1_unnorm.y,
        p2_unnorm.x,
        p2_unnorm.y,
        1.,
        WHITE,
    );
}

pub trait Draw {
    fn draw(&self);
}

impl Draw for Body {
    fn draw(&self) {
        match self {
            Body::Affine(ab, _) => {
                for iv in 0..ab.nvert {
                    draw_point(&ab.pos(&ab.q, iv));
                }
                for iv in 0..ab.nvert {
                    let (i, j) = (iv, (iv + 1) % ab.nvert);
                    draw_link(&ab.pos(&ab.q, i), &ab.pos(&ab.q, j));
                }
            }
            Body::Soft() => todo!(),
            Body::Springs(spbody, _) => {
                let n = spbody.ndof / 2;
                for i in 0..n {
                    let x = glm::vec2(spbody.x[i * 2], spbody.x[i * 2 + 1]);
                    draw_point(&x);
                }
                for c in &spbody.constraints {
                    let x1 = glm::vec2(spbody.x[c.i1 * 2], spbody.x[c.i1 * 2 + 1]);
                    let x2 = glm::vec2(spbody.x[c.i2 * 2], spbody.x[c.i2 * 2 + 1]);
                    draw_link(&x1, &x2);
                }
            }
        }
    }
}
