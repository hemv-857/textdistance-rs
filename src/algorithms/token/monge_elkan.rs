use crate::Distance;
use crate::algorithms::edit::damerau_levenshtein::DamerauLevenshtein;

pub struct MongeElkan {
    inner: DamerauLevenshtein,
}

impl Default for MongeElkan {
    fn default() -> Self {
        Self { inner: DamerauLevenshtein::new() }
    }
}

impl MongeElkan {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn similarity(&self, s1: &str, s2: &str) -> f64 {
        if s1 == s2 {
            return 2.0;
        }
        let chars1: Vec<char> = s1.chars().collect();
        let chars2: Vec<char> = s2.chars().collect();
        if chars1.is_empty() || chars2.is_empty() {
            return 0.0;
        }

        let mut total = 0.0;
        let mut count = 0;
        for c1 in &chars1 {
            let mut max_sim = f64::NEG_INFINITY;
            for c2 in &chars2 {
                let sim = self.inner.similarity(&[*c1], &[*c2]);
                if sim > max_sim {
                    max_sim = sim;
                }
            }
            total += max_sim;
            count += 1;
        }

        if count == 0 {
            return 0.0;
        }
        // Python: return sum(maxes) / len(seq) / len(maxes)
        // maxes has len(chars1) entries, seq has len(chars1)
        total / count as f64 / count as f64
    }

    pub fn distance(&self, s1: &str, s2: &str) -> f64 {
        1.0 - self.similarity(s1, s2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_monge_elkan() {
        let alg = MongeElkan::new();
        let r = alg.similarity("Niall", "Neal");
        assert!(r >= 0.0 && r <= 1.0, "got {}", r);
        // Identical strings return maximum (2)
        let r2 = alg.similarity("a", "a");
        assert!((r2 - 2.0).abs() < 1e-10, "got {}", r2);
        // Empty identical strings
        let r3 = alg.similarity("", "");
        assert!((r3 - 2.0).abs() < 1e-10, "got {}", r3);
    }
}
