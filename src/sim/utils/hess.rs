use faer::sparse::SparseColMat;

pub struct Hess {
    pub n: usize,
    pub trip: Vec<(usize, usize, f32)>,
    pub csc: Option<SparseColMat<usize, f32>>,
}

impl Hess {
    fn check_content(&self) {
        let _ = self
            .csc
            .as_ref()
            .expect("Hess is not built! call hess.build first");
    }
    pub fn new(n: usize) -> Self {
        Hess {
            trip: Vec::new(),
            csc: None,
            n: n,
        }
    }
    pub fn reset(&mut self) {
        self.trip.clear();
        self.csc = None;
    }
    pub fn add_elem(&mut self, i: usize, j: usize, val: f32) {
        // ownership transfer
        self.trip.push((i, j, val))
    }
    pub fn build(&mut self) {
        self.csc = Some(
            SparseColMat::try_new_from_triplets(self.n, self.n, &self.trip)
                .expect("Hess build failed!"),
        );
    }
    pub fn lu(&self) -> faer::sparse::linalg::solvers::Lu<usize, f32> {
        self.check_content();
        let lu = (self.csc.as_ref())
            .expect("Call build() first!")
            .sp_lu()
            .expect("LLt decomposition failed!");
        lu
    }
    pub fn llt(&self) -> faer::sparse::linalg::solvers::Cholesky<usize, f32> {
        self.check_content();
        let llt = (self.csc.as_ref())
            .expect("Call build() first!")
            .sp_cholesky(faer::Side::Lower)
            .expect("LLt decomposition failed!");
        llt
    }
}
