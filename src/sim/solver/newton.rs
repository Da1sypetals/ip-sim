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

pub struct NewtonSolver {
    // meta
    pub max_iters: u32,
    pub tol: f32,
}

impl NewtonSolver {
    pub fn new(max_iters: u32, tol: f32) -> Self {
        NewtonSolver { max_iters, tol }
    }
    pub fn run(
        &mut self,
        dof_init: Col<f32>, // move
        mut fill_grad_hess: impl FnMut(&mut NewtonFrame) -> (),
    ) -> Col<f32> {
        println!("newton solve start!");
        // Frame that is in the *optimization path* but not search path.
        // Note: line search generates temporary frames.
        let mut frame = NewtonFrame::new(&dof_init);

        for iter in 0..self.max_iters {
            // clear grad and hess, initialize dof
            frame.new_iteration();

            fill_grad_hess(&mut frame);
            let direction = frame.hess.lu().solve(-&frame.grad);

            let norm_dir = direction.norm_l2();
            if norm_dir < self.tol {
                break;
            }

            frame.dof += &direction;
            println!("Iteration {}, norm of direction = {}", iter, norm_dir);
        }

        frame.dof

        // todo!()
    }
}
