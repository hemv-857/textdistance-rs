pub struct Editex {
    match_cost: i64,
    group_cost: i64,
    mismatch_cost: i64,
    local: bool,
    groups: Vec<Vec<char>>,
    grouped: Vec<char>,
    ungrouped: Vec<char>,
}

impl Default for Editex {
    fn default() -> Self {
        let groups = vec![
            vec!['A', 'E', 'I', 'O', 'U', 'Y'],
            vec!['B', 'P'],
            vec!['C', 'K', 'Q'],
            vec!['D', 'T'],
            vec!['L', 'R'],
            vec!['M', 'N'],
            vec!['G', 'J'],
            vec!['F', 'P', 'V'],
            vec!['S', 'X', 'Z'],
            vec!['C', 'S', 'Z'],
        ];
        let grouped: Vec<char> = groups.iter().flat_map(|g| g.iter().cloned()).collect();
        let ungrouped = vec!['H', 'W'];

        Self {
            match_cost: 0,
            group_cost: 1,
            mismatch_cost: 2,
            local: false,
            groups,
            grouped,
            ungrouped,
        }
    }
}

impl Editex {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_local(local: bool) -> Self {
        let mut e = Self::default();
        e.local = local;
        e
    }

    fn in_grouped(&self, ch: char) -> bool {
        self.grouped.contains(&ch)
    }

    fn in_ungrouped(&self, ch: char) -> bool {
        self.ungrouped.contains(&ch)
    }

    fn same_group(&self, c1: char, c2: char) -> bool {
        self.groups.iter().any(|g| g.contains(&c1) && g.contains(&c2))
    }

    fn r_cost(&self, c1: char, c2: char) -> i64 {
        if c1 == c2 {
            return self.match_cost;
        }
        if !self.in_grouped(c1) || !self.in_grouped(c2) {
            return self.mismatch_cost;
        }
        if self.same_group(c1, c2) {
            return self.group_cost;
        }
        self.mismatch_cost
    }

    fn d_cost(&self, c1: char, c2: char) -> i64 {
        if c1 != c2 && self.in_ungrouped(c1) {
            return self.group_cost;
        }
        self.r_cost(c1, c2)
    }

    pub fn distance(&self, s1: &str, s2: &str) -> i64 {
        let max_length = (s1.chars().count().max(s2.chars().count()) * self.mismatch_cost as usize) as i64;
        let s1_upper: Vec<char> = std::iter::once(' ').chain(s1.to_uppercase().chars()).collect();
        let s2_upper: Vec<char> = std::iter::once(' ').chain(s2.to_uppercase().chars()).collect();
        let len1 = s1_upper.len();
        let len2 = s2_upper.len();

        let mut d = vec![vec![0i64; len2]; len1];

        if !self.local {
            for i in 1..len1 {
                d[i][0] = d[i - 1][0] + self.d_cost(s1_upper[i - 1], s1_upper[i]);
            }
        }
        for j in 1..len2 {
            d[0][j] = d[0][j - 1] + self.d_cost(s2_upper[j - 1], s2_upper[j]);
        }

        for i in 1..len1 {
            for j in 1..len2 {
                d[i][j] = [
                    d[i - 1][j] + self.d_cost(s1_upper[i - 1], s1_upper[i]),
                    d[i][j - 1] + self.d_cost(s2_upper[j - 1], s2_upper[j]),
                    d[i - 1][j - 1] + self.r_cost(s1_upper[i], s2_upper[j]),
                ]
                .into_iter()
                .min()
                .unwrap();
            }
        }

        d[len1 - 1][len2 - 1].min(max_length)
    }

    pub fn similarity(&self, s1: &str, s2: &str) -> f64 {
        let max_len = (s1.chars().count().max(s2.chars().count()) * self.mismatch_cost as usize) as f64;
        max_len - self.distance(s1, s2) as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_editex() {
        let alg = Editex::new();
        assert_eq!(alg.distance("", ""), 0);
        assert_eq!(alg.distance("ab", "a"), 2);
        assert_eq!(alg.distance("ab", "c"), 4);
        assert_eq!(alg.distance("nelson", "neilsen"), 2);
        assert_eq!(alg.distance("niall", "neal"), 1);
    }

    #[test]
    fn test_editex_local() {
        let alg = Editex::with_local(true);
        assert_eq!(alg.distance("", ""), 0);
        assert_eq!(alg.distance("ab", "a"), 2);
        assert_eq!(alg.distance("nelson", "neilsen"), 2);
        assert_eq!(alg.distance("niall", "neal"), 1);
    }
}
