use std::collections::HashMap;

pub struct EntropyNCD {
    base: f64,
}

impl Default for EntropyNCD {
    fn default() -> Self {
        Self { base: 2.0 }
    }
}

impl EntropyNCD {
    pub fn new() -> Self {
        Self::default()
    }

    fn char_counts(s: &str) -> HashMap<char, usize> {
        let mut counts = HashMap::new();
        for ch in s.chars() {
            *counts.entry(ch).or_insert(0) += 1;
        }
        counts
    }

    pub fn compress_size(&self, data: &str) -> f64 {
        if data.is_empty() {
            return 0.0;
        }
        let counts = Self::char_counts(data);
        let total = data.len() as f64;
        let mut entropy = 0.0;
        for &count in counts.values() {
            let p = count as f64 / total;
            entropy -= p * p.log(self.base);
        }
        1.0 + entropy
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
    fn test_entropy_ncd() {
        let alg = EntropyNCD::new();
        assert!((alg.similarity("test", "test") - 1.0).abs() < 1e-10);
        assert!((alg.similarity("aaa", "bbb") - 0.0).abs() < 1e-10);
        assert!((alg.similarity("test", "nani") - 0.6).abs() < 0.1);
    }
}
