pub struct Identity;

impl Default for Identity {
    fn default() -> Self {
        Self
    }
}

impl Identity {
    pub fn new() -> Self {
        Self
    }

    pub fn similarity(&self, s1: &str, s2: &str) -> f64 {
        if s1 == s2 {
            1.0
        } else {
            0.0
        }
    }

    pub fn distance(&self, s1: &str, s2: &str) -> f64 {
        if s1 == s2 {
            0.0
        } else {
            1.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identity() {
        let alg = Identity::new();
        assert_eq!(alg.similarity("abc", "abc"), 1.0);
        assert_eq!(alg.similarity("abc", "xyz"), 0.0);
        assert_eq!(alg.distance("abc", "abc"), 0.0);
        assert_eq!(alg.distance("abc", "xyz"), 1.0);
    }
}
