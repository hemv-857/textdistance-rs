pub struct LZMANCD;

impl Default for LZMANCD {
    fn default() -> Self {
        Self
    }
}

impl LZMANCD {
    pub fn new() -> Self {
        Self
    }

    fn compress_size(&self, data: &str) -> f64 {
        let mut output = Vec::new();
        lzma_rs::lzma_compress(&mut data.as_bytes(), &mut output).unwrap_or(());
        // Python uses lzma.compress(data)[14:] which strips XZ container header
        // lzma_rs produces raw LZMA format - strip its 13-byte header
        // (5 bytes properties + 8 bytes uncompressed size)
        let header_size = 13;
        let body = if output.len() > header_size {
            &output[header_size..]
        } else {
            &[]
        };
        body.len() as f64
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
    fn test_lzma_ncd() {
        let alg = LZMANCD::new();
        let same = alg.distance("test", "test");
        let diff = alg.distance("test", "nani");
        assert!(same <= diff);
    }
}

