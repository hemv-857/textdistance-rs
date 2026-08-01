use crate::Similarity;

pub struct JaroWinkler {
    pub long_tolerance: bool,
    pub winklerize: bool,
}

impl JaroWinkler {
    pub fn new() -> Self {
        Self {
            long_tolerance: false,
            winklerize: true,
        }
    }

    pub fn jaro() -> Self {
        Self {
            long_tolerance: false,
            winklerize: false,
        }
    }

    pub fn long_tolerance() -> Self {
        Self {
            long_tolerance: true,
            winklerize: true,
        }
    }
}

impl Default for JaroWinkler {
    fn default() -> Self {
        Self::new()
    }
}

impl Similarity for JaroWinkler {
    fn similarity(&self, s1: &[char], s2: &[char]) -> f64 {
        jaro_winkler_impl(s1, s2, self.winklerize, self.long_tolerance)
    }

    fn maximum(&self, _s1: &[char], _s2: &[char]) -> f64 {
        1.0
    }
}

fn jaro_winkler_impl(s1: &[char], s2: &[char], winklerize: bool, long_tolerance: bool) -> f64 {
    let s1_len = s1.len();
    let s2_len = s2.len();

    if s1_len == 0 && s2_len == 0 {
        return 1.0;
    }
    if s1_len == 0 || s2_len == 0 {
        return 0.0;
    }

    let min_len = std::cmp::min(s1_len, s2_len);
    let mut search_range = std::cmp::max(s1_len, s2_len) / 2;
    if search_range > 0 {
        search_range -= 1;
    }

    let mut s1_flags = vec![false; s1_len];
    let mut s2_flags = vec![false; s2_len];

    // Count matching characters within search range
    let mut common_chars = 0;
    for (i, &s1_ch) in s1.iter().enumerate() {
        let low = if i >= search_range { i - search_range } else { 0 };
        let hi = std::cmp::min(i + search_range, s2_len - 1);
        for j in low..=hi {
            if !s2_flags[j] && s2[j] == s1_ch {
                s1_flags[i] = true;
                s2_flags[j] = true;
                common_chars += 1;
                break;
            }
        }
    }

    if common_chars == 0 {
        return 0.0;
    }

    // Count transpositions
    let mut k = 0;
    let mut trans_count = 0;
    for (i, &s1_f) in s1_flags.iter().enumerate() {
        if s1_f {
            while k < s2_len {
                if s2_flags[k] {
                    break;
                }
                k += 1;
            }
            if k < s2_len && s1[i] != s2[k] {
                trans_count += 1;
            }
            k += 1;
        }
    }
    trans_count /= 2;

    // Weight computation
    let common = common_chars as f64;
    let mut weight = common / s1_len as f64 + common / s2_len as f64;
    weight += (common - trans_count as f64) / common;
    weight /= 3.0;

    // Winkler modification
    if !winklerize {
        return weight;
    }
    if weight <= 0.7 {
        return weight;
    }

    // Adjust for up to first 4 chars in common
    let j = std::cmp::min(min_len, 4);
    let mut prefix_len = 0;
    while prefix_len < j && s1[prefix_len] == s2[prefix_len] {
        prefix_len += 1;
    }
    if prefix_len > 0 {
        weight += prefix_len as f64 * 0.1 * (1.0 - weight);
    }

    // Optionally adjust for long strings
    if !long_tolerance || min_len <= 4 {
        return weight;
    }
    if common_chars <= prefix_len + 1 || 2 * common_chars < min_len + prefix_len {
        return weight;
    }
    let tmp = (common_chars - prefix_len - 1) as f64 / (s1_len + s2_len - prefix_len * 2 + 2) as f64;
    weight += (1.0 - weight) * tmp;
    weight
}

pub fn similarity_str(s1: &str, s2: &str) -> f64 {
    let c1: Vec<char> = s1.chars().collect();
    let c2: Vec<char> = s2.chars().collect();
    JaroWinkler::new().similarity(&c1, &c2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jaro_winkler() {
        let alg = JaroWinkler::new();
        let eps = 1e-6;
        let test = |a: &str, b: &str| {
            let c1: Vec<char> = a.chars().collect();
            let c2: Vec<char> = b.chars().collect();
            alg.similarity(&c1, &c2)
        };
        assert!((test("MARTHA", "MARHTA") - 0.9611111111111111).abs() < eps);
        assert!((test("DWAYNE", "DUANE") - 0.84).abs() < eps);
        assert!((test("frog", "fog") - 0.925).abs() < eps);
        assert!((test("fly", "ant") - 0.0).abs() < eps);
    }

    #[test]
    fn test_jaro() {
        let alg = JaroWinkler::jaro();
        let eps = 1e-6;
        let test = |a: &str, b: &str| {
            let c1: Vec<char> = a.chars().collect();
            let c2: Vec<char> = b.chars().collect();
            alg.similarity(&c1, &c2)
        };
        assert!((test("MARTHA", "MARHTA") - 0.944444444).abs() < eps);
        assert!((test("DWAYNE", "DUANE") - 0.822222222).abs() < eps);
        assert!((test("fly", "ant") - 0.0).abs() < eps);
    }
}
