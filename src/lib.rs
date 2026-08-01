pub mod algorithms;

/// Trait for distance-based algorithms (lower = more similar)
pub trait Distance {
    /// Compute distance between two sequences
    fn distance(&self, s1: &[char], s2: &[char]) -> f64;

    /// Maximum possible distance
    fn maximum(&self, s1: &[char], s2: &[char]) -> f64 {
        std::cmp::max(s1.len(), s2.len()) as f64
    }

    /// Normalized distance (0 to 1)
    fn normalized_distance(&self, s1: &[char], s2: &[char]) -> f64 {
        let max = self.maximum(s1, s2);
        if max == 0.0 {
            0.0
        } else {
            self.distance(s1, s2) / max
        }
    }

    /// Similarity = maximum - distance
    fn similarity(&self, s1: &[char], s2: &[char]) -> f64 {
        self.maximum(s1, s2) - self.distance(s1, s2)
    }

    /// Normalized similarity (0 to 1)
    fn normalized_similarity(&self, s1: &[char], s2: &[char]) -> f64 {
        1.0 - self.normalized_distance(s1, s2)
    }
}

/// Trait for similarity-based algorithms (higher = more similar)
pub trait Similarity {
    /// Compute similarity between two sequences
    fn similarity(&self, s1: &[char], s2: &[char]) -> f64;

    /// Maximum possible similarity
    fn maximum(&self, _s1: &[char], _s2: &[char]) -> f64 {
        1.0
    }

    /// Distance = maximum - similarity
    fn distance(&self, s1: &[char], s2: &[char]) -> f64 {
        self.maximum(s1, s2) - self.similarity(s1, s2)
    }

    /// Normalized distance (0 to 1)
    fn normalized_distance(&self, s1: &[char], s2: &[char]) -> f64 {
        let max = self.maximum(s1, s2);
        if max == 0.0 {
            0.0
        } else {
            self.distance(s1, s2) / max
        }
    }

    /// Normalized similarity (0 to 1)
    fn normalized_similarity(&self, s1: &[char], s2: &[char]) -> f64 {
        1.0 - self.normalized_distance(s1, s2)
    }
}

/// Trait for vector-based algorithms (operate on float vectors, not strings)
pub trait VectorDistance {
    /// Compute distance between two float vectors
    fn vector_distance(&self, v1: &[f64], v2: &[f64]) -> f64;

    /// Maximum possible distance (for normalization)
    fn vector_maximum(&self, v1: &[f64], v2: &[f64]) -> f64 {
        let max_val = v1.iter().chain(v2.iter()).fold(0.0_f64, |a, b| a.max(*b));
        max_val * v1.len() as f64
    }

    /// Normalized distance (0 to 1)
    fn vector_normalized_distance(&self, v1: &[f64], v2: &[f64]) -> f64 {
        let max = self.vector_maximum(v1, v2);
        if max == 0.0 {
            0.0
        } else {
            self.vector_distance(v1, v2) / max
        }
    }

    /// Similarity = maximum - distance
    fn vector_similarity(&self, v1: &[f64], v2: &[f64]) -> f64 {
        self.vector_maximum(v1, v2) - self.vector_distance(v1, v2)
    }

    /// Normalized similarity (0 to 1)
    fn vector_normalized_similarity(&self, v1: &[f64], v2: &[f64]) -> f64 {
        1.0 - self.vector_normalized_distance(v1, v2)
    }
}

/// Convert a string to Vec<char> for processing
pub fn to_chars(s: &str) -> Vec<char> {
    s.chars().collect()
}

/// Split string into q-grams
pub fn find_ngrams(s: &[char], n: usize) -> Vec<Vec<char>> {
    if n == 0 || s.len() < n {
        return vec![];
    }
    s.windows(n).map(|w| w.to_vec()).collect()
}

/// Split string by words
pub fn split_words(s: &str) -> Vec<String> {
    s.split_whitespace().map(String::from).collect()
}
