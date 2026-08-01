use std::collections::HashMap;

pub struct Matrix {
    match_cost: i64,
    mismatch_cost: i64,
    symmetric: bool,
    entries: HashMap<(char, char), i64>,
}

impl Default for Matrix {
    fn default() -> Self {
        Self {
            match_cost: 1,
            mismatch_cost: 0,
            symmetric: true,
            entries: HashMap::new(),
        }
    }
}

impl Matrix {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_entries(
        match_cost: i64,
        mismatch_cost: i64,
        symmetric: bool,
        entries: HashMap<(char, char), i64>,
    ) -> Self {
        Self {
            match_cost,
            mismatch_cost,
            symmetric,
            entries,
        }
    }

    pub fn similarity(&self, s1: &str, s2: &str) -> f64 {
        let c1: Vec<char> = s1.chars().collect();
        let c2: Vec<char> = s2.chars().collect();
        if c1.is_empty() && c2.is_empty() {
            return self.match_cost as f64;
        }
        if c1.is_empty() || c2.is_empty() {
            return self.mismatch_cost as f64;
        }
        if self.entries.is_empty() {
            if c1 == c2 {
                return self.match_cost as f64;
            }
            return self.mismatch_cost as f64;
        }

        let key = (c1[0], c2[0]);
        if let Some(&cost) = self.entries.get(&key) {
            return cost as f64;
        }
        if self.symmetric {
            let rkey = (c2[0], c1[0]);
            if let Some(&cost) = self.entries.get(&rkey) {
                return cost as f64;
            }
        }
        if c1 == c2 {
            self.match_cost as f64
        } else {
            self.mismatch_cost as f64
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_default() {
        let alg = Matrix::new();
        assert_eq!(alg.similarity("a", "a"), 1.0);
        assert_eq!(alg.similarity("a", "b"), 0.0);
    }

    #[test]
    fn test_matrix_custom() {
        let mut entries = HashMap::new();
        entries.insert(('A', 'A'), 2);
        entries.insert(('B', 'B'), 3);
        let alg = Matrix::with_entries(1, 0, true, entries);
        assert_eq!(alg.similarity("A", "A"), 2.0);
        assert_eq!(alg.similarity("B", "B"), 3.0);
        assert_eq!(alg.similarity("A", "B"), 0.0);
    }
}
