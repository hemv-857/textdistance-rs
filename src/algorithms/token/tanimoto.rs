use super::jaccard::Jaccard;

pub struct Tanimoto {
    jaccard: Jaccard,
}

impl Default for Tanimoto {
    fn default() -> Self {
        Self {
            jaccard: Jaccard::new(),
        }
    }
}

impl Tanimoto {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn similarity(&self, s1: &str, s2: &str) -> f64 {
        let j = self.jaccard.similarity(s1, s2);
        if j == 0.0 {
            f64::NEG_INFINITY
        } else {
            j.log2()
        }
    }

    pub fn distance(&self, s1: &str, s2: &str) -> f64 {
        1.0 - self.similarity(s1, s2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tanimoto_zero() {
        let alg = Tanimoto::new();
        let r = alg.similarity("abc", "xyz");
        assert_eq!(r, f64::NEG_INFINITY);
    }
}
