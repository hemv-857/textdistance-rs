use crate::Distance;

pub struct Hamming;

impl Distance for Hamming {
    fn distance(&self, s1: &[char], s2: &[char]) -> f64 {
        let len = std::cmp::max(s1.len(), s2.len()) as f64;
        if len == 0.0 {
            return 0.0;
        }
        let mut count = 0;
        for (a, b) in s1.iter().zip(s2.iter()) {
            if a != b {
                count += 1;
            }
        }
        count += (s1.len() as i64 - s2.len() as i64).unsigned_abs() as usize;
        count as f64
    }

    fn maximum(&self, s1: &[char], s2: &[char]) -> f64 {
        std::cmp::max(s1.len(), s2.len()) as f64
    }
}

pub fn distance_str(s1: &str, s2: &str) -> f64 {
    let c1: Vec<char> = s1.chars().collect();
    let c2: Vec<char> = s2.chars().collect();
    Hamming.distance(&c1, &c2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hamming() {
        let alg = Hamming;
        let test = |a: &str, b: &str| {
            let c1: Vec<char> = a.chars().collect();
            let c2: Vec<char> = b.chars().collect();
            alg.distance(&c1, &c2) as i64
        };
        assert_eq!(test("test", "text"), 1);
        assert_eq!(test("test", "tset"), 2);
        assert_eq!(test("test", "qwe"), 4);
        assert_eq!(test("test", "testit"), 2);
        assert_eq!(test("test", "tesst"), 2);
        assert_eq!(test("test", "tet"), 2);
        assert_eq!(test("", ""), 0);
        assert_eq!(test("test", "test"), 0);
    }
}
