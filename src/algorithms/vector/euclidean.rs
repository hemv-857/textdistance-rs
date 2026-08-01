/// Euclidean distance (L2 norm)
///
/// Computes the square root of the sum of squared differences.
/// This is the "straight line" distance between two points in Euclidean space.
///
/// Formula: sqrt(Σ(a_i - b_i)²)
use crate::VectorDistance;

pub struct Euclidean {
    pub squared: bool,
}

impl Euclidean {
    pub fn new() -> Self {
        Self { squared: false }
    }

    pub fn squared() -> Self {
        Self { squared: true }
    }
}

impl Default for Euclidean {
    fn default() -> Self {
        Self::new()
    }
}

impl VectorDistance for Euclidean {
    fn vector_distance(&self, v1: &[f64], v2: &[f64]) -> f64 {
        if v1.len() != v2.len() {
            panic!("Vectors must have the same length");
        }
        let sum_sq: f64 = v1.iter().zip(v2.iter()).map(|(a, b)| (a - b).powi(2)).sum();
        if self.squared {
            sum_sq
        } else {
            sum_sq.sqrt()
        }
    }

    fn vector_maximum(&self, v1: &[f64], v2: &[f64]) -> f64 {
        let max_val = v1.iter().chain(v2.iter()).fold(0.0_f64, |a, b| a.max(*b));
        max_val * (v1.len() as f64).sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_euclidean_basic() {
        let alg = Euclidean::new();
        let v1 = vec![0.0, 0.0];
        let v2 = vec![3.0, 4.0];
        assert!((alg.vector_distance(&v1, &v2) - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_euclidean_squared() {
        let alg = Euclidean::squared();
        let v1 = vec![0.0, 0.0];
        let v2 = vec![3.0, 4.0];
        assert!((alg.vector_distance(&v1, &v2) - 25.0).abs() < 1e-10);
    }

    #[test]
    fn test_euclidean_identical() {
        let alg = Euclidean::new();
        let v1 = vec![1.0, 2.0, 3.0];
        assert!((alg.vector_distance(&v1, &v1) - 0.0).abs() < 1e-10);
    }
}
