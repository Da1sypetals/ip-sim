use crate::sim::contact::contact::ContactHessType;

/// View dof as nodes.
pub fn dof_index(inode: usize, offset: usize) -> (usize, usize) {
    (offset + inode * 2, offset + inode * 2 + 1)
}

pub fn hess_spd_proj(hess_raw: &ContactHessType) -> ContactHessType {
    // hess projection
    let hess_svd = hess_raw.svd(true, true);
    let eig = hess_svd
        .singular_values
        .map(|x| if x > 0f32 { x } else { 0f32 });
    let eig_diag = ContactHessType::from_diagonal(&eig);

    // return
    hess_svd.u.unwrap() * eig_diag * hess_svd.v_t.unwrap()
}
