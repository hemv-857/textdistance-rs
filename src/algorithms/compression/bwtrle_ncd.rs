pub struct BWTRLENCD {
    terminator: char,
}

impl Default for BWTRLENCD {
    fn default() -> Self {
        Self { terminator: '\0' }
    }
}

impl BWTRLENCD {
    pub fn new() -> Self {
        Self::default()
    }

    fn bwt(&self, data: &str) -> String {
        if data.is_empty() {
            return self.terminator.to_string();
        }
        let mut s = data.to_string();
        if !s.contains(self.terminator) {
            s.push(self.terminator);
        }
        let n = s.len();
        let bytes: Vec<u8> = s.bytes().collect();
        let mut indices: Vec<usize> = (0..n).collect();
        indices.sort_by(|&a, &b| {
            bytes[a..].iter().chain(bytes[..a].iter())
                .zip(bytes[b..].iter().chain(bytes[..b].iter()))
                .find_map(|(x, y)| if x != y { Some(x.cmp(y)) } else { None })
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        indices.iter().map(|&i| {
            if i == 0 {
                *bytes.last().unwrap()
            } else {
                bytes[i - 1]
            }
        }).map(|b| b as char).collect()
    }

    fn rle_compress(&self, data: &str) -> String {
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

    fn compress(&self, data: &str) -> String {
        let bwt = self.bwt(data);
        self.rle_compress(&bwt)
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
    fn test_bwtrle_ncd() {
        let alg = BWTRLENCD::new();
        let same = alg.distance("test", "test");
        let diff = alg.distance("test", "nani");
        assert!(same <= diff);
        assert!((same - 0.6).abs() < 0.05);
    }
}
