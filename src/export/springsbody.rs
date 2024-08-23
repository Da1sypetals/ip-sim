use crate::sim::{body::springsbody::SpringsBody, sim::Simulation, utils::misc::dof_index};

use super::base::Export;

impl Export for SpringsBody {
    fn export_nodes(&self) -> Vec<(f32, f32)> {
        let mut nodes = Vec::new();
        for ip in 0..self.ndof / 2 {
            let (ix, iy) = (ip * 2, ip * 2 + 1);
            nodes.push((self.x[ix], self.x[iy]));
        }
        nodes
    }

    fn export_edges(&self) -> Vec<((f32, f32), (f32, f32))> {
        let mut edges = Vec::new();
        for c in &self.constraints {
            let (i1x, i1y) = (c.i1 * 2, c.i1 * 2 + 1);
            let (i2x, i2y) = (c.i2 * 2, c.i2 * 2 + 1);
            edges.push(((self.x[i1x], self.x[i1y]), (self.x[i2x], self.x[i2y])));
        }
        edges
    }
}


