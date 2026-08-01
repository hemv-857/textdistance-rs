use super::lcsstr::LCSStr;

pub struct RatcliffObershelp {
    lcsstr: LCSStr,
}

impl Default for RatcliffObershelp {
    fn default() -> Self {
        Self {
            lcsstr: LCSStr::new(),
        }
    }
}

impl RatcliffObershelp {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn similarity(&self, s1: &str, s2: &str) -> f64 {
        let total_len = s1.chars().count() + s2.chars().count();
        if total_len == 0 {
            return 1.0;
        }
        let matched = self._find(s1, s2);
        2.0 * matched as f64 / total_len as f64
    }

    pub fn distance(&self, s1: &str, s2: &str) -> f64 {
        1.0 - self.similarity(s1, s2)
    }

    fn _find(&self, s1: &str, s2: &str) -> usize {
        let subseq = self.lcsstr.lcss(s1, s2);
        let length = subseq.chars().count();
        if length == 0 {
            return 0;
        }
        let chars1: Vec<char> = s1.chars().collect();
        let chars2: Vec<char> = s2.chars().collect();
        let subseq_chars: Vec<char> = subseq.chars().collect();

        let pos1 = chars1
            .windows(subseq_chars.len())
            .position(|w| w == subseq_chars.as_slice())
            .unwrap_or(0);
        let pos2 = chars2
            .windows(subseq_chars.len())
            .position(|w| w == subseq_chars.as_slice())
            .unwrap_or(0);

        let before1: String = chars1[..pos1].iter().collect();
        let after1: String = chars1[pos1 + length..].iter().collect();
        let before2: String = chars2[..pos2].iter().collect();
        let after2: String = chars2[pos2 + length..].iter().collect();

        length + self._find(&before1, &before2) + self._find(&after1, &after2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ratcliff_obershelp() {
        let alg = RatcliffObershelp::new();
        let r = alg.similarity("test", "test");
        assert!((r - 1.0).abs() < 1e-10);
        let r = alg.similarity("ab", "cd");
        assert!((r - 0.0).abs() < 1e-10);
    }
}
