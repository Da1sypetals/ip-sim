use core::alloc;

use super::frame::NewtonFrame;
use crate::sim::{
    body::body::{Body, Ip},
    contact::contact::ContactPair,
    sim::{Boundary, Simulation},
    utils::hess::Hess,
};
use faer::{solvers::SpSolver, Col};

pub struct DampedNewtonSolverWithContact {
    // meta
    pub max_iters: u32,
    pub tol: f32,
    pub max_linesearch_step: u32,
    pub tau: f32,  // line search step multiplier
    pub beta: f32, // armijo condition parameter
    pub dhat: f32,
}

impl DampedNewtonSolverWithContact {
    /// **Implements Armijo condition**
    /// - `alpha` may be initialized via CCD
    fn line_search(
        &self,
        sim: &mut Simulation,
        contact_pairs: &Vec<ContactPair>,
        start_frame: &NewtonFrame,
        dir: &Col<f32>,
        alpha_init: f32,
    ) -> f32 {
        let mut alpha = alpha_init;
        for i_search in 0..self.max_linesearch_step {
            // 1. current frame
            let cur_dof = &start_frame.dof + faer::scale(alpha) * dir;
            let mut cur_frame = NewtonFrame::new(&cur_dof);

            // 2. armijo condition
            DampedNewtonSolverWithContact::fill_frame(
                sim,
                &contact_pairs,
                &mut cur_frame,
                true,
                false,
                false,
            ); // fill energy of cur_frame
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

    /// **Assume dof is already filled**
    /// - use the current dof
    /// - and metadata from bodies.
    pub fn fill_frame(
        sim: &mut Simulation,
        contact_pairs: &Vec<ContactPair>,
        frame: &mut NewtonFrame,
        fill_energy: bool,
        fill_grad: bool,
        fill_hess: bool,
    ) {
        // todo: energy, grad and hessian of contact pairs.

        // energy, grad and hessian of bodies:
        for body in &sim.bodies {
            let dof = body.extract_dof(&frame.dof);
            match body {
                Body::Affine() => todo!(),
                Body::Soft() => todo!(),
                Body::Springs(spbody, offset) => {
                    // fill fields according to option
                    if fill_energy {
                        let energy = sim.springsbody_ip.value(spbody, &dof);
                        frame.energy += energy;
                    }
                    if fill_grad {
                        let mut grad = Col::<f32>::zeros(spbody.ndof);
                        sim.springsbody_ip.grad(spbody, &dof, &mut grad);
                        frame.append_grad(&grad, *offset);
                    }
                    if fill_hess {
                        let mut hess = Hess::new(spbody.ndof);
                        sim.springsbody_ip.hess(spbody, &dof, &mut hess);
                        frame.append_hess(&hess, *offset);
                    }
                }
            }
        }
        frame.hess.build();
    }

    /// # TODO!
    /// - Given configuration,
    pub fn find_contact_pairs(
        &self,
        frame: &NewtonFrame,
        sim: &mut Simulation,
    ) -> Vec<ContactPair> {
        return Vec::new();

        let mut contact_pairs = Vec::<ContactPair>::new();
        for body in &sim.bodies {
            let dof = body.extract_dof(&frame.dof);
            match body {
                Body::Affine() => todo!(),
                Body::Soft() => todo!(),
                Body::Springs(spbody, offset) => {
                    let n: usize = spbody.ndof / 2;
                    for i in 0..n {
                        // todo: add collision pair with static object: boundary
                        todo!()
                    }
                }
            }
        }
        contact_pairs
    }

    pub fn run(
        &mut self,
        sim: &mut Simulation,
        dof_init: Col<f32>, // move
    ) -> Col<f32> {
        println!("newton solve start!");
        // Frame that is in the *optimization path* but not search path.
        // Note: line search generates temporary frames.
        let mut frame = NewtonFrame::new(&dof_init);

        for iter in 0..self.max_iters {
            // clear grad and hess, initialize dof
            frame.new_iteration();

            // find contact pairs which contributes to IP
            let contact_pairs = self.find_contact_pairs(&frame, sim);
            // fill grad and hess of search starting frame
            DampedNewtonSolverWithContact::fill_frame(
                sim,
                &contact_pairs,
                &mut frame,
                true,
                true,
                true,
            );

            let direction = frame.hess.lu().solve(-&frame.grad);
            // dbg!(direction.transpose());

            // line search
            // todo: collision detection
            let alpha_init = 1f32;
            let alpha = self.line_search(sim, &contact_pairs, &frame, &direction, alpha_init);

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
