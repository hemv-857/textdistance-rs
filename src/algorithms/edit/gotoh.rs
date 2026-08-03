use crate::Similarity;

pub struct Gotoh {
    pub gap_open: f64,
    pub gap_ext: f64,
}

impl Gotoh {
    pub fn new() -> Self {
        Self {
            gap_open: 1.0,
            gap_ext: 0.4,
        }
    }

    pub fn with_costs(gap_open: f64, gap_ext: f64) -> Self {
        Self { gap_open, gap_ext }
    }
}

impl Default for Gotoh {
    fn default() -> Self {
        Self::new()
    }
}

impl Similarity for Gotoh {
    fn similarity(&self, s1: &[char], s2: &[char]) -> f64 {
        gotoh_impl(s1, s2, self.gap_open, self.gap_ext)
    }

    fn maximum(&self, s1: &[char], s2: &[char]) -> f64 {
        std::cmp::min(s1.len(), s2.len()) as f64
    }
}

fn gotoh_impl(s1: &[char], s2: &[char], gap_open: f64, gap_ext: f64) -> f64 {
    let len1 = s1.len();
    let len2 = s2.len();
    let rows = len1 + 1;
    let cols = len2 + 1;
    let neg_inf = f64::NEG_INFINITY;

    let mut d_mat = vec![vec![0.0f64; cols]; rows];
    let mut p_mat = vec![vec![0.0f64; cols]; rows];
    let mut q_mat = vec![vec![0.0f64; cols]; rows];

    d_mat[0][0] = 0.0;
    p_mat[0][0] = neg_inf;
    q_mat[0][0] = neg_inf;

    for i in 1..rows {
        d_mat[i][0] = neg_inf;
        p_mat[i][0] = -gap_open - gap_ext * (i as f64 - 1.0);
        q_mat[i][0] = neg_inf;
    }
    for j in 1..cols {
        d_mat[0][j] = neg_inf;
        p_mat[0][j] = neg_inf;
        q_mat[0][j] = -gap_open - gap_ext * (j as f64 - 1.0);
    }

    // Fix p_mat[1][*] and q_mat[*][1]
    if cols > 1 {
        p_mat[0][1] = -gap_open;
    }
    if rows > 1 {
        q_mat[1][0] = -gap_open;
    }

    for i in 1..rows {
        for j in 1..cols {
            let sim_val = if s1[i - 1] == s2[j - 1] { 1.0 } else { 0.0 };

            d_mat[i][j] = (d_mat[i - 1][j - 1] + sim_val)
                .max(p_mat[i - 1][j - 1] + sim_val)
                .max(q_mat[i - 1][j - 1] + sim_val);

            p_mat[i][j] = (d_mat[i - 1][j] - gap_open).max(p_mat[i - 1][j] - gap_ext);

            q_mat[i][j] = (d_mat[i][j - 1] - gap_open).max(q_mat[i][j - 1] - gap_ext);
        }
    }

    d_mat[len1][len2]
        .max(p_mat[len1][len2])
        .max(q_mat[len1][len2])
}

pub fn similarity_str(s1: &str, s2: &str) -> f64 {
    let c1: Vec<char> = s1.chars().collect();
    let c2: Vec<char> = s2.chars().collect();
    Gotoh::new().similarity(&c1, &c2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gotoh() {
        let alg = Gotoh::new();
        let _eps = 1e-6;
        let test = |a: &str, b: &str| {
            let c1: Vec<char> = a.chars().collect();
            let c2: Vec<char> = b.chars().collect();
            alg.similarity(&c1, &c2)
        };
        // Identical strings
        assert!(test("ABCD", "ABCD") > 0.0);
        // Partial match
        assert!(test("ABCDEFG", "XBCDFG") > 0.0);
    }
}
