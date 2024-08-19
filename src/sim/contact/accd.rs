use super::contact::{ContactIndex, ContactPair};
pub struct ContactPairDir {
    pub edge: (glm::Vec2, glm::Vec2),
    pub point: glm::Vec2,

    pub index: ContactIndex,
}

pub struct Accd {
    s: f32,
    t_c: f32,
}

impl Accd {
    pub fn new(s: f32) -> Self {
        Self { s, t_c: 1f32 }
    }

    /// See: https://ipc-sim.github.io/C-IPC/
    /// - currently `xi` is not supported
    pub fn toi(
        &self,
        pair: &ContactPair,   // x
        dir: &ContactPairDir, // p
    ) -> f32 {
        let pbar = (dir.point + dir.edge.0 + dir.edge.1) / 3f32;

        let mut x = pair.clone();
        let p = ContactPairDir {
            point: dir.point - pbar,
            edge: (dir.edge.0 - pbar, dir.edge.1 - pbar),
            index: pair.index,
        };

        let l_p = p.point.magnitude() + p.edge.0.magnitude().max(p.edge.1.magnitude());
        if l_p == 0f32 {
            return 1f32;
        }

        // let dsqr = pair.distance().powi(2);
        let d = x.distance();
        let g = self.s * d;

        let mut t = 0f32;
        let mut t_l = (1f32 - self.s) * d / l_p;

        loop {
            x.point += t_l * p.point;
            x.edge.0 += t_l * p.edge.0;
            x.edge.1 += t_l * p.edge.1;

            let d = x.distance();
            if t > 0f32 && d < g {
                return t;
            }

            t += t_l;
            if t > self.t_c {
                return self.t_c;
            }
            t_l = 0.9f32 * d / l_p;
        }

        t
    }
}
