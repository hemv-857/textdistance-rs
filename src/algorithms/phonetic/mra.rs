pub struct MRA;

impl Default for MRA {
    fn default() -> Self {
        Self
    }
}

impl MRA {
    pub fn new() -> Self {
        Self
    }

    fn calc_mra(&self, word: &str) -> String {
        if word.is_empty() {
            return word.to_string();
        }
        let upper: String = word.to_uppercase();
        let vowels = ['A', 'E', 'I', 'O', 'U'];

        let first = upper.chars().next().unwrap();
        let rest: String = upper.chars().skip(1).filter(|c| !vowels.contains(c)).collect();

        // Remove consecutive duplicates (like Unix uniq)
        let mut deduped = String::new();
        let mut prev = '\0';
        for ch in std::iter::once(first).chain(rest.chars()) {
            if ch != prev {
                deduped.push(ch);
                prev = ch;
            }
        }

        if deduped.chars().count() > 6 {
            let chars: Vec<char> = deduped.chars().collect();
            let prefix: String = chars[..3].iter().collect();
            let suffix: String = chars[chars.len()-3..].iter().collect();
            format!("{}{}", prefix, suffix)
        } else {
            deduped
        }
    }

    pub fn similarity(&self, s1: &str, s2: &str) -> f64 {
        if s1 == s2 {
            return self.maximum(s1, s2);
        }
        self.call(s1, s2)
    }

    pub fn distance(&self, s1: &str, s2: &str) -> f64 {
        let maximum = self.maximum(s1, s2);
        maximum - self.similarity(s1, s2)
    }

    fn maximum(&self, s1: &str, s2: &str) -> f64 {
        if s1.is_empty() || s2.is_empty() {
            return 0.0;
        }
        let mra1: Vec<char> = self.calc_mra(s1).chars().collect();
        let mra2: Vec<char> = self.calc_mra(s2).chars().collect();
        mra1.len().max(mra2.len()) as f64
    }

    fn call(&self, s1: &str, s2: &str) -> f64 {
        if s1.is_empty() || s2.is_empty() {
            return 0.0;
        }

        let mra1: Vec<char> = self.calc_mra(s1).chars().collect();
        let mra2: Vec<char> = self.calc_mra(s2).chars().collect();
        let max_length = mra1.len().max(mra2.len());

        if mra1 == mra2 {
            return max_length as f64;
        }

        if (mra1.len() as i64 - mra2.len() as i64).unsigned_abs() > 2 {
            return 0.0;
        }

        // Python MRA algorithm: iterative position-based matching
        let mut seqs: Vec<Vec<char>> = vec![mra1.clone(), mra2.clone()];
        let mut lengths: Vec<usize> = seqs.iter().map(|s| s.len()).collect();
        let count = seqs.len();

        for _ in 0..count {
            let minlen = *lengths.iter().min().unwrap_or(&0);
            let mut new_seqs: Vec<Vec<char>> = Vec::new();

            // Find mismatching character pairs at each position
            for pos in 0..minlen {
                let chars: Vec<char> = seqs.iter().map(|s| s[pos]).collect();
                if !self.identical(&chars) {
                    new_seqs.push(chars);
                }
            }

            if new_seqs.is_empty() {
                break;
            }

            // Transpose new_seqs and rebuild sequences
            let transposed: Vec<Vec<char>> = if !new_seqs.is_empty() {
                let inner_len = new_seqs[0].len();
                (0..inner_len).map(|i| new_seqs.iter().map(|s| s[i]).collect()).collect()
            } else {
                Vec::new()
            };

            // Rebuild sequences: matched portion + remainder
            // transposed[i] = mismatched chars from sequence i (Python's zip_longest pattern)
            for i in 0..count {
                let matched: Vec<char> = transposed.get(i).cloned().unwrap_or_default();
                let remainder: Vec<char> = seqs[i][minlen..].to_vec();
                seqs[i] = matched.into_iter().chain(remainder).collect();
            }

            lengths = seqs.iter().map(|s| s.len()).collect();
        }

        if lengths.is_empty() {
            max_length as f64
        } else {
            (max_length as i64 - *lengths.iter().max().unwrap_or(&0) as i64) as f64
        }
    }

    fn identical(&self, chars: &[char]) -> bool {
        if chars.is_empty() {
            return true;
        }
        let first = chars[0];
        chars.iter().all(|&c| c == first)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mra_calc() {
        let alg = MRA::new();
        assert_eq!(alg.calc_mra("Robert"), "RBRT");
        assert_eq!(alg.calc_mra("Rupert"), "RPRT");
        assert_eq!(alg.calc_mra(""), "");
    }

    #[test]
    fn test_mra_distance() {
        let alg = MRA::new();
        let d = alg.distance("MARTHA", "MARHTA");
        assert!((d - 2.0).abs() < 1e-10, "got {}", d);
    }
}

