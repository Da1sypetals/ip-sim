use core::alloc;

use super::frame::NewtonFrame;
use crate::sim::utils::hess::Hess;
use faer::{solvers::SpSolver, Col};

pub struct DampedNewtonSolver {
    // meta
    pub max_iters: u32,
    pub tol: f32,
    pub max_linesearch_step: u32,
    pub tau: f32,  // line search step multiplier
    pub beta: f32, // armijo condition parameter
}

impl DampedNewtonSolver {
    /// **Implements Armijo condition**
    /// - `alpha` may be initialized via CCD
    fn line_search(
        &self,
        start_frame: &NewtonFrame,
        dir: &Col<f32>,
        alpha_init: f32,
        fill_frame: &mut impl FnMut(&mut NewtonFrame, bool, bool, bool) -> (),
    ) -> f32 {
        let mut alpha = alpha_init;
        for i_search in 0..self.max_linesearch_step {
            // 1. current frame
            let cur_dof = &start_frame.dof + faer::scale(alpha) * dir;
            let mut cur_frame = NewtonFrame::new(&cur_dof);

            // 2. armijo condition
            fill_frame(&mut cur_frame, true, false, false); // fill energy of cur_frame
            let dot = start_frame.grad.transpose() * dir;
            let armijo = cur_frame.energy <= start_frame.energy + alpha * self.beta * dot;
            if armijo {
                println!("<!> armijo condition satisfied! alpha = {}", alpha);
                break;
            }

            alpha *= self.tau;

            // println!(" > Line search iteration {}", i_search);
        }
        alpha
    }

    pub fn run(
        &mut self,
        dof_init: Col<f32>, // move
        mut fill_frame: impl FnMut(&mut NewtonFrame, bool, bool, bool) -> (),
    ) -> Col<f32> {
        println!("newton solve start!");
        // Frame that is in the *optimization path* but not search path.
        // Note: line search generates temporary frames.
        let mut frame = NewtonFrame::new(&dof_init);

        for iter in 0..self.max_iters {
            // clear grad and hess, initialize dof
            frame.new_iteration();

            fill_frame(&mut frame, true, true, true);
            let direction = frame.hess.lu().solve(-&frame.grad);
            // dbg!(direction.transpose());

            // line search
            // todo: collision detection
            let alpha_init = 1f32;
            let alpha = self.line_search(&frame, &direction, alpha_init, &mut fill_frame);

            frame.dof += faer::scale(alpha) * &direction;

            let norm_dir = direction.norm_l2();
            println!("Iteration {}, norm of direction = {}", iter, norm_dir);

            // quit criteria
            if norm_dir < self.tol {
                break;
            }
        }

        frame.dof

        // todo!()
    }
}
