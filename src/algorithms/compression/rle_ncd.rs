pub struct RLENCD;

impl Default for RLENCD {
    fn default() -> Self {
        Self
    }
}

impl RLENCD {
    pub fn new() -> Self {
        Self
    }

    fn compress(&self, data: &str) -> String {
        let mut result = String::new();
        let chars: Vec<char> = data.chars().collect();
        let mut i = 0;
        while i < chars.len() {
            let mut run_len = 1;
            while i + run_len < chars.len() && chars[i + run_len] == chars[i] {
                run_len += 1;
            }
            if run_len > 2 {
                result.push_str(&run_len.to_string());
                result.push(chars[i]);
            } else if run_len == 2 {
                result.push(chars[i]);
                result.push(chars[i]);
            } else {
                result.push(chars[i]);
            }
            i += run_len;
        }
        result
    }

    fn compress_size(&self, data: &str) -> f64 {
        self.compress(data).len() as f64
    }

    fn ncd(&self, s1: &str, s2: &str) -> f64 {
        let c1 = self.compress_size(s1);
        let c2 = self.compress_size(s2);
        let concat_min = self.compress_size(&format!("{}{}", s1, s2))
            .min(self.compress_size(&format!("{}{}", s2, s1)));
        let min_compressed = c1.min(c2);
        let max_compressed = c1.max(c2);
        if max_compressed == 0.0 {
            return 0.0;
        }
        (concat_min - min_compressed) / max_compressed
    }

    pub fn distance(&self, s1: &str, s2: &str) -> f64 {
        self.ncd(s1, s2)
    }

    pub fn similarity(&self, s1: &str, s2: &str) -> f64 {
        1.0 - self.ncd(s1, s2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rle_compress() {
        let alg = RLENCD::new();
        assert_eq!(alg.compress("AAABBBCCCD"), "3A3B3CD");
        assert_eq!(alg.compress("ABCD"), "ABCD");
        assert_eq!(alg.compress("AABB"), "AABB");
    }
}
