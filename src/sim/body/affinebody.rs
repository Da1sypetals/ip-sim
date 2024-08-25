use super::body::Ip;
use crate::sim::generated::abd::grad::grad::orth_grad;
use crate::sim::utils::hess::Hess;
use crate::sim::utils::misc::MatrixMisc;
use crate::RunConfig;
use faer::{unzipped, zipped, Col};
use nalgebra::U6;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

pub type MassMatType = nalgebra::Matrix<f32, U6, U6, nalgebra::ArrayStorage<f32, 6, 6>>;

/// `a` and `t` are not arbitrary names.
/// - `a` is for **a**ffine transform
/// - `t` is for **t**ranslation
pub struct AbdProps;
impl AbdProps {
    /// all dof passed in are AB dofs (size=6)
    pub fn a_mat(dof: &Col<f32>) -> glm::Mat2 {
        assert_eq!(dof.nrows(), 6);
        glm::mat2(dof[0], dof[1], dof[2], dof[3])
    }
    pub fn a_vec(dof: &Col<f32>) -> glm::Vec4 {
        assert_eq!(dof.nrows(), 6);
        glm::vec4(dof[0], dof[1], dof[2], dof[3])
    }
    pub fn t_vec(dof: &Col<f32>) -> glm::Vec2 {
        assert_eq!(dof.nrows(), 6);
        glm::vec2(dof[4], dof[5])
    }
}

pub struct AffineBody {
    pub ndof: usize, // = 6

    /// the 6 dofs of 2d affine body
    /// [t1, t2, a11, a12, a21, a22]
    pub q: Col<f32>,
    /// derivative to time
    pub dq: Col<f32>,
    /// initial node / edge configurations
    pub nodes: Vec<glm::Vec2>,
    /// each u32 is a node index
    pub edges: Vec<(usize, usize)>,
    /// Mass matrix is 6x6, we use dense storage
    pub mass: MassMatType,

    // constants
    pub kappa: f32,
    pub volume: f32,
}

impl AffineBody {
    pub fn num_edges(&self) -> usize {
        self.edges.len()
    }

    pub fn node(&self, dof: &Col<f32>, inode: usize) -> glm::Vec2 {
        assert_eq!(dof.nrows(), 6);
        let a = AbdProps::a_mat(dof);
        let x0 = self.nodes[inode];
        a * x0
    }

    pub fn edge(&self, dof: &Col<f32>, iedge: usize) -> (glm::Vec2, glm::Vec2) {
        let (i, j) = self.edges[iedge];
        (self.node(dof, i), self.node(dof, j))
    }

    // todo:
    // 1. new
    // 2. from_file
}

pub struct AffineOrthIp;

impl Ip for AffineOrthIp {
    type BodyT = AffineBody;

    fn prepare(&self, body: &mut Self::BodyT) {
        () // do nothing
    }

    fn value(&self, body: &Self::BodyT, dof: &Col<f32>) -> f32 {
        let a = AbdProps::a_mat(dof);
        let at = a.transpose();
        // frobenius norm
        body.kappa * body.volume * (a * at - glm::Mat2::identity()).norm_squared()
    }

    /// result is 6-dof
    fn grad(&self, body: &Self::BodyT, dof: &Col<f32>, result: &mut Col<f32>) {
        let a_vec = AbdProps::a_vec(dof);
        let gr = body.kappa * body.volume * orth_grad(a_vec);

        for i in 2..6 {
            result[i] += gr[i];
        }
    }

    /// ## TODO!
    fn hess(&self, body: &Self::BodyT, dof: &Col<f32>, result: &mut Hess) {
        // todo!()
    }
}

pub struct AffineInertialIp;

/// Refer to ABD paper, page 4 and 5
impl Ip for AffineInertialIp {
    type BodyT = AffineBody;

    fn prepare(&self, body: &mut Self::BodyT) {
        // 1. compute qtilde
        // - how to compute f (external force)?
        todo!()
    }

    fn value(&self, body: &Self::BodyT, dof: &Col<f32>) -> f32 {
        todo!()
    }

    fn grad(&self, body: &Self::BodyT, dof: &Col<f32>, result: &mut Col<f32>) {
        todo!()
    }

    fn hess(&self, body: &Self::BodyT, dof: &Col<f32>, result: &mut Hess) {
        todo!()
    }
}
