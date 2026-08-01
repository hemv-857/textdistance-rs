pub struct ZLIBNCD;

impl Default for ZLIBNCD {
    fn default() -> Self {
        Self
    }
}

impl ZLIBNCD {
    pub fn new() -> Self {
        Self
    }

    fn compress_size(&self, data: &str) -> f64 {
        use flate2::write::ZlibEncoder;
        use flate2::Compression;
        use std::io::Write;

        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::best());
        encoder.write_all(data.as_bytes()).ok();
        let compressed = encoder.finish().unwrap_or_default();
        // Strip 2 bytes of header to match Python's codecs.encode(data, 'zlib_codec')[2:]
        let stripped = if compressed.len() > 2 {
            &compressed[2..]
        } else {
            &compressed[..]
        };
        stripped.len() as f64
    }

    fn ncd(&self, s1: &str, s2: &str) -> f64 {
        let c1 = self.compress_size(s1);
        let c2 = self.compress_size(s2);
        let concat_min = self
            .compress_size(&format!("{}{}", s1, s2))
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
    fn test_zlib_ncd() {
        let alg = ZLIBNCD::new();
        let same = alg.distance("test", "test");
        let diff = alg.distance("test", "nani");
        assert!(same <= diff);
    }
}
