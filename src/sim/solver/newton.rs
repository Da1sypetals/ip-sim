use super::frame::NewtonFrame;
use crate::sim::utils::hess::Hess;
use faer::{solvers::SpSolver, Col};

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
        mut fill_frame: impl FnMut(&mut NewtonFrame) -> (),
    ) -> Col<f32> {
        println!("newton solve start!");
        // Frame that is in the *optimization path* but not search path.
        // Note: line search generates temporary frames.
        let mut frame = NewtonFrame::new(&dof_init);

        for iter in 0..self.max_iters {
            // clear grad and hess, initialize dof
            frame.new_iteration();

            fill_frame(&mut frame);
            let direction = frame.hess.lu().solve(-&frame.grad);

            let norm_dir = direction.norm_l2();
            // quit criteria
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
