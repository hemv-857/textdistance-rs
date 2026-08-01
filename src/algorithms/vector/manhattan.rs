/// Manhattan distance (L1 norm)
///
/// Computes the sum of absolute differences between vector elements.
/// Also known as City Block or Taxicab distance.
///
/// Formula: Σ|a_i - b_i|

use crate::VectorDistance;

pub struct Manhattan;

impl VectorDistance for Manhattan {
    fn vector_distance(&self, v1: &[f64], v2: &[f64]) -> f64 {
        if v1.len() != v2.len() {
            panic!("Vectors must have the same length");
        }
        v1.iter()
            .zip(v2.iter())
            .map(|(a, b)| (a - b).abs())
            .sum()
    }

    fn vector_maximum(&self, v1: &[f64], v2: &[f64]) -> f64 {
        let max_val = v1.iter().chain(v2.iter()).fold(0.0_f64, |a, b| a.max(*b));
        max_val * v1.len() as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manhattan_basic() {
        let alg = Manhattan;
        let v1 = vec![1.0, 2.0, 3.0];
        let v2 = vec![4.0, 5.0, 6.0];
        assert_eq!(alg.vector_distance(&v1, &v2), 9.0);
    }

    #[test]
    fn test_manhattan_identical() {
        let alg = Manhattan;
        let v1 = vec![1.0, 2.0, 3.0];
        assert_eq!(alg.vector_distance(&v1, &v1), 0.0);
    }

    #[test]
    fn test_manhattan_single() {
        let alg = Manhattan;
        assert_eq!(alg.vector_distance(&[1.0], &[5.0]), 4.0);
    }
}
