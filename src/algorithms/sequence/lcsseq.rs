use std::cmp::max;

pub struct LCSSeq;

impl Default for LCSSeq {
    fn default() -> Self {
        Self
    }
}

impl LCSSeq {
    pub fn new() -> Self {
        Self
    }

    pub fn similarity(&self, s1: &str, s2: &str) -> f64 {
        self.lcs(s1, s2).chars().count() as f64
    }

    pub fn distance(&self, s1: &str, s2: &str) -> f64 {
        let max_len = max(s1.chars().count(), s2.chars().count()) as f64;
        max_len - self.similarity(s1, s2)
    }

    fn lcs(&self, s1: &str, s2: &str) -> String {
        let chars1: Vec<char> = s1.chars().collect();
        let chars2: Vec<char> = s2.chars().collect();
        let len1 = chars1.len();
        let len2 = chars2.len();

        let mut dp = vec![vec![0usize; len2 + 1]; len1 + 1];

        for i in 1..=len1 {
            for j in 1..=len2 {
                if chars1[i - 1] == chars2[j - 1] {
                    dp[i][j] = dp[i - 1][j - 1] + 1;
                } else {
                    dp[i][j] = max(dp[i - 1][j], dp[i][j - 1]);
                }
            }
        }

        let mut result = Vec::new();
        let mut i = len1;
        let mut j = len2;
        while i > 0 && j > 0 {
            if chars1[i - 1] == chars2[j - 1] {
                result.push(chars1[i - 1]);
                i -= 1;
                j -= 1;
            } else if dp[i - 1][j] > dp[i][j - 1] {
                i -= 1;
            } else {
                j -= 1;
            }
        }
        result.reverse();
        result.into_iter().collect()
    }

    pub fn lcs_multiseq(&self, sequences: &[&str]) -> String {
        if sequences.is_empty() {
            return String::new();
        }
        if sequences.len() == 1 {
            return sequences[0].to_string();
        }
        let mut result = self.lcs(sequences[0], sequences[1]);
        for seq in &sequences[2..] {
            result = self.lcs(&result, seq);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lcsseq() {
        let alg = LCSSeq::new();
        assert_eq!(alg.lcs("ab", "cd"), "");
        assert_eq!(alg.lcs("abcd", "abcd"), "abcd");
        assert_eq!(alg.lcs("test", "text"), "tet");
        assert_eq!(alg.lcs("DIXON", "DICKSONX"), "DION");
    }

    #[test]
    fn test_lcsseq_multiseq() {
        let alg = LCSSeq::new();
        assert_eq!(alg.lcs_multiseq(&["a", "b", "c"]), "");
        assert_eq!(alg.lcs_multiseq(&["a", "a", "a"]), "a");
        assert_eq!(alg.lcs_multiseq(&["test", "text", "tempest"]), "tet");
    }
}
