/// View dof as nodes.
pub fn dof_index(inode: usize, offset: usize) -> (usize, usize) {
    (offset + inode * 2, offset + inode * 2 + 1)
}

// pub fn hess_spd_proj(hess_raw: &ContactHessType) -> ContactHessType {
//     // hess projection
//     let hess_svd = hess_raw.svd(true, true);
//     let eig = hess_svd
//         .singular_values
//         .map(|x| if x > 0f32 { x } else { 0f32 });
//     let eig_diag = ContactHessType::from_diagonal(&eig);

//     // return
//     hess_svd.u.unwrap() * eig_diag * hess_svd.v_t.unwrap()
// }

pub fn hess_spd_proj(hess_raw: &faer::Mat<f32>) -> faer::Mat<f32> {
    let svd = hess_raw.svd();
    let mut eig = faer::Col::<f32>::zeros(hess_raw.nrows());
    eig.copy_from(svd.s_diagonal());
    for i in 0..eig.nrows() {
        eig[i] = if eig[i] > 0f32 { eig[i] } else { 0f32 }
    }
    svd.u() * eig.column_vector_into_diagonal() * svd.v().transpose()
}
