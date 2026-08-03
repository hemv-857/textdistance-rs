use super::bag::{char_counts, count_sum, intersect_counters};

pub struct Overlap;

impl Default for Overlap {
    fn default() -> Self {
        Self
    }
}

impl Overlap {
    pub fn new() -> Self {
        Self
    }

    pub fn similarity(&self, s1: &str, s2: &str) -> f64 {
        let c1 = char_counts(s1);
        let c2 = char_counts(s2);
        if c1.is_empty() && c2.is_empty() {
            return 1.0;
        }
        let inter = intersect_counters(&c1, &c2);
        let inter_count = count_sum(&inter);
        let min_count = count_sum(&c1).min(count_sum(&c2));
        if min_count == 0 {
            0.0
        } else {
            inter_count as f64 / min_count as f64
        }
    }

    pub fn distance(&self, s1: &str, s2: &str) -> f64 {
        1.0 - self.similarity(s1, s2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_overlap() {
        let alg = Overlap::new();
        let r = alg.similarity("test", "text");
        assert!((r - 3.0 / 4.0).abs() < 1e-10);

        let r = alg.similarity("testme", "textthis");
        assert!((r - 4.0 / 6.0).abs() < 1e-10);

        let r = alg.similarity("nelson", "neilsen");
        assert!((r - 5.0 / 6.0).abs() < 1e-10);
    }
}
