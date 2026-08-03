use super::bag::{char_counts, count_sum, intersect_counters};

pub struct Sorensen;

impl Default for Sorensen {
    fn default() -> Self {
        Self
    }
}

impl Sorensen {
    pub fn new() -> Self {
        Self
    }

    pub fn similarity(&self, s1: &str, s2: &str) -> f64 {
        let c1 = char_counts(s1);
        let c2 = char_counts(s2);
        let inter = intersect_counters(&c1, &c2);
        let inter_count = count_sum(&inter);
        let total = count_sum(&c1) + count_sum(&c2);
        if total == 0 {
            0.0
        } else {
            2.0 * inter_count as f64 / total as f64
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
    fn test_sorensen() {
        let alg = Sorensen::new();
        let r = alg.similarity("test", "text");
        assert!((r - 2.0 * 3.0 / 8.0).abs() < 1e-10);
    }
}
