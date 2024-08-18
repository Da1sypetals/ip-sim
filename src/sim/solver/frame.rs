use crate::sim::utils::hess::Hess;
use faer::{solvers::SpSolver, Col};

/// For all data that are *unique* in a line search iteration,
pub struct NewtonFrame {
    pub ndof: usize,
    pub energy: f32,
    pub dof: Col<f32>,
    pub grad: Col<f32>,
    pub hess: Hess,
}

impl NewtonFrame {
    pub fn new(dof: &Col<f32>) -> Self {
        let ndof = dof.nrows();
        NewtonFrame {
            ndof,
            dof: dof.clone(),
            energy: 0f32,
            grad: Col::zeros(ndof),
            hess: Hess::new(ndof),
        }
    }
    pub fn new_iteration(&mut self) {
        self.grad.fill(0f32);
        self.hess.reset();
    }
    pub fn append_grad(&mut self, grad: &Col<f32>, offset: usize) {
        for i in 0..grad.nrows() {
            let idof = i + offset;
            self.grad[idof] += grad[i];
        }
    }
    pub fn append_hess(&mut self, hess: &Hess, offset: usize) {
        for (i, j, val) in &hess.trip {
            self.hess.add_elem(*i, *j, *val);
        }
    }
}
