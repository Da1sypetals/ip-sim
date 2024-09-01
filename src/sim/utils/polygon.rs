use super::affine_utils::j::j_per_triangle;
use super::affine_utils::mass::mass_matrix_per_triangle;
use super::types::{Mat2x6, Mat6x2, Mat6x6, Vec6};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub struct Polygon {
    /// Stored in order:
    /// edges are: (0, 1), (1, 2), ..., (n-1, 0)
    pub n: usize,
    pub nodes: Vec<glm::Vec2>,
    _edges: Vec<(glm::Vec2, glm::Vec2)>,
    _normals: Vec<glm::Vec2>,
    _volume: f32,
    _positive: Vec<bool>,
    _mass: Mat6x6,
    _mass_inv: Mat6x6,
    _j: Mat2x6,
    _density: f32,
}

// privates
impl Polygon {
    fn init(&mut self, density: f32) {
        self.init_edges();
        self.init_normals_positive_volume();
        // dbg!(&self._positive);
        self.init_mass_matrix(density);
        self.init_j(density);
    }

    fn init_edges(&mut self) {
        let mut edges = Vec::new();
        for i in 0..self.nodes.len() - 1 {
            edges.push((self.nodes[i].clone(), self.nodes[i + 1].clone()));
        }
        edges.push((*self.nodes.last().unwrap(), *self.nodes.first().unwrap()));
        self._edges = edges;
    }

    fn init_normals_positive_volume(&mut self) {
        let mut normals = Vec::new();
        // first edge
        let e0 = self._edges[0].1 - self._edges[0].0;
        let n0 = glm::vec2(e0.y, -e0.x).normalize();
        normals.push(n0);

        // compute the rest
        let mut nprev = n0;
        for i in 1..self._edges.len() {
            let ecur = self._edges[i].1 - self._edges[i].0;
            let mut ncur = glm::vec2(ecur.y, -ecur.x).normalize();
            // direction align
            let (p1, p2, p3) = (self._edges[i - 1].0, self._edges[i].0, self._edges[i].1);
            let tmp = (p1 + p3) / 2f32 - p2;
            if tmp.dot(&nprev).signum() != tmp.dot(&ncur).signum() {
                ncur *= -1f32;
            }
            normals.push(ncur);
            nprev = ncur;
        }
        self._normals = normals;

        // compute volume, correct volume
        let v = self.init_positive_volume();
        if v < 0f32 {
            // revert all normals
            for n in &mut self._normals {
                *n *= -1f32;
            }
            // revert positive marker
            for p in &mut self._positive {
                *p = !(*p);
            }
        }

        self._volume = v.abs();
        println!("The volume of polygon is {}", self._volume);
    }

    fn init_positive_volume(&mut self) -> f32 {
        let mut v = 0f32;
        self._positive.resize(self._edges.len(), false);
        for (i, (edge, normal)) in self._edges.iter().zip(self._normals.iter()).enumerate() {
            let v_triangle = (edge.0.x * edge.1.y - edge.1.x * edge.0.y).abs() * 0.5f32;
            // dbg!(v_triangle);
            if edge.0.dot(normal) >= 0f32 {
                // positive contribution
                self._positive[i] = true;
                v += v_triangle;
            } else {
                // negative contribution
                self._positive[i] = false;
                v -= v_triangle;
            }
        }
        v
    }

    fn init_mass_matrix(&mut self, density: f32) {
        let mut res = Mat6x6::zeros();
        for (edge, positive) in self._edges.iter().zip(self._positive.iter()) {
            let mat = density * mass_matrix_per_triangle(edge.0, edge.1);
            // dbg!(*positive);
            res += if *positive { mat } else { -mat };
        }
        self._mass = res;
        self._mass_inv = res.try_inverse().expect("Mass matrix is not invertible!");
    }

    fn init_j(&mut self, density: f32) {
        let mut res = Mat2x6::zeros();
        for (edge, positive) in self._edges.iter().zip(self._positive.iter()) {
            let mat = density * j_per_triangle(edge.0, edge.1);

            res += if *positive { mat } else { -mat };
        }
        // dbg!(res);
        self._j = res;
    }
}

impl Polygon {
    pub fn new(nodes: Vec<glm::Vec2>, density: f32) -> Self {
        assert!(
            nodes.len() >= 3,
            "Provide at least 3 nodes to make a polygon!"
        );

        let mut res = Self {
            n: nodes.len(),
            nodes,
            _edges: Vec::new(),
            _normals: Vec::new(),
            _positive: Vec::new(),
            _volume: 0f32,
            _mass: Mat6x6::zeros(),
            _mass_inv: Mat6x6::zeros(),
            _j: Mat2x6::zeros(),
            _density: density,
        };

        res.init(density);

        res
    }

    pub fn edges(&self, i: usize) -> (glm::Vec2, glm::Vec2) {
        self._edges[i]
    }

    pub fn volume(&self) -> f32 {
        self._volume
    }

    pub fn mass_matrix(&self) -> Mat6x6 {
        self._mass
    }

    pub fn mass_inv(&self) -> Mat6x6 {
        self._mass_inv
    }

    pub fn j(&self) -> Mat2x6 {
        self._j
    }

    pub fn jt(&self) -> Mat6x2 {
        self._j.transpose()
    }

    /// Turn per-point accel into force applied to affine body
    pub fn uniform_accel(&self, a: glm::Vec2) -> Vec6 {
        self._density * self.jt() * a
    }
}
