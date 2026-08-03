use crate::Similarity;

pub struct SmithWaterman {
    pub gap_cost: f64,
}

impl SmithWaterman {
    pub fn new() -> Self {
        Self { gap_cost: 1.0 }
    }

    pub fn with_gap_cost(gap_cost: f64) -> Self {
        Self { gap_cost }
    }
}

impl Default for SmithWaterman {
    fn default() -> Self {
        Self::new()
    }
}

impl Similarity for SmithWaterman {
    fn similarity(&self, s1: &[char], s2: &[char]) -> f64 {
        sw_impl(s1, s2, self.gap_cost)
    }

    fn maximum(&self, s1: &[char], s2: &[char]) -> f64 {
        std::cmp::min(s1.len(), s2.len()) as f64
    }
}

fn sw_impl(s1: &[char], s2: &[char], gap_cost: f64) -> f64 {
    let len1 = s1.len();
    let len2 = s2.len();
    let rows = len1 + 1;
    let cols = len2 + 1;

    let mut dist_mat = vec![vec![0.0f64; cols]; rows];

    for i in 1..rows {
        for j in 1..cols {
            let sim_val = if s1[i - 1] == s2[j - 1] { 1.0 } else { 0.0 };
            let match_score = dist_mat[i - 1][j - 1] + sim_val;
            let delete_score = dist_mat[i - 1][j] - gap_cost;
            let insert_score = dist_mat[i][j - 1] - gap_cost;
            dist_mat[i][j] = 0.0_f64.max(match_score).max(delete_score).max(insert_score);
        }
    }

    dist_mat[len1][len2]
}

pub fn similarity_str(s1: &str, s2: &str) -> f64 {
    let c1: Vec<char> = s1.chars().collect();
    let c2: Vec<char> = s2.chars().collect();
    SmithWaterman::new().similarity(&c1, &c2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sw() {
        let alg = SmithWaterman::new();
        let eps = 1e-6;
        let test = |a: &str, b: &str| {
            let c1: Vec<char> = a.chars().collect();
            let c2: Vec<char> = b.chars().collect();
            alg.similarity(&c1, &c2)
        };
        // Identical strings
        assert!((test("ABCD", "ABCD") - 4.0).abs() < eps);
        // Partial match
        assert!(test("ABCDEFG", "XBCDFG") > 0.0);
    }
}
