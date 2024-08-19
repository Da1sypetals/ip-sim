use super::{springsbody::SpringsBody, staticbody::StaticBody};
use crate::sim::utils::hess::Hess;
use faer::Col;

pub trait Ip {
    type BodyT;
    /// ### Everything that only gets computed ONCE per STEP.
    fn prepare(&self, body: &mut Self::BodyT);
    fn value(&self, body: &Self::BodyT, dof: &Col<f32>) -> f32;
    fn grad(&self, body: &Self::BodyT, dof: &Col<f32>, result: &mut Col<f32>);
    fn hess(&self, body: &Self::BodyT, dof: &Col<f32>, result: &mut Hess);
}

pub enum GenericBody {
    // together with its `dof_offset`
    Affine(),
    Soft(),
    Springs(SpringsBody),
}

impl GenericBody {
    pub fn get_ndof(&self) -> usize {
        match self {
            GenericBody::Affine() => todo!(),
            GenericBody::Soft() => todo!(),
            GenericBody::Springs(spbody) => spbody.ndof,
        }
    }
}

pub enum Body {
    // together with its `dof_offset`
    Affine(),
    Soft(),
    Springs(SpringsBody, usize),
}

impl Body {
    pub fn extract_dof(&self, full_dof: &Col<f32>) -> Col<f32> {
        match self {
            Body::Affine() => todo!(),
            Body::Soft() => todo!(),
            Body::Springs(spbody, offset) => {
                let ndof = spbody.ndof;
                let mut res = Col::zeros(ndof);
                res.copy_from(full_dof.as_ref().subrows(*offset, spbody.ndof));
                res
            }
        }
    }
    pub fn get_offset(&self) -> usize {
        match self {
            Body::Affine() => todo!(),
            Body::Soft() => todo!(),
            Body::Springs(_, offset) => *offset,
        }
    }
    pub fn get_ndof(&self) -> usize {
        match self {
            Body::Affine() => todo!(),
            Body::Soft() => todo!(),
            Body::Springs(spbody, _) => spbody.ndof,
        }
    }
}
