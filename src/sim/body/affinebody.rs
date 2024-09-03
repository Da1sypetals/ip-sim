use super::body::Ip;
use crate::{
    sim::{
        generated::orth::{grad::orth_grad, hess::orth_hess},
        utils::{
            affine_utils::interop::{AffineDof, InteropCol, InteropHess},
            hess::Hess,
            polygon::Polygon,
            types::Vec6,
        },
    },
    RunConfig,
};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub struct AffineBody {
    pub ndof: usize, // = 6
    /// number of vertices
    pub nvert: usize,

    pub q: Vec6,
    pub dq: Vec6,
    pub qtilde: Vec6,

    pub poly: Polygon,
    pub kappa: f32,
}

impl AffineBody {
    pub fn edges_enumerate(&self, q: &Vec6) -> Vec<((usize, usize), (glm::Vec2, glm::Vec2))> {
        let mut res = Vec::new();
        for i in 0..self.nvert {
            let iu = i;
            let iv = (i + 1) % self.nvert;
            let e = (self.pos(q, iu), self.pos(q, iv));
            res.push(((iu, iv), e));
        }
        res
    }

    pub fn edges_and_deltas(
        &self,
        q: &Vec6,
        dq: &Vec6,
    ) -> Vec<((glm::Vec2, glm::Vec2), (glm::Vec2, glm::Vec2))> {
        let mut res = Vec::new();
        for i in 0..self.nvert {
            let iu = i;
            let iv = (i + 1) % self.nvert;
            let e = (self.pos(q, iu), self.pos(q, iv));
            let de = (self.pos_delta(q, dq, iu), self.pos_delta(q, dq, iv));
            res.push((e, de));
        }
        res
    }

    pub fn new(poly: Polygon, kappa: f32, x0: f32, y0: f32) -> Self {
        let scale = 1f32;
        let q0 = Vec6::new(x0, y0, scale, 0.0, 0.0, scale);
        Self {
            ndof: 6,
            nvert: poly.n,
            poly,
            q: q0,
            dq: Vec6::zeros(),
            qtilde: Vec6::zeros(),
            kappa,
        }
    }

    pub fn from_file(path: &str) -> io::Result<Self> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let mut nodes = Vec::new();
        let mut density = 0.0;
        let mut kappa: f32 = 0.0;
        let mut section = None;
        let (mut x0, mut y0) = (0f32, 0f32);

        for line in reader.lines() {
            let line = line?;
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            if line.starts_with("!") {
                match line.split_whitespace().next().unwrap() {
                    "!density" => {
                        if let Some(density_str) = line.split_whitespace().nth(1) {
                            density = density_str.parse().map_err(|_| {
                                io::Error::new(io::ErrorKind::InvalidData, "Invalid density format")
                            })?;
                        } else {
                            return Err(io::Error::new(
                                io::ErrorKind::InvalidData,
                                "Density value missing",
                            ));
                        }
                        section = None; // Move to the next section
                    }
                    "!kappa" => {
                        if let Some(kappa_str) = line.split_whitespace().nth(1) {
                            kappa = kappa_str.parse().map_err(|_| {
                                io::Error::new(io::ErrorKind::InvalidData, "Invalid density format")
                            })?;
                        } else {
                            return Err(io::Error::new(
                                io::ErrorKind::InvalidData,
                                "Kappa value missing",
                            ));
                        }
                        section = None; // Move to the next section
                    }
                    "!translate" => {
                        if let (Some(x0s), Some(y0s)) = (
                            line.split_whitespace().nth(1),
                            line.split_whitespace().nth(2),
                        ) {
                            x0 = x0s.parse().map_err(|_| {
                                io::Error::new(io::ErrorKind::InvalidData, "Invalid density format")
                            })?;
                            y0 = y0s.parse().map_err(|_| {
                                io::Error::new(io::ErrorKind::InvalidData, "Invalid density format")
                            })?;
                        } else {
                            return Err(io::Error::new(
                                io::ErrorKind::InvalidData,
                                "Kappa value missing",
                            ));
                        }
                        section = None; // Move to the next section
                    }
                    "!nodes" => section = Some("nodes"),
                    "!end" => break,
                    _ => {
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            "Unknown section",
                        ))
                    }
                }
                continue;
            }

            match section {
                Some("nodes") => {
                    let coords: Vec<f32> = line
                        .split_whitespace()
                        .map(|s| {
                            s.parse().map_err(|_| {
                                io::Error::new(io::ErrorKind::InvalidData, "Invalid node format")
                            })
                        })
                        .collect::<Result<Vec<f32>, _>>()?;
                    if coords.len() != 2 {
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            "Each node must have exactly 2 coordinates",
                        ));
                    }
                    nodes.push(glm::vec2(coords[0], coords[1]));
                }
                _ => {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "Unexpected data outside of sections",
                    ))
                }
            }
        }

        if nodes.is_empty() {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "No nodes found"));
        }

        let poly = Polygon::new(nodes, density);

        Ok(Self::new(poly, kappa, x0, y0))
    }

    pub fn index(offset: usize) -> (usize, usize, usize, usize, usize, usize) {
        (
            offset,
            offset + 1,
            offset + 2,
            offset + 3,
            offset + 4,
            offset + 5,
        )
    }

    pub fn pos(&self, q: &Vec6, i: usize) -> glm::Vec2 {
        let a = q.a_mat();
        let t = q.t_vec();
        let p = self.poly.nodes[i];
        a * p + t
    }

    /// Given current frame `q` and search (direction) vector `q_delta`,
    /// return pos_delta.
    /// - (Position incremented) - (position original)
    pub fn pos_delta(&self, q: &Vec6, q_delta: &Vec6, i: usize) -> glm::Vec2 {
        self.pos(&(q + q_delta), i) - self.pos(q, i)
    }

    pub fn pos_init(&self, i: usize) -> glm::Vec2 {
        self.poly.nodes[i]
    }
}

pub struct AffineBodyIp {
    pub rc: RunConfig,
    orth: OrthIp,
    inertial: AbInertialIp,
}

impl AffineBodyIp {
    pub fn new(rc: &RunConfig) -> Self {
        Self {
            rc: rc.clone(),
            orth: OrthIp { rc: rc.clone() },
            inertial: AbInertialIp { rc: rc.clone() },
        }
    }
}

impl Ip for AffineBodyIp {
    type BodyT = AffineBody;

    fn prepare(&self, body: &mut Self::BodyT) {
        self.orth.prepare(body);
        self.inertial.prepare(body);
    }

    fn value(&self, body: &Self::BodyT, dof: &faer::Col<f32>) -> f32 {
        self.orth.value(body, dof) + self.inertial.value(body, dof)
    }

    fn grad(&self, body: &Self::BodyT, dof: &faer::Col<f32>, result: &mut faer::Col<f32>) {
        self.orth.grad(body, dof, result);
        self.inertial.grad(body, dof, result);
    }

    fn hess(&self, body: &Self::BodyT, dof: &faer::Col<f32>, result: &mut Hess) {
        self.orth.hess(body, dof, result);
        self.inertial.hess(body, dof, result);
    }
}

pub struct AbInertialIp {
    pub rc: RunConfig,
}

impl Ip for AbInertialIp {
    type BodyT = AffineBody;

    fn prepare(&self, body: &mut Self::BodyT) {
        let g = glm::vec2(0f32, self.rc.g);
        let g_q = body.poly.uniform_accel(g);
        body.qtilde =
            body.q + self.rc.dt * body.dq + self.rc.dt * self.rc.dt * body.poly.mass_inv() * g_q;
    }

    fn value(&self, body: &Self::BodyT, dof: &faer::Col<f32>) -> f32 {
        let q = Vec6::from_col(dof);
        let qdiff = q - body.qtilde;
        0.5f32 * (qdiff.transpose() * body.poly.mass_matrix() * qdiff).x
    }

    fn grad(&self, body: &Self::BodyT, dof: &faer::Col<f32>, result: &mut faer::Col<f32>) {
        let q = Vec6::from_col(dof);
        let _grad = body.poly.mass_matrix() * (q - body.qtilde);
        *result += _grad.to_col();
    }

    fn hess(&self, body: &Self::BodyT, dof: &faer::Col<f32>, result: &mut Hess) {
        let _hess = body.poly.mass_matrix();
        _hess.append_to_hess(result);
    }
}

pub struct OrthIp {
    pub rc: RunConfig,
}

impl Ip for OrthIp {
    type BodyT = AffineBody;

    /// size of `dof` = size of `q` = 6
    fn prepare(&self, body: &mut Self::BodyT) {
        ()
    }

    fn value(&self, body: &Self::BodyT, dof: &faer::Col<f32>) -> f32 {
        let (a11, a12, a21, a22) = Vec6::from_col(dof).a_unpack();
        body.kappa
            * (2f32 * (a11 * a12 + a21 * a22).powi(2)
                + (a11.powi(2) + a21.powi(2) - 1f32).powi(2)
                + (a12.powi(2) + a22.powi(2) - 1f32).powi(2))
    }

    fn grad(&self, body: &Self::BodyT, dof: &faer::Col<f32>, result: &mut faer::Col<f32>) {
        let (a11, a12, a21, a22) = Vec6::from_col(dof).a_unpack();
        let _grad = body.kappa * orth_grad(a11, a12, a21, a22);
        *result += _grad.to_col();
    }

    fn hess(&self, body: &Self::BodyT, dof: &faer::Col<f32>, result: &mut Hess) {
        let (a11, a12, a21, a22) = Vec6::from_col(dof).a_unpack();
        let _hess = body.kappa * orth_hess(a11, a12, a21, a22);
        // assume only dof from `self`
        _hess.append_to_hess(result);
    }
}
