pub struct Postfix;

impl Default for Postfix {
    fn default() -> Self {
        Self
    }
}

impl Postfix {
    pub fn new() -> Self {
        Self
    }

    pub fn postfix_len(&self, s1: &str, s2: &str) -> usize {
        let chars1: Vec<char> = s1.chars().collect();
        let chars2: Vec<char> = s2.chars().collect();
        let mut count = 0;
        for (c1, c2) in chars1.iter().rev().zip(chars2.iter().rev()) {
            if c1 == c2 {
                count += 1;
            } else {
                break;
            }
        }
        count
    }

    pub fn postfix_str(&self, s1: &str, s2: &str) -> String {
        let len = self.postfix_len(s1, s2);
        let chars: Vec<char> = s1.chars().collect();
        chars[chars.len() - len..].iter().collect()
    }

    pub fn similarity(&self, s1: &str, s2: &str) -> f64 {
        self.postfix_len(s1, s2) as f64
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
    fn test_postfix() {
        let alg = Postfix::new();
        assert_eq!(alg.postfix_len("abcxyz", "xyz"), 3);
        assert_eq!(alg.postfix_len("abc", "xyz"), 0);
    }
}
