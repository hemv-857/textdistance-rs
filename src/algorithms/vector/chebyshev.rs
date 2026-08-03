/// Chebyshev distance (L-infinity norm)
///
/// Computes the maximum absolute difference between vector elements.
/// Also known as Maximum distance or Chessboard distance.
///
/// Formula: max(|a_i - b_i|)
use crate::VectorDistance;

pub struct Chebyshev;

impl VectorDistance for Chebyshev {
    fn vector_distance(&self, v1: &[f64], v2: &[f64]) -> f64 {
        if v1.len() != v2.len() {
            panic!("Vectors must have the same length");
        }
        v1.iter()
            .zip(v2.iter())
            .map(|(a, b)| (a - b).abs())
            .fold(0.0_f64, f64::max)
    }

    fn vector_maximum(&self, v1: &[f64], v2: &[f64]) -> f64 {
        v1.iter().chain(v2.iter()).fold(0.0_f64, |a, b| a.max(*b))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chebyshev_basic() {
        let alg = Chebyshev;
        let v1 = vec![1.0, 2.0, 3.0];
        let v2 = vec![4.0, 5.0, 6.0];
        assert_eq!(alg.vector_distance(&v1, &v2), 3.0);
    }

    #[test]
    fn test_chebyshev_identical() {
        let alg = Chebyshev;
        let v1 = vec![1.0, 2.0, 3.0];
        assert_eq!(alg.vector_distance(&v1, &v1), 0.0);
    }

    #[test]
    fn test_chebyshev_single() {
        let alg = Chebyshev;
        assert_eq!(alg.vector_distance(&[1.0], &[5.0]), 4.0);
    }
}
