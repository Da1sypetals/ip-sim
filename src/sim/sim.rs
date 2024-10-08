use faer::Col;

use crate::{
    export::base::VisFrameVec,
    sim::solver::{
        damped_newton::DampedNewtonSolver,
        damped_newton_with_contact::DampedNewtonSolverWithContact,
    },
    RunConfig,
};

use super::{
    body::{
        affinebody::AffineBodyIp,
        body::{Body, GenericBody, Ip},
        springsbody::{SpringsBody, SpringsBodyIp},
    },
    contact::affine_contact::ContactElemIp,
    solver::{frame::NewtonFrame, newton::NewtonSolver},
    utils::{affine_utils::interop::InteropCol, hess::Hess, types::Vec6},
};

pub struct Simulation {
    pub bodies: Vec<Body>,
    pub dof: Col<f32>,
    pub ndof: usize,

    // instantiate all IPs, hardcode is OK
    pub springsbody_ip: SpringsBodyIp,
    pub contact_ip: ContactElemIp,
    pub affinebody_ip: AffineBodyIp,

    // serialization related
    pub vis_frames: VisFrameVec,
}

impl Simulation {
    /// ### Move `gen_bodies` into the struct.
    /// 1. count dof
    /// 2. set all offsets
    pub fn new(gen_bodies: Vec<GenericBody>, run_config: &RunConfig) -> Self {
        let mut ndof_accumulated = 0usize;
        let mut bodies: Vec<Body> = Vec::new();
        // count dof & set offset
        for gen_body in gen_bodies {
            let body: Body;
            let offset = ndof_accumulated;
            match gen_body {
                GenericBody::Affine(ab) => {
                    body = Body::Affine(ab, offset);
                }
                GenericBody::Soft() => todo!(),
                GenericBody::Springs(spbody) => {
                    // offset = current dof added
                    // accumulate ndof
                    body = Body::Springs(spbody, offset);
                }
            }
            ndof_accumulated += body.get_ndof();
            bodies.push(body);
        }

        Simulation {
            ndof: ndof_accumulated,
            dof: Col::<f32>::zeros(ndof_accumulated),
            bodies,
            springsbody_ip: SpringsBodyIp::new(run_config),
            contact_ip: ContactElemIp::new(run_config),
            affinebody_ip: AffineBodyIp::new(run_config),
            vis_frames: VisFrameVec { frames: Vec::new() },
        }
    }

    /// ### Initialize dof using body configurations.
    fn init_dof(&self) -> Col<f32> {
        let mut dof = Col::zeros(self.ndof);
        for body in &self.bodies {
            match body {
                Body::Affine(ab, offset) => {
                    for i in 0..6 {
                        let idof = i + offset;
                        dof[idof] = ab.q[i];
                    }
                }
                Body::Soft() => todo!(),
                Body::Springs(spbody, offset) => {
                    for i in 0..spbody.ndof {
                        let idof = i + offset;
                        dof[idof] = spbody.x[i];
                    }
                }
            }
        }
        dof
    }

    /// 1. write dof into bodies
    /// 2. update body data (xprev, v, etc.)
    fn post_step(&mut self) {
        for (i, body) in self.bodies.iter_mut().enumerate() {
            match body {
                Body::Affine(ab, offset) => {
                    let new_q = Vec6::from_dof(&self.dof, *offset);

                    ab.dq = (new_q - ab.q) / self.affinebody_ip.rc.dt;
                    ab.q = new_q;
                }
                Body::Soft() => todo!(),
                Body::Springs(spbody, offset) => {
                    spbody.xprev.copy_from(&spbody.x);
                    // copy new x from dof to body
                    let mut dof = Col::<f32>::zeros(spbody.ndof);
                    dof.copy_from(self.dof.as_ref().subrows(*offset, spbody.ndof));

                    // dof[0] = spbody.x[0];
                    // dof[1] = spbody.x[1];

                    // update position and velocity
                    spbody.x.copy_from(&dof);
                    spbody.v = faer::scale(1f32 / self.springsbody_ip.run_cfg.dt)
                        * (&spbody.x - &spbody.xprev);
                }
            }
        }
    }

    pub fn step_damped_newton_with_contact(
        &mut self,
        max_iters: u32,
        tol: f32,
        max_linesearch_step: u32,
        tau: f32,
        beta: f32,
        s: f32,
        accd_max_iter: u32,
    ) {
        println!("step start!");
        let dof_init = self.init_dof();

        println!("Init dof: {:?}", dof_init.transpose());

        // call `prepare` for all bodies
        for body in &mut self.bodies {
            match body {
                Body::Affine(ab, _) => self.affinebody_ip.prepare(ab),
                Body::Soft() => todo!(),
                Body::Springs(spbody, _) => self.springsbody_ip.prepare(spbody),
            }
        }

        // run newton solver with closure as parameter.
        let dof_next = DampedNewtonSolverWithContact {
            max_iters,
            tol,
            max_linesearch_step,
            tau,
            beta,
            s,
            accd_max_iter,
        }
        .run(self, dof_init);

        self.dof.copy_from(dof_next);

        println!("Optimized dof: {:?}", self.dof.transpose());

        self.post_step();
    }

    #[deprecated]
    pub fn step_damped_newton(
        &mut self,
        max_iters: u32,
        tol: f32,
        max_linesearch_step: u32,
        tau: f32,
        beta: f32,
    ) {
        todo!()
        /*

        println!("step start!");
        let dof_init = self.init_dof();

        println!("Init dof: {:?}", dof_init.transpose());

        // call `prepare` for all bodies
        for body in &mut self.bodies {
            match body {
                Body::Affine() => todo!(),
                Body::Soft() => todo!(),
                Body::Springs(spbody, _) => {
                    self.springsbody_ip.prepare(spbody);
                }
            }
        }

        // run newton solver with closure as parameter.
        let dof_next = DampedNewtonSolver {
            max_iters,
            tol,
            max_linesearch_step,
            tau,
            beta,
        }
        .run(
            dof_init,
            // closure: compute energy, grad and hess: given DOF
            |frame: &mut NewtonFrame, fill_energy: bool, fill_grad: bool, fill_hess: bool| {
                for body in &mut self.bodies {
                    match body {
                        Body::Affine() => todo!(),
                        Body::Soft() => todo!(),
                        Body::Springs(spbody, offset) => {
                            let mut dof = Col::<f32>::zeros(spbody.ndof);
                            dof.copy_from(frame.dof.as_ref().subrows(*offset, spbody.ndof));

                            // fill fields according to option
                            if fill_energy {
                                let energy = self.springsbody_ip.value(spbody, &dof);
                                frame.energy += energy;
                            }
                            if fill_grad {
                                let mut grad = Col::<f32>::zeros(spbody.ndof);
                                self.springsbody_ip.grad(spbody, &dof, &mut grad);
                                frame.append_grad(&grad, *offset);
                            }
                            if fill_hess {
                                let mut hess = Hess::new(spbody.ndof);
                                self.springsbody_ip.hess(spbody, &dof, &mut hess);
                                frame.append_hess(&hess, *offset);
                            }
                        }
                    }
                }
                frame.hess.build();
            },
        );

        self.dof.copy_from(dof_next);

        println!("Optimized dof: {:?}", self.dof.transpose());

        self.post_step();

         */
    }

    #[deprecated]
    pub fn step_newton(&mut self, max_iters: u32, tol: f32) {
        todo!()
        /*

        println!("step start!");
        let dof_init = self.init_dof();

        println!("Init dof: {:?}", dof_init.transpose());

        // call `prepare` for all bodies
        for body in &mut self.bodies {
            match body {
                Body::Affine() => todo!(),
                Body::Soft() => todo!(),
                Body::Springs(spbody, _) => {
                    self.springsbody_ip.prepare(spbody);
                }
            }
        }

        // run newton solver with closure as parameter.
        let dof_next = NewtonSolver { max_iters, tol }.run(
            dof_init,
            // closure: compute energy, grad and hess: given DOF
            |frame: &mut NewtonFrame| {
                for body in &mut self.bodies {
                    match body {
                        Body::Affine() => todo!(),
                        Body::Soft() => todo!(),
                        Body::Springs(spbody, offset) => {
                            let mut dof = Col::<f32>::zeros(spbody.ndof);
                            dof.copy_from(frame.dof.as_ref().subrows(*offset, spbody.ndof));
                            let mut grad = Col::<f32>::zeros(spbody.ndof);
                            let mut hess = Hess::new(spbody.ndof);

                            // replace dof here with springdof type.
                            let energy = self.springsbody_ip.value(spbody, &dof);
                            self.springsbody_ip.grad(spbody, &dof, &mut grad);
                            self.springsbody_ip.hess(spbody, &dof, &mut hess);

                            frame.energy += energy;
                            frame.append_grad(&grad, *offset);
                            frame.append_hess(&hess, *offset);

                            // todo!()
                        }
                    }
                }
                frame.hess.build();
            },
        );

        self.dof.copy_from(dof_next);

        println!("Optimized dof: {:?}", self.dof.transpose());

        self.post_step();


         */
    }
}
