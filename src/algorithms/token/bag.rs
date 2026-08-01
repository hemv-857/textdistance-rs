use std::collections::HashMap;

pub struct Bag;

impl Default for Bag {
    fn default() -> Self {
        Self
    }
}

impl Bag {
    pub fn new() -> Self {
        Self
    }

    pub fn distance(&self, s1: &str, s2: &str) -> f64 {
        let c1 = char_counts(s1);
        let c2 = char_counts(s2);
        let intersection = intersect_counters(&c1, &c2);

        let d1: usize = c1
            .iter()
            .map(|(ch, cnt)| cnt - intersection.get(ch).unwrap_or(&0))
            .sum();
        let d2: usize = c2
            .iter()
            .map(|(ch, cnt)| cnt - intersection.get(ch).unwrap_or(&0))
            .sum();

        d1.max(d2) as f64
    }

    pub fn similarity(&self, s1: &str, s2: &str) -> f64 {
        let max_len = s1.chars().count().max(s2.chars().count()) as f64;
        max_len - self.distance(s1, s2)
    }
}

pub fn char_counts(s: &str) -> HashMap<char, usize> {
    let mut counts = HashMap::new();
    for ch in s.chars() {
        *counts.entry(ch).or_insert(0) += 1;
    }
    counts
}

pub fn intersect_counters(
    a: &HashMap<char, usize>,
    b: &HashMap<char, usize>,
) -> HashMap<char, usize> {
    let mut result = HashMap::new();
    for (ch, &cnt_a) in a {
        if let Some(&cnt_b) = b.get(ch) {
            result.insert(*ch, cnt_a.min(cnt_b));
        }
    }
    result
}

pub fn union_counters(a: &HashMap<char, usize>, b: &HashMap<char, usize>) -> HashMap<char, usize> {
    let mut result = a.clone();
    for (ch, &cnt) in b {
        let entry = result.entry(*ch).or_insert(0);
        if cnt > *entry {
            *entry = cnt;
        }
    }
    result
}

pub fn sum_counters(a: &HashMap<char, usize>, b: &HashMap<char, usize>) -> HashMap<char, usize> {
    let mut result = a.clone();
    for (ch, &cnt) in b {
        *result.entry(*ch).or_insert(0) += cnt;
    }
    result
}

pub fn count_sum(c: &HashMap<char, usize>) -> usize {
    c.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bag() {
        let alg = Bag::new();
        assert_eq!(alg.distance("qwe", "qwe"), 0.0);
        assert_eq!(alg.distance("qwe", "erty"), 3.0);
        assert_eq!(alg.distance("qwe", "ewq"), 0.0);
        assert_eq!(alg.distance("qwe", "rtys"), 4.0);
    }
}
