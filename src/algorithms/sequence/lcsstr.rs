use std::cmp::max;

pub struct LCSStr;

impl Default for LCSStr {
    fn default() -> Self {
        Self
    }
}

impl LCSStr {
    pub fn new() -> Self {
        Self
    }

    pub fn similarity(&self, s1: &str, s2: &str) -> f64 {
        self.lcss(s1, s2).chars().count() as f64
    }

    pub fn distance(&self, s1: &str, s2: &str) -> f64 {
        let max_len = max(s1.chars().count(), s2.chars().count()) as f64;
        max_len - self.similarity(s1, s2)
    }

    pub fn lcss(&self, s1: &str, s2: &str) -> String {
        if s1.is_empty() || s2.is_empty() {
            return String::new();
        }

        let chars1: Vec<char> = s1.chars().collect();
        let chars2: Vec<char> = s2.chars().collect();
        let len1 = chars1.len();
        let len2 = chars2.len();

        let (short_chars, long_chars, short_len, _long_len) = if len1 <= len2 {
            (&chars1, &chars2, len1, len2)
        } else {
            (&chars2, &chars1, len2, len1)
        };

        for n in (1..=short_len).rev() {
            for i in 0..=(short_len - n) {
                let candidate: String = short_chars[i..i + n].iter().collect();
                if long_chars.windows(n).any(|w| w == &short_chars[i..i + n]) {
                    return candidate;
                }
            }
        }

        String::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lcsstr() {
        let alg = LCSStr::new();
        assert_eq!(alg.lcss("ab", "abcd"), "ab");
        assert_eq!(alg.lcss("abcd", "ab"), "ab");
        assert_eq!(alg.lcss("abcd", "bc"), "bc");
        assert_eq!(alg.lcss("bc", "abcd"), "bc");
        assert_eq!(alg.lcss("abcd", "cd"), "cd");
        assert_eq!(alg.lcss("abcd", "ef"), "");
    }
}
