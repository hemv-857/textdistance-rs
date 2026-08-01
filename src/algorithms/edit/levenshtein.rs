use crate::Distance;

pub struct Levenshtein;

impl Distance for Levenshtein {
    fn distance(&self, s1: &[char], s2: &[char]) -> f64 {
        let len1 = s1.len();
        let len2 = s2.len();

        if len1 == 0 {
            return len2 as f64;
        }
        if len2 == 0 {
            return len1 as f64;
        }

        let mut prev: Vec<usize> = (0..=len2).collect();
        let mut curr = vec![0usize; len2 + 1];

        for i in 1..=len1 {
            curr[0] = i;
            for j in 1..=len2 {
                let cost = if s1[i - 1] == s2[j - 1] { 0 } else { 1 };
                curr[j] = std::cmp::min(
                    std::cmp::min(
                        prev[j] + 1,      // deletion
                        curr[j - 1] + 1,  // insertion
                    ),
                    prev[j - 1] + cost,   // substitution
                );
            }
            std::mem::swap(&mut prev, &mut curr);
        }

        prev[len2] as f64
    }
}

pub fn distance_str(s1: &str, s2: &str) -> f64 {
    let c1: Vec<char> = s1.chars().collect();
    let c2: Vec<char> = s2.chars().collect();
    Levenshtein.distance(&c1, &c2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_levenshtein() {
        let alg = Levenshtein;
        let test = |a: &str, b: &str| {
            let c1: Vec<char> = a.chars().collect();
            let c2: Vec<char> = b.chars().collect();
            alg.distance(&c1, &c2) as i64
        };
        assert_eq!(test("test", "text"), 1);
        assert_eq!(test("test", "tset"), 2);
        assert_eq!(test("test", "qwe"), 4);
        assert_eq!(test("test", "testit"), 2);
        assert_eq!(test("test", "tesst"), 1);
        assert_eq!(test("test", "tet"), 1);
        assert_eq!(test("", ""), 0);
        assert_eq!(test("test", "test"), 0);
    }
}
