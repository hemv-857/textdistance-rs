use super::bag::{char_counts, count_sum, intersect_counters};

pub struct Cosine;

impl Default for Cosine {
    fn default() -> Self {
        Self
    }
}

impl Cosine {
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
        let inter_count = count_sum(&inter) as f64;
        let count1 = count_sum(&c1) as f64;
        let count2 = count_sum(&c2) as f64;
        let prod = count1 * count2;
        if prod == 0.0 {
            0.0
        } else {
            inter_count / prod.sqrt()
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
    fn test_cosine() {
        let alg = Cosine::new();
        let r = alg.similarity("test", "text");
        assert!((r - 3.0 / 4.0).abs() < 1e-10);

        let r = alg.similarity("nelson", "neilsen");
        let expected = 5.0 / (6.0 * 7.0_f64).sqrt();
        assert!((r - expected).abs() < 1e-10);
    }
}
