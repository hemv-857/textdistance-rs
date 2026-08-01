use std::collections::HashMap;
use crate::Similarity;

pub struct StrCmp95 {
    pub long_strings: bool,
}

impl StrCmp95 {
    pub fn new() -> Self {
        Self { long_strings: false }
    }
}

impl Default for StrCmp95 {
    fn default() -> Self {
        Self::new()
    }
}

impl Similarity for StrCmp95 {
    fn similarity(&self, s1: &[char], s2: &[char]) -> f64 {
        strcmp95_impl(s1, s2, self.long_strings)
    }

    fn maximum(&self, _s1: &[char], _s2: &[char]) -> f64 {
        1.0
    }
}

const SP_MX: &[(&str, &str)] = &[
    ("A", "E"), ("A", "I"), ("A", "O"), ("A", "U"), ("B", "V"), ("E", "I"),
    ("E", "O"), ("E", "U"), ("I", "O"), ("I", "U"), ("O", "U"), ("I", "Y"),
    ("E", "Y"), ("C", "G"), ("E", "F"), ("W", "U"), ("W", "V"), ("X", "K"),
    ("S", "Z"), ("X", "S"), ("Q", "C"), ("U", "V"), ("M", "N"), ("L", "I"),
    ("Q", "O"), ("P", "R"), ("I", "J"), ("2", "Z"), ("5", "S"), ("8", "B"),
    ("1", "I"), ("1", "L"), ("0", "O"), ("0", "Q"), ("C", "K"), ("G", "J"),
];

fn in_range(ch: char) -> bool {
    let o = ch as u32;
    o > 0 && o < 91
}

fn strcmp95_impl(s1: &[char], s2: &[char], long_strings: bool) -> f64 {
    // Uppercase and strip
    let s1: Vec<char> = s1.iter().map(|c| c.to_uppercase().next().unwrap_or(*c)).collect();
    let s2: Vec<char> = s2.iter().map(|c| c.to_uppercase().next().unwrap_or(*c)).collect();

    let len_s1 = s1.len();
    let len_s2 = s2.len();

    if len_s1 == 0 && len_s2 == 0 {
        return 1.0;
    }
    if len_s1 == 0 || len_s2 == 0 {
        return 0.0;
    }

    // Build adjustment weight table
    let mut adjwt: HashMap<(char, char), i32> = HashMap::new();
    for &(c1, c2) in SP_MX {
        let a = c1.chars().next().unwrap();
        let b = c2.chars().next().unwrap();
        adjwt.insert((a, b), 3);
        adjwt.insert((b, a), 3);
    }

    let search_range = std::cmp::max(len_s1, len_s2);
    let minv = std::cmp::min(len_s1, len_s2);

    let mut s1_flag = vec![0i32; search_range];
    let mut s2_flag = vec![0i32; search_range];
    let sr = if search_range > 2 { search_range / 2 - 1 } else { 0 };

    // Count matching pairs
    let mut num_com = 0;
    for (i, &sc1) in s1.iter().enumerate() {
        let lowlim = if i > sr { i - sr } else { 0 };
        let hilim = std::cmp::min(i + sr, len_s2 - 1);
        for j in lowlim..=hilim {
            if s2_flag[j] == 0 && s2[j] == sc1 {
                s2_flag[j] = 1;
                s1_flag[i] = 1;
                num_com += 1;
                break;
            }
        }
    }

    if num_com == 0 {
        return 0.0;
    }

    // Count transpositions
    let mut k = 0;
    let mut n_trans = 0;
    for (i, &sc1) in s1.iter().enumerate() {
        if s1_flag[i] == 0 {
            continue;
        }
        while k < len_s2 {
            if s2_flag[k] != 0 {
                break;
            }
            k += 1;
        }
        if k < len_s2 && sc1 != s2[k] {
            n_trans += 1;
        }
        k += 1;
    }
    n_trans /= 2;

    // Adjust for similar unmatched characters
    let mut n_simi = 0;
    if minv > num_com {
        for i in 0..len_s1 {
            if s1_flag[i] != 0 || !in_range(s1[i]) {
                continue;
            }
            for j in 0..len_s2 {
                if s2_flag[j] != 0 || !in_range(s2[j]) {
                    continue;
                }
                if let Some(&w) = adjwt.get(&(s1[i], s2[j])) {
                    n_simi += w;
                    s2_flag[j] = 2;
                    break;
                }
            }
        }
    }

    let num_sim = n_simi as f64 / 10.0 + num_com as f64;

    // Main weight
    let mut weight = num_sim / len_s1 as f64 + num_sim / len_s2 as f64;
    weight += (num_com - n_trans) as f64 / num_com as f64;
    weight /= 3.0;

    if weight <= 0.7 {
        return weight;
    }

    // Boost for common prefix
    let j = std::cmp::min(minv, 4);
    let mut prefix_len = 0;
    while prefix_len < j && s1[prefix_len] == s2[prefix_len] && !s1[prefix_len].is_ascii_digit() {
        prefix_len += 1;
    }
    if prefix_len > 0 {
        weight += prefix_len as f64 * 0.1 * (1.0 - weight);
    }

    if !long_strings {
        return weight;
    }
    if minv <= 4 {
        return weight;
    }
    if num_com <= prefix_len + 1 || 2 * num_com < minv + prefix_len {
        return weight;
    }
    if s1[0].is_ascii_digit() {
        return weight;
    }
    let res = (num_com - prefix_len - 1) as f64 / (len_s1 + len_s2 - prefix_len * 2 + 2) as f64;
    weight += (1.0 - weight) * res;
    weight
}

pub fn similarity_str(s1: &str, s2: &str) -> f64 {
    let c1: Vec<char> = s1.chars().collect();
    let c2: Vec<char> = s2.chars().collect();
    StrCmp95::new().similarity(&c1, &c2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strcmp95() {
        let alg = StrCmp95::new();
        let eps = 1e-6;
        let test = |a: &str, b: &str| {
            let c1: Vec<char> = a.chars().collect();
            let c2: Vec<char> = b.chars().collect();
            alg.similarity(&c1, &c2)
        };
        assert!((test("MARTHA", "MARHTA") - 0.9611111111111111).abs() < eps);
        assert!((test("DWAYNE", "DUANE") - 0.873).abs() < eps);
        assert!((test("DIXON", "DICKSONX") - 0.839333333).abs() < eps);
        assert!((test("TEST", "TEXT") - 0.9066666666666666).abs() < eps);
    }
}
