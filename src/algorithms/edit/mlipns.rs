use crate::{Similarity, Distance};
use super::hamming::Hamming;

pub struct MLIPNS {
    pub threshold: f64,
    pub max_mismatches: usize,
}

impl MLIPNS {
    pub fn new() -> Self {
        Self {
            threshold: 0.25,
            max_mismatches: 2,
        }
    }
}

impl Default for MLIPNS {
    fn default() -> Self {
        Self::new()
    }
}

impl Similarity for MLIPNS {
    fn similarity(&self, s1: &[char], s2: &[char]) -> f64 {
        mlipns_impl(s1, s2, self.threshold, self.max_mismatches)
    }

    fn maximum(&self, _s1: &[char], _s2: &[char]) -> f64 {
        1.0
    }
}

fn mlipns_impl(s1: &[char], s2: &[char], threshold: f64, max_mismatches: usize) -> f64 {
    let maxlen = std::cmp::max(s1.len(), s2.len());
    if maxlen == 0 {
        return 1.0;
    }

    // If either sequence is empty, return 0 (matches Python behavior)
    if s1.is_empty() || s2.is_empty() {
        return 0.0;
    }

    let ham = Hamming.distance(s1, s2) as usize;
    let mut ham = ham;
    let mut mismatches = 0;
    let mut len = maxlen;

    while len > 0 && mismatches <= max_mismatches {
        if 1.0 - (len as f64 - ham as f64) / len as f64 <= threshold {
            return 1.0;
        }
        mismatches += 1;
        ham = ham.saturating_sub(1);
        len -= 1;
    }

    if len == 0 {
        1.0
    } else {
        0.0
    }
}

pub fn similarity_str(s1: &str, s2: &str) -> f64 {
    let c1: Vec<char> = s1.chars().collect();
    let c2: Vec<char> = s2.chars().collect();
    MLIPNS::new().similarity(&c1, &c2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mlipns() {
        let alg = MLIPNS::new();
        let test = |a: &str, b: &str| {
            let c1: Vec<char> = a.chars().collect();
            let c2: Vec<char> = b.chars().collect();
            alg.similarity(&c1, &c2)
        };
        assert_eq!(test("", ""), 1.0);
        assert_eq!(test("a", ""), 0.0);
        assert_eq!(test("", "a"), 0.0);
        assert_eq!(test("a", "a"), 1.0);
        assert_eq!(test("ab", "a"), 1.0);
        assert_eq!(test("abc", "abc"), 1.0);
        assert_eq!(test("abc", "abcde"), 1.0);
        assert_eq!(test("Tomato", "Tamato"), 1.0);
    }
}
