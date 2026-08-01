use crate::Similarity;

pub struct NeedlemanWunsch {
    pub gap_cost: f64,
}

impl NeedlemanWunsch {
    pub fn new() -> Self {
        Self { gap_cost: 1.0 }
    }

    pub fn with_gap_cost(gap_cost: f64) -> Self {
        Self { gap_cost }
    }
}

impl Default for NeedlemanWunsch {
    fn default() -> Self {
        Self::new()
    }
}

impl Similarity for NeedlemanWunsch {
    fn similarity(&self, s1: &[char], s2: &[char]) -> f64 {
        nw_impl(s1, s2, self.gap_cost)
    }

    fn maximum(&self, s1: &[char], s2: &[char]) -> f64 {
        std::cmp::max(s1.len(), s2.len()) as f64
    }
}

fn nw_impl(s1: &[char], s2: &[char], gap_cost: f64) -> f64 {
    let len1 = s1.len();
    let len2 = s2.len();
    let rows = len1 + 1;
    let cols = len2 + 1;

    let mut dist_mat = vec![vec![0.0f64; cols]; rows];

    // Initialize first column
    for i in 0..rows {
        dist_mat[i][0] = -(i as f64) * gap_cost;
    }
    // Initialize first row
    for j in 0..cols {
        dist_mat[0][j] = -(j as f64) * gap_cost;
    }

    // Fill matrix
    for i in 1..rows {
        for j in 1..cols {
            let sim_val = if s1[i - 1] == s2[j - 1] { 1.0 } else { 0.0 };
            let match_score = dist_mat[i - 1][j - 1] + sim_val;
            let delete_score = dist_mat[i - 1][j] - gap_cost;
            let insert_score = dist_mat[i][j - 1] - gap_cost;
            dist_mat[i][j] = match_score.max(delete_score).max(insert_score);
        }
    }

    dist_mat[len1][len2]
}

pub fn similarity_str(s1: &str, s2: &str) -> f64 {
    let c1: Vec<char> = s1.chars().collect();
    let c2: Vec<char> = s2.chars().collect();
    NeedlemanWunsch::new().similarity(&c1, &c2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nw() {
        let alg = NeedlemanWunsch::new();
        let eps = 1e-6;
        let test = |a: &str, b: &str| {
            let c1: Vec<char> = a.chars().collect();
            let c2: Vec<char> = b.chars().collect();
            alg.similarity(&c1, &c2)
        };
        // Identical strings
        assert!((test("ABCD", "ABCD") - 4.0).abs() < eps);
        // Completely different
        assert!((test("AAAA", "BBBB") - 0.0).abs() < eps);
    }
}
