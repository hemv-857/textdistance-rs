/// Mahalanobis distance
///
/// Measures the distance between a point and a distribution,
/// or between two points, accounting for correlations between variables.
///
/// Formula: sqrt((v1 - v2)^T * S^-1 * (v1 - v2))
/// where S is the covariance matrix.
///
/// Falls back to Euclidean if no inverse covariance matrix is provided.

use crate::VectorDistance;

pub struct Mahalanobis {
    /// Inverse of the covariance matrix
    pub inverse_covariance: Option<Vec<Vec<f64>>>,
}

impl Mahalanobis {
    pub fn new() -> Self {
        Self {
            inverse_covariance: None,
        }
    }

    pub fn with_inverse_covariance(inverse_cov: Vec<Vec<f64>>) -> Self {
        Self {
            inverse_covariance: Some(inverse_cov),
        }
    }
}

impl Default for Mahalanobis {
    fn default() -> Self {
        Self::new()
    }
}

impl VectorDistance for Mahalanobis {
    fn vector_distance(&self, v1: &[f64], v2: &[f64]) -> f64 {
        if v1.len() != v2.len() {
            panic!("Vectors must have the same length");
        }

        let diff: Vec<f64> = v1.iter().zip(v2.iter()).map(|(a, b)| a - b).collect();

        match &self.inverse_covariance {
            Some(inv_cov) => {
                // (v1 - v2)^T * S^-1 * (v1 - v2)
                let temp = mat_vec_mul(inv_cov, &diff);
                let result: f64 = diff.iter().zip(temp.iter()).map(|(a, b)| a * b).sum();
                result.sqrt()
            }
            None => {
                // Fallback to Euclidean if no covariance matrix provided
                let sum_sq: f64 = diff.iter().map(|x| x * x).sum();
                sum_sq.sqrt()
            }
        }
    }

    fn vector_maximum(&self, v1: &[f64], v2: &[f64]) -> f64 {
        let max_val = v1.iter().chain(v2.iter()).fold(0.0_f64, |a, b| a.max(*b));
        max_val * (v1.len() as f64).sqrt()
    }
}

/// Helper: matrix-vector multiplication
fn mat_vec_mul(mat: &[Vec<f64>], vec: &[f64]) -> Vec<f64> {
    mat.iter()
        .map(|row| row.iter().zip(vec.iter()).map(|(a, b)| a * b).sum())
        .collect()
}

/// Compute covariance matrix from a set of vectors
pub fn covariance_matrix(vectors: &[Vec<f64>]) -> Option<Vec<Vec<f64>>> {
    if vectors.is_empty() {
        return None;
    }

    let n = vectors.len();
    let dim = vectors[0].len();

    // Compute mean
    let mut mean = vec![0.0; dim];
    for v in vectors {
        for (i, &val) in v.iter().enumerate() {
            mean[i] += val;
        }
    }
    for m in &mut mean {
        *m /= n as f64;
    }

    // Compute covariance matrix
    let mut cov = vec![vec![0.0; dim]; dim];
    for v in vectors {
        for i in 0..dim {
            for j in 0..dim {
                cov[i][j] += (v[i] - mean[i]) * (v[j] - mean[j]);
            }
        }
    }
    for i in 0..dim {
        for j in 0..dim {
            cov[i][j] /= (n - 1) as f64;
        }
    }

    Some(cov)
}

/// Compute inverse of a matrix using Gaussian elimination
pub fn matrix_inverse(mat: &[Vec<f64>]) -> Option<Vec<Vec<f64>>> {
    let n = mat.len();
    if n == 0 || mat[0].len() != n {
        return None;
    }

    // Create augmented matrix [A | I]
    let mut aug: Vec<Vec<f64>> = (0..n)
        .map(|i| {
            let mut row = mat[i].clone();
            for j in 0..n {
                row.push(if i == j { 1.0 } else { 0.0 });
            }
            row
        })
        .collect();

    // Forward elimination
    for col in 0..n {
        // Find pivot
        let mut max_row = col;
        for row in col..n {
            if aug[row][col].abs() > aug[max_row][col].abs() {
                max_row = row;
            }
        }
        aug.swap(col, max_row);

        if aug[col][col].abs() < 1e-10 {
            return None; // Singular matrix
        }

        // Scale pivot row
        let pivot = aug[col][col];
        for j in 0..2 * n {
            aug[col][j] /= pivot;
        }

        // Eliminate column
        for row in 0..n {
            if row != col {
                let factor = aug[row][col];
                for j in 0..2 * n {
                    aug[row][j] -= factor * aug[col][j];
                }
            }
        }
    }

    // Extract inverse matrix
    let inv: Vec<Vec<f64>> = (0..n).map(|i| aug[i][n..].to_vec()).collect();

    Some(inv)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mahalanobis_euclidean_fallback() {
        let alg = Mahalanobis::new();
        let v1 = vec![0.0, 0.0];
        let v2 = vec![3.0, 4.0];
        assert!((alg.vector_distance(&v1, &v2) - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_mahalanobis_with_covariance() {
        // Identity covariance matrix should give Euclidean distance
        let inv_cov = vec![vec![1.0, 0.0], vec![0.0, 1.0]];
        let alg = Mahalanobis::with_inverse_covariance(inv_cov);
        let v1 = vec![0.0, 0.0];
        let v2 = vec![3.0, 4.0];
        assert!((alg.vector_distance(&v1, &v2) - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_matrix_inverse() {
        let mat = vec![vec![2.0, 1.0], vec![1.0, 3.0]];
        let inv = matrix_inverse(&mat).unwrap();
        // Check A * A^-1 ≈ I by multiplying row by column
        let r0c0: f64 = mat[0].iter().zip(inv.iter().map(|r| r[0])).map(|(a, b)| a * b).sum();
        let r0c1: f64 = mat[0].iter().zip(inv.iter().map(|r| r[1])).map(|(a, b)| a * b).sum();
        let r1c0: f64 = mat[1].iter().zip(inv.iter().map(|r| r[0])).map(|(a, b)| a * b).sum();
        let r1c1: f64 = mat[1].iter().zip(inv.iter().map(|r| r[1])).map(|(a, b)| a * b).sum();
        assert!((r0c0 - 1.0).abs() < 1e-10);
        assert!((r0c1).abs() < 1e-10);
        assert!((r1c0).abs() < 1e-10);
        assert!((r1c1 - 1.0).abs() < 1e-10);
    }
}
