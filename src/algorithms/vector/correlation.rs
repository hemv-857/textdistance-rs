/// Correlation distance
///
/// Computes the correlation-based distance between two vectors.
/// Correlation distance is defined as 1 - correlation coefficient.
///
/// Formula: 1 - (Σ(a_i - mean_a)(b_i - mean_b)) / (std_a * std_b * n)
use crate::VectorDistance;

pub struct Correlation;

impl VectorDistance for Correlation {
    fn vector_distance(&self, v1: &[f64], v2: &[f64]) -> f64 {
        if v1.len() != v2.len() {
            panic!("Vectors must have the same length");
        }

        let n = v1.len() as f64;
        if n == 0.0 {
            return 0.0;
        }

        let mean1: f64 = v1.iter().sum::<f64>() / n;
        let mean2: f64 = v2.iter().sum::<f64>() / n;

        let centered1: Vec<f64> = v1.iter().map(|x| x - mean1).collect();
        let centered2: Vec<f64> = v2.iter().map(|x| x - mean2).collect();

        let dot_product: f64 = centered1
            .iter()
            .zip(centered2.iter())
            .map(|(a, b)| a * b)
            .sum();

        let norm1: f64 = centered1.iter().map(|x| x * x).sum::<f64>().sqrt();
        let norm2: f64 = centered2.iter().map(|x| x * x).sum::<f64>().sqrt();

        if norm1 == 0.0 || norm2 == 0.0 {
            return 1.0; // Undefined correlation, return maximum distance
        }

        let correlation = dot_product / (norm1 * norm2);

        // Clamp to [-1, 1] to handle floating point errors
        let correlation = correlation.clamp(-1.0, 1.0);

        // Distance = 1 - correlation
        1.0 - correlation
    }

    fn vector_maximum(&self, _v1: &[f64], _v2: &[f64]) -> f64 {
        2.0 // Max distance is 2.0 (perfectly anti-correlated)
    }
}

impl Correlation {
    /// Compute correlation coefficient (similarity)
    pub fn correlation_coefficient(&self, v1: &[f64], v2: &[f64]) -> f64 {
        1.0 - self.vector_distance(v1, v2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_correlation_identical() {
        let alg = Correlation;
        let v1 = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        assert!((alg.vector_distance(&v1, &v1) - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_correlation_perfect_positive() {
        let alg = Correlation;
        let v1 = vec![1.0, 2.0, 3.0];
        let v2 = vec![2.0, 4.0, 6.0]; // Perfectly correlated
        assert!((alg.vector_distance(&v1, &v2) - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_correlation_perfect_negative() {
        let alg = Correlation;
        let v1 = vec![1.0, 2.0, 3.0];
        let v2 = vec![3.0, 2.0, 1.0]; // Perfectly anti-correlated
        assert!((alg.vector_distance(&v1, &v2) - 2.0).abs() < 1e-10);
    }

    #[test]
    fn test_correlation_orthogonal() {
        let alg = Correlation;
        let v1 = vec![1.0, 0.0, 0.0];
        let v2 = vec![0.0, 1.0, 0.0];
        // Mean-subjected: [0.667, -0.333, -0.333] vs [-0.333, 0.667, -0.333]
        // Correlation = -0.5, distance = 1.5
        let dist = alg.vector_distance(&v1, &v2);
        assert!((dist - 1.5).abs() < 0.01);
    }
}
