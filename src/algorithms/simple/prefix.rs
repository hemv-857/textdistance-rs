pub struct Prefix;

impl Default for Prefix {
    fn default() -> Self {
        Self
    }
}

impl Prefix {
    pub fn new() -> Self {
        Self
    }

    pub fn prefix_len(&self, s1: &str, s2: &str) -> usize {
        let chars1: Vec<char> = s1.chars().collect();
        let chars2: Vec<char> = s2.chars().collect();
        let mut count = 0;
        for (c1, c2) in chars1.iter().zip(chars2.iter()) {
            if c1 == c2 {
                count += 1;
            } else {
                break;
            }
        }
        count
    }

    pub fn prefix_str(&self, s1: &str, s2: &str) -> String {
        s1.chars().take(self.prefix_len(s1, s2)).collect()
    }

    pub fn similarity(&self, s1: &str, s2: &str) -> f64 {
        self.prefix_len(s1, s2) as f64
    }

    pub fn distance(&self, s1: &str, s2: &str) -> f64 {
        let max_len = s1.chars().count().max(s2.chars().count());
        max_len as f64 - self.similarity(s1, s2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prefix() {
        let alg = Prefix::new();
        assert_eq!(alg.prefix_len("abcdef", "abcxyz"), 3);
        assert_eq!(alg.prefix_len("abc", "abc"), 3);
        assert_eq!(alg.prefix_len("abc", "xyz"), 0);
    }
}
