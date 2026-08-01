use crate::Distance;

pub struct DamerauLevenshtein {
    pub restricted: bool,
}

impl DamerauLevenshtein {
    pub fn new() -> Self {
        Self { restricted: true }
    }

    pub fn unrestricted() -> Self {
        Self { restricted: false }
    }
}

impl Default for DamerauLevenshtein {
    fn default() -> Self {
        Self::new()
    }
}

impl Distance for DamerauLevenshtein {
    fn distance(&self, s1: &[char], s2: &[char]) -> f64 {
        if self.restricted {
            restricted_dl(s1, s2)
        } else {
            unrestricted_dl(s1, s2)
        }
    }
}

#[allow(clippy::needless_range_loop)]
fn restricted_dl(s1: &[char], s2: &[char]) -> f64 {
    let len1 = s1.len();
    let len2 = s2.len();

    if len1 == 0 {
        return len2 as f64;
    }
    if len2 == 0 {
        return len1 as f64;
    }

    // Use a flat 2D matrix stored as 1D
    let mut d = vec![0usize; (len1 + 1) * (len2 + 1)];

    for i in 0..=len1 {
        d[i * (len2 + 1)] = i;
    }
    for j in 0..=len2 {
        d[j] = j;
    }

    for i in 1..=len1 {
        for j in 1..=len2 {
            let cost = if s1[i - 1] == s2[j - 1] { 0 } else { 1 };
            let idx = i * (len2 + 1) + j;

            d[idx] = std::cmp::min(
                std::cmp::min(
                    d[(i - 1) * (len2 + 1) + j] + 1, // deletion
                    d[i * (len2 + 1) + (j - 1)] + 1, // insertion
                ),
                d[(i - 1) * (len2 + 1) + (j - 1)] + cost, // substitution
            );

            // Transposition
            if i > 1 && j > 1 && s1[i - 1] == s2[j - 2] && s1[i - 2] == s2[j - 1] {
                d[idx] = std::cmp::min(d[idx], d[(i - 2) * (len2 + 1) + (j - 2)] + cost);
            }
        }
    }

    d[len1 * (len2 + 1) + len2] as f64
}

fn unrestricted_dl(s1: &[char], s2: &[char]) -> f64 {
    let len1 = s1.len();
    let len2 = s2.len();

    if len1 == 0 {
        return len2 as f64;
    }
    if len2 == 0 {
        return len1 as f64;
    }

    let maxdist = (len1 + len2) as i64;
    let mut d: std::collections::HashMap<(i64, i64), i64> = std::collections::HashMap::new();
    let mut da: std::collections::HashMap<char, i64> = std::collections::HashMap::new();

    d.insert((-1, -1), maxdist);
    for i in 0..=len1 as i64 {
        d.insert((i, -1), maxdist);
        d.insert((i, 0), i);
    }
    for j in 0..=len2 as i64 {
        d.insert((-1, j), maxdist);
        d.insert((0, j), j);
    }

    for i in 1..=len1 as i64 {
        let mut db = 0;
        for j in 1..=len2 as i64 {
            let i1 = *da.get(&s2[(j - 1) as usize]).unwrap_or(&0);
            let j1 = db;

            let cost = if s1[(i - 1) as usize] == s2[(j - 1) as usize] {
                db = j;
                0
            } else {
                1
            };

            // Python: d[i1-1, j1-1] + (i-i1)-1 + (j-j1)
            let trans_cost = d[&(i1 - 1, j1 - 1)] + (i - i1) - 1 + (j - j1);

            let val_sub = d[&(i - 1, j - 1)] + cost;
            let val_ins = d[&(i, j - 1)] + 1;
            let val_del = d[&(i - 1, j)] + 1;

            d.insert(
                (i, j),
                std::cmp::min(
                    std::cmp::min(val_sub, std::cmp::min(val_ins, val_del)),
                    trans_cost,
                ),
            );
        }
        da.insert(s1[(i - 1) as usize], i);
    }

    *d.get(&(len1 as i64, len2 as i64)).unwrap_or(&0) as f64
}

pub fn distance_str(s1: &str, s2: &str) -> f64 {
    let c1: Vec<char> = s1.chars().collect();
    let c2: Vec<char> = s2.chars().collect();
    DamerauLevenshtein::new().distance(&c1, &c2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_restricted() {
        let alg = DamerauLevenshtein::new();
        let test = |a: &str, b: &str| {
            let c1: Vec<char> = a.chars().collect();
            let c2: Vec<char> = b.chars().collect();
            alg.distance(&c1, &c2) as i64
        };
        assert_eq!(test("test", "text"), 1);
        assert_eq!(test("test", "tset"), 1);
        assert_eq!(test("ab", "ba"), 1);
        assert_eq!(test("BA", "ACB"), 3);
    }

    #[test]
    fn test_unrestricted() {
        let alg = DamerauLevenshtein::unrestricted();
        let test = |a: &str, b: &str| {
            let c1: Vec<char> = a.chars().collect();
            let c2: Vec<char> = b.chars().collect();
            alg.distance(&c1, &c2) as i64
        };
        // Python test expectations for unrestricted
        assert_eq!(test("ab", "bca"), 2);
        assert_eq!(test("abcd", "bdac"), 3);
        assert_eq!(test("test", "text"), 1);
        assert_eq!(test("test", "tset"), 1);
        assert_eq!(test("ab", "ba"), 1);
        assert_eq!(test("ab", "cde"), 3);
        assert_eq!(test("Niall", "Neil"), 3);
        assert_eq!(test("ATCG", "TAGC"), 2);
    }
}
