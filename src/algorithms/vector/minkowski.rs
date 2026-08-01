/// Minkowski distance (Lp norm)
///
/// A generalized distance metric that includes Manhattan (p=1) and Euclidean (p=2) as special cases.
///
/// Formula: (Σ|a_i - b_i|^p)^(1/p)

use crate::VectorDistance;

pub struct Minkowski {
    pub p: f64,
    pub weight: f64,
}

impl Minkowski {
    pub fn new(p: f64, weight: f64) -> Self {
        if p < 1.0 {
            panic!("p must be at least 1");
        }
        Self { p, weight }
    }

    pub fn default_p() -> Self {
        Self::new(2.0, 1.0)
    }
}

impl Default for Minkowski {
    fn default() -> Self {
        Self::default_p()
    }
}

impl VectorDistance for Minkowski {
    fn vector_distance(&self, v1: &[f64], v2: &[f64]) -> f64 {
        if v1.len() != v2.len() {
            panic!("Vectors must have the same length");
        }
        let result: f64 = v1
            .iter()
            .zip(v2.iter())
            .map(|(a, b)| (self.weight * (a - b).abs()).powf(self.p))
            .sum();
        result.powf(1.0 / self.p)
    }

    fn vector_maximum(&self, v1: &[f64], v2: &[f64]) -> f64 {
        let max_val = v1.iter().chain(v2.iter()).fold(0.0_f64, |a, b| a.max(*b));
        (self.weight * max_val) * v1.len() as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minkowski_p2_is_euclidean() {
        let alg = Minkowski::new(2.0, 1.0);
        let v1 = vec![0.0, 0.0];
        let v2 = vec![3.0, 4.0];
        assert!((alg.vector_distance(&v1, &v2) - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_minkowski_p1_is_manhattan() {
        let alg = Minkowski::new(1.0, 1.0);
        let v1 = vec![1.0, 2.0, 3.0];
        let v2 = vec![4.0, 5.0, 6.0];
        assert_eq!(alg.vector_distance(&v1, &v2), 9.0);
    }

    #[test]
    fn test_minkowski_identical() {
        let alg = Minkowski::default_p();
        let v1 = vec![1.0, 2.0, 3.0];
        assert!((alg.vector_distance(&v1, &v1) - 0.0).abs() < 1e-10);
    }
}
