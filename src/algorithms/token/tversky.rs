use super::bag::{char_counts, count_sum, intersect_counters};

pub struct Tversky {
    pub alpha: f64,
    pub beta: f64,
}

impl Default for Tversky {
    fn default() -> Self {
        Self {
            alpha: 1.0,
            beta: 1.0,
        }
    }
}

impl Tversky {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_params(alpha: f64, beta: f64) -> Self {
        Self { alpha, beta }
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

        let a_minus_b = count1 - inter_count;
        let b_minus_a = count2 - inter_count;
        let denom = inter_count + self.alpha * a_minus_b + self.beta * b_minus_a;
        if denom == 0.0 {
            0.0
        } else {
            inter_count / denom
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
    fn test_tversky_as_jaccard() {
        let alg = Tversky::with_params(1.0, 1.0);
        let r = alg.similarity("test", "text");
        assert!((r - 3.0 / 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_tversky_as_sorensen() {
        let alg = Tversky::with_params(0.5, 0.5);
        let r = alg.similarity("test", "text");
        assert!((r - 2.0 * 3.0 / 8.0).abs() < 1e-10);
    }
}
