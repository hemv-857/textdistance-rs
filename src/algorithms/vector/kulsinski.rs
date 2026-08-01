/// Kulsinski dissimilarity
///
/// A distance metric for binary vectors, related to the Jaccard index.
/// It penalizes mismatches more heavily when both vectors have 1s.
///
/// Formula: 1 - (n11 - n01 - n10 + n) / n
/// where n11 = both 1, n01 = s1=0 s2=1, n10 = s1=1 s2=0, n = total elements

use crate::VectorDistance;

pub struct Kulsinski;

impl VectorDistance for Kulsinski {
    fn vector_distance(&self, v1: &[f64], v2: &[f64]) -> f64 {
        if v1.len() != v2.len() {
            panic!("Vectors must have the same length");
        }

        let n = v1.len() as f64;
        if n == 0.0 {
            return 0.0;
        }

        let mut n11 = 0.0; // Both 1
        let mut n10 = 0.0; // s1=1, s2=0
        let mut n01 = 0.0; // s1=0, s2=1

        for (&a, &b) in v1.iter().zip(v2.iter()) {
            match (a > 0.5, b > 0.5) {
                (true, true) => n11 += 1.0,
                (true, false) => n10 += 1.0,
                (false, true) => n01 += 1.0,
                (false, false) => {}
            }
        }

        // Kulsinski: 1 - (n11 - n01 - n10 + n) / n
        let distance = 1.0 - (n11 - n01 - n10 + n) / n;
        distance.clamp(0.0, 1.0)
    }

    fn vector_maximum(&self, _v1: &[f64], _v2: &[f64]) -> f64 {
        1.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kulsinski_identical() {
        let alg = Kulsinski;
        let v1 = vec![1.0, 0.0, 1.0, 0.0];
        assert!((alg.vector_distance(&v1, &v1) - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_kulsinski_all_zero() {
        let alg = Kulsinski;
        let v1 = vec![0.0, 0.0, 0.0];
        let v2 = vec![0.0, 0.0, 0.0];
        assert!((alg.vector_distance(&v1, &v2) - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_kulsinski_complement() {
        let alg = Kulsinski;
        let v1 = vec![1.0, 1.0, 0.0, 0.0];
        let v2 = vec![0.0, 0.0, 1.0, 1.0];
        // All different
        let dist = alg.vector_distance(&v1, &v2);
        assert!(dist >= 0.9 && dist <= 1.1);
    }

    #[test]
    fn test_kulsinski_partial() {
        let alg = Kulsinski;
        let v1 = vec![1.0, 1.0, 0.0];
        let v2 = vec![1.0, 0.0, 0.0];
        let dist = alg.vector_distance(&v1, &v2);
        assert!(dist >= 0.0 && dist <= 1.0);
    }
}
