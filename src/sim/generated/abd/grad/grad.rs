use super::grad_orth_0::w_grad_orth_0;
use super::grad_orth_1::w_grad_orth_1;
use super::grad_orth_2::w_grad_orth_2;
use super::grad_orth_3::w_grad_orth_3;

pub fn orth_grad(a_vec: glm::Vec4) -> glm::Vec4 {
    let mut res = glm::Vec4::zeros();
    let (a11, a12, a21, a22) = (
        a_vec.x as f64,
        a_vec.y as f64,
        a_vec.z as f64,
        a_vec.w as f64,
    );

    res.x = w_grad_orth_0(a11, a12, a21, a11) as f32;
    res.y = w_grad_orth_1(a11, a12, a21, a11) as f32;
    res.z = w_grad_orth_2(a11, a12, a21, a11) as f32;
    res.w = w_grad_orth_3(a11, a12, a21, a11) as f32;

    res
}
