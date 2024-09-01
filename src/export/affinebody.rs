use crate::sim::body::affinebody::AffineBody;

use super::base::Export;

impl Export for AffineBody {
    fn export_nodes(&self) -> Vec<(f32, f32)> {
        let mut res = Vec::new();
        for i in 0..self.nvert {
            let node = self.pos(&self.q, i);
            res.push((node.x, node.y));
        }
        res
    }

    fn export_edges(&self) -> Vec<((f32, f32), (f32, f32))> {
        let mut res = Vec::new();
        for iv in 0..self.nvert {
            let (i, j) = (iv, (iv + 1) % self.nvert);
            let pi = self.pos(&self.q, i);
            let pj = self.pos(&self.q, j);
            res.push(((pi.x, pi.y), (pj.x, pj.y)));
        }

        res
    }
}
