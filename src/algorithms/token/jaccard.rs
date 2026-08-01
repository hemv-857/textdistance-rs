use super::bag::{char_counts, intersect_counters, union_counters, count_sum};

pub struct Jaccard;

impl Default for Jaccard {
    fn default() -> Self {
        Self
    }
}

impl Jaccard {
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
        let union = union_counters(&c1, &c2);
        let union_size = count_sum(&union);
        if union_size == 0 {
            0.0
        } else {
            count_sum(&inter) as f64 / union_size as f64
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
    fn test_jaccard() {
        let alg = Jaccard::new();
        let r = alg.similarity("test", "text");
        assert!((r - 3.0 / 5.0).abs() < 1e-10);

        let r = alg.similarity("nelson", "neilsen");
        assert!((r - 5.0 / 8.0).abs() < 1e-10);

        let r = alg.similarity("decide", "resize");
        assert!((r - 3.0 / 9.0).abs() < 1e-10);
    }
}
