pub struct Length;

impl Default for Length {
    fn default() -> Self {
        Self
    }
}

impl Length {
    pub fn new() -> Self {
        Self
    }

    pub fn distance(&self, s1: &str, s2: &str) -> f64 {
        (s1.chars().count() as i64 - s2.chars().count() as i64).unsigned_abs() as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length() {
        let alg = Length::new();
        assert_eq!(alg.distance("abc", "abc"), 0.0);
        assert_eq!(alg.distance("abc", "ab"), 1.0);
        assert_eq!(alg.distance("ab", "abcd"), 2.0);
    }
}
