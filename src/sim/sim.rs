use faer::Col;

use crate::{
    sim::{
        contact::contact::ContactPair,
        solver::{
            damped_newton::DampedNewtonSolver,
            damped_newton_with_contact::DampedNewtonSolverWithContact,
        },
    },
    RunConfig,
};

use super::{
    body::{
        body::{Body, GenericBody, Ip},
        springsbody::SpringsBodyIp,
    },
    solver::frame::NewtonFrame,
    solver::newton::NewtonSolver,
    utils::hess::Hess,
};

pub struct Boundary;
impl Boundary {
    // Left Right; Bottom Top

    pub fn left_bot() -> glm::Vec2 {
        return glm::vec2(-1., -1.);
    }

    pub fn left_top() -> glm::Vec2 {
        return glm::vec2(-1., 1.);
    }

    pub fn right_bot() -> glm::Vec2 {
        return glm::vec2(1., -1.);
    }

    pub fn right_top() -> glm::Vec2 {
        return glm::vec2(1., 1.);
    }
    pub fn points() -> Vec<glm::Vec2> {
        vec![
            Boundary::left_bot(),
            Boundary::left_top(),
            Boundary::right_bot(),
            Boundary::right_top(),
        ]
    }

    pub fn bot() -> (glm::Vec2, glm::Vec2) {
        return (Self::left_bot(), Self::right_bot());
    }

    pub fn top() -> (glm::Vec2, glm::Vec2) {
        return (Self::left_top(), Self::right_top());
    }

    pub fn left() -> (glm::Vec2, glm::Vec2) {
        return (Self::left_bot(), Self::left_top());
    }

    pub fn right() -> (glm::Vec2, glm::Vec2) {
        return (Self::right_bot(), Self::right_top());
    }

    pub fn edges() -> Vec<(glm::Vec2, glm::Vec2)> {
        vec![
            Boundary::bot(),
            Boundary::top(),
            Boundary::left(),
            Boundary::right(),
        ]
    }

    // pub fn collect_collision_pair_with_boundary(
    //     p: &glm::Vec2,
    //     pairs: &mut Vec<ContactPair>,
    //     dhat: f32,
    // ) {
    //     for edge in Boundary::edges() {
    //         let pair = ContactPair { edge, point: *p };
    //         if pair.distance() < dhat {
    //             pairs.push(pair);
    //         }
    //     }
    // }
}

pub struct Simulation {
    pub bodies: Vec<Body>,
    pub dof: Col<f32>,
    pub ndof: usize,

    // instantiate all IPs, hardcode is OK
    pub springsbody_ip: SpringsBodyIp,
}

impl Simulation {
    /// ### Move `gen_bodies` into the struct.
    /// 1. count dof
    /// 2. set all offsets
    pub fn new(gen_bodies: Vec<GenericBody>, run_config: &RunConfig) -> Self {
        let mut ndof = 0usize;
        let mut bodies: Vec<Body> = Vec::new();
        // count dof & set offset
        for gen_body in gen_bodies {
            let body: Body;
            match gen_body {
                GenericBody::Affine() => todo!(),
                GenericBody::Soft() => todo!(),
                GenericBody::Static(_) => todo!(),
                GenericBody::Springs(spbody) => {
                    // offset = current dof added
                    let offset = ndof;
                    // accumulate ndof
                    ndof += spbody.ndof;
                    body = Body::Springs(spbody, offset);
                }
            }
            bodies.push(body);
        }

        Simulation {
            ndof,
            dof: Col::<f32>::zeros(ndof),
            bodies,
            springsbody_ip: SpringsBodyIp::new(run_config),
        }
    }

    /// ### Initialize dof using body configurations.
    fn init_dof(&self) -> Col<f32> {
        let mut dof = Col::zeros(self.ndof);
        for body in &self.bodies {
            match body {
                Body::Affine() => todo!(),
                Body::Soft() => todo!(),
                Body::Springs(spbody, offset) => {
                    for i in 0..spbody.ndof {
                        let idof = i + offset;
                        dof[idof] = spbody.x[i];
                    }
                }
                Body::Static(stbody, offset) => {
                    for i in 0..stbody.ndof {
                        let idof = i + offset;
                        dof[idof] = stbody.x[i];
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
                Body::Affine() => todo!(),
                Body::Soft() => todo!(),
                Body::Springs(spbody, offset) => {
                    spbody.xprev.copy_from(&spbody.x);
                    // copy new x from dof to body
                    let mut dof = Col::<f32>::zeros(spbody.ndof);
                    dof.copy_from(self.dof.as_ref().subrows(*offset, spbody.ndof));

                    dof[0] = spbody.x[0];
                    dof[1] = spbody.x[1];
                    spbody.x.copy_from(&dof);
                    spbody.v = faer::scale(1f32 / self.springsbody_ip.run_cfg.dt)
                        * (&spbody.x - &spbody.xprev);
                }
                Body::Static(stbody, offset) => {
                    // copy new x from dof to body
                    let mut dof = Col::<f32>::zeros(stbody.ndof);
                    dof.copy_from(self.dof.as_ref().subrows(*offset, stbody.ndof));
                    stbody.x.copy_from(&dof);
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
        dhat: f32,
    ) {
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
                Body::Static(_, _) => (), // do nothing
            }
        }

        // run newton solver with closure as parameter.
        let dof_next = DampedNewtonSolverWithContact {
            max_iters,
            tol,
            max_linesearch_step,
            tau,
            beta,
            dhat,
        }
        .run(self, dof_init);

        self.dof.copy_from(dof_next);

        println!("Optimized dof: {:?}", self.dof.transpose());

        self.post_step();
    }

    pub fn step_damped_newton(
        &mut self,
        max_iters: u32,
        tol: f32,
        max_linesearch_step: u32,
        tau: f32,
        beta: f32,
    ) {
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
                Body::Static(_, _) => (), // do nothing
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
                        Body::Static(_, _) => (),
                    }
                }
                frame.hess.build();
            },
        );

        self.dof.copy_from(dof_next);

        println!("Optimized dof: {:?}", self.dof.transpose());

        self.post_step();
    }

    pub fn step_newton(&mut self, max_iters: u32, tol: f32) {
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
                Body::Static(_, _) => (), // do nothing
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
                        Body::Static(_, _) => (),
                    }
                }
                frame.hess.build();
            },
        );

        self.dof.copy_from(dof_next);

        println!("Optimized dof: {:?}", self.dof.transpose());

        self.post_step();
    }
}
