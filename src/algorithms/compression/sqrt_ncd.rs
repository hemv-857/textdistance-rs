use std::collections::HashMap;

fn char_counts(s: &str) -> HashMap<char, usize> {
    let mut counts = HashMap::new();
    for ch in s.chars() {
        *counts.entry(ch).or_insert(0) += 1;
    }
    counts
}

pub struct SqrtNCD;

impl Default for SqrtNCD {
    fn default() -> Self {
        Self
    }
}

impl SqrtNCD {
    pub fn new() -> Self {
        Self
    }

    fn compress_size(&self, data: &str) -> f64 {
        let counts = char_counts(data);
        counts.values().map(|&c| (c as f64).sqrt()).sum()
    }

    fn ncd(&self, s1: &str, s2: &str) -> f64 {
        let c1 = self.compress_size(s1);
        let c2 = self.compress_size(s2);
        let concat_min = self
            .compress_size(&format!("{}{}", s1, s2))
            .min(self.compress_size(&format!("{}{}", s2, s1)));
        let min_compressed = c1.min(c2);
        let max_compressed = c1.max(c2);
        if max_compressed == 0.0 {
            return 0.0;
        }
        (concat_min - min_compressed) / max_compressed
    }

    pub fn distance(&self, s1: &str, s2: &str) -> f64 {
        self.ncd(s1, s2)
    }

    pub fn similarity(&self, s1: &str, s2: &str) -> f64 {
        1.0 - self.ncd(s1, s2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sqrt_ncd() {
        let alg = SqrtNCD::new();
        assert!((alg.distance("test", "test") - 0.41421356237309503).abs() < 1e-10);
        assert!((alg.distance("test", "nani") - 1.0).abs() < 1e-10);
    }
}
