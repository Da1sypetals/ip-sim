use faer::Col;

use crate::sim::utils::{
    hess::Hess,
    types::{Mat6x6, Vec6},
};

pub trait AffineDof {
    type Ttype;
    type Atype;

    fn t_unpack(&self) -> (f32, f32);
    fn a_unpack(&self) -> (f32, f32, f32, f32);
    fn t_vec(&self) -> glm::Vec2;
    fn a_vec(&self) -> glm::Vec4;
    fn a_mat(&self) -> glm::Mat2x2;
    fn from_t_a(t: Self::Ttype, a: Self::Atype) -> Self;
}

impl AffineDof for Vec6 {
    type Ttype = glm::Vec2;
    type Atype = glm::Mat2x2;

    fn from_t_a(t: Self::Ttype, a: Self::Atype) -> Self {
        Self::new(t.x, t.y, a[(0, 0)], a[(0, 1)], a[(1, 0)], a[(1, 1)])
    }

    fn t_unpack(&self) -> (f32, f32) {
        (self.x, self.y)
    }

    fn a_unpack(&self) -> (f32, f32, f32, f32) {
        (self.z, self.w, self.a, self.b)
    }

    fn t_vec(&self) -> glm::Vec2 {
        self.xy()
    }

    fn a_vec(&self) -> glm::Vec4 {
        glm::vec4(self.z, self.w, self.a, self.b)
    }

    fn a_mat(&self) -> glm::Mat2x2 {
        glm::mat2x2(self.z, self.w, self.a, self.b)
    }
}

pub trait InteropCol {
    fn from_col(col: &Col<f32>) -> Self;
    fn from_dof(dof: &Col<f32>, offset: usize) -> Self;
    fn to_col(&self) -> Col<f32>;
}

pub trait InteropHess<const N: usize> {
    fn to_hess(&self) -> Hess;
    fn append_to_hess(&self, res: &mut Hess) {
        let hess = self.to_hess();
        for (i, j, val) in hess.trip {
            res.add_elem(i, j, val);
        }
    }
}

impl InteropCol for Vec6 {
    fn from_dof(dof: &Col<f32>, offset: usize) -> Self {
        assert!(offset + 6 <= dof.nrows());
        let mut res = Vec6::zeros();
        for i in 0..6 {
            res[i] = dof[i + offset];
        }
        res
    }

    fn from_col(col: &Col<f32>) -> Self {
        assert_eq!(col.nrows(), 6);
        let mut res = Vec6::zeros();
        for i in 0..6 {
            res[i] = col[i];
        }
        res
    }

    fn to_col(&self) -> Col<f32> {
        let mut res = Col::zeros(6);
        for i in 0..6 {
            res[i] = self[i];
        }
        res
    }
}

impl InteropHess<6> for Mat6x6 {
    fn to_hess(&self) -> Hess {
        let mut res = Hess::new(6);
        for (i, j) in (0..6).zip(0..6) {
            res.add_elem(i, j, self[(i, j)]);
        }
        res
    }
}
