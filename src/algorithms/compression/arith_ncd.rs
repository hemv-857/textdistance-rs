use num_bigint::{BigInt, Sign};
use num_traits::{ToPrimitive, Zero};
use std::collections::HashMap;

pub struct ArithNCD {
    base: f64,
}

impl Default for ArithNCD {
    fn default() -> Self {
        Self { base: 2.0 }
    }
}

#[derive(Clone)]
#[allow(dead_code)]
struct Fraction {
    numer: BigInt,
    denom: BigInt,
}

#[allow(dead_code)]
impl Fraction {
    fn new(numer: BigInt, denom: BigInt) -> Self {
        Self { numer, denom }
    }

    fn from_integer(n: BigInt) -> Self {
        Self {
            numer: n,
            denom: BigInt::from(1),
        }
    }

    fn zero() -> Self {
        Self::from_integer(BigInt::zero())
    }

    fn one() -> Self {
        Self::from_integer(BigInt::from(1))
    }

    fn is_zero(&self) -> bool {
        self.numer.is_zero()
    }

    fn floor(&self) -> Self {
        if self.denom.is_zero() || self.denom.sign() == Sign::Minus {
            panic!("floor of invalid fraction");
        }
        let q = &self.numer / &self.denom;
        if self.numer.sign() == Sign::Minus && !self.numer.is_zero() {
            let rem = &self.numer % &self.denom;
            if !rem.is_zero() {
                return Self::from_integer(q - BigInt::from(1));
            }
        }
        Self::from_integer(q)
    }

    fn to_f64(&self) -> f64 {
        let n = self.numer.to_f64().unwrap_or(0.0);
        let d = self.denom.to_f64().unwrap_or(1.0);
        n / d
    }
}

impl std::ops::Add for Fraction {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        let numer = &self.numer * &rhs.denom + &rhs.numer * &self.denom;
        let denom = &self.denom * &rhs.denom;
        Self::new(numer, denom)
    }
}

impl std::ops::Sub for Fraction {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        let numer = &self.numer * &rhs.denom - &rhs.numer * &self.denom;
        let denom = &self.denom * &rhs.denom;
        Self::new(numer, denom)
    }
}

impl std::ops::Mul for Fraction {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self::new(&self.numer * &rhs.numer, &self.denom * &rhs.denom)
    }
}

impl std::ops::Div for Fraction {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        Self::new(&self.numer * &rhs.denom, &self.denom * &rhs.numer)
    }
}

impl PartialOrd for Fraction {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let a = &self.numer * &other.denom;
        let b = &other.numer * &self.denom;
        Some(a.cmp(&b))
    }
}

impl PartialEq for Fraction {
    fn eq(&self, other: &Self) -> bool {
        &self.numer * &other.denom == &other.numer * &self.denom
    }
}

impl ArithNCD {
    pub fn new() -> Self {
        Self::default()
    }

    fn char_counts(s: &str) -> HashMap<char, usize> {
        let mut counts = HashMap::new();
        for ch in s.chars() {
            *counts.entry(ch).or_insert(0) += 1;
        }
        counts
    }

    fn make_probs(data: &str) -> (Vec<char>, Vec<Fraction>, Fraction) {
        let counts = Self::char_counts(data);
        let total = BigInt::from(counts.values().sum::<usize>());
        let mut sorted: Vec<(char, usize)> = counts.into_iter().collect();
        sorted.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));

        let mut chars = Vec::new();
        let mut starts = Vec::new();
        let mut cum = BigInt::zero();
        for (ch, count) in &sorted {
            chars.push(*ch);
            starts.push(Fraction::new(cum.clone(), total.clone()));
            cum += BigInt::from(*count);
        }
        (chars, starts, Fraction::from_integer(total))
    }

    fn get_range(
        data: &str,
        chars: &[char],
        starts: &[Fraction],
        total: &Fraction,
    ) -> (Fraction, Fraction) {
        let mut start = Fraction::zero();
        let mut width = Fraction::one();
        let counts = Self::char_counts(data);

        for ch in data.chars() {
            if let Some(idx) = chars.iter().position(|c| *c == ch) {
                start = start + width.clone() * starts[idx].clone();
                width = width * Fraction::from_integer(BigInt::from(counts[&ch])) / total.clone();
            }
        }
        (start.clone(), start + width)
    }

    fn compress_fraction(data: &str) -> Fraction {
        let (chars, starts, total) = Self::make_probs(data);
        if chars.is_empty() || total.is_zero() {
            return Fraction::zero();
        }
        let (s, e) = Self::get_range(data, &chars, &starts, &total);

        let mut num = Fraction::one();
        let mut den_int = BigInt::from(1);
        let one_int = BigInt::from(1);
        for _ in 0..64 {
            if s <= num && num < e {
                return num;
            }
            // num_numerator = 1 + floor(s.numer * den_int / s.denom)
            let s_num = &s.numer * &den_int;
            let floor_val = &s_num / &s.denom;
            let num_numerator = one_int.clone() + floor_val;
            num = Fraction::new(num_numerator, den_int.clone());
            den_int *= 2;
        }
        num
    }

    fn compress_size(&self, data: &str) -> f64 {
        let f = Self::compress_fraction(data);
        let numer = f.numer.to_f64().unwrap_or(0.0);
        if numer <= 0.0 {
            0.0
        } else {
            numer.log(self.base).ceil()
        }
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
        1.0 - self.distance(s1, s2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arith_ncd_monotonicity() {
        let alg = ArithNCD::new();
        let same = alg.distance("test", "test");
        let similar = alg.distance("test", "text");
        let diff = alg.distance("test", "nani");
        assert!(same <= similar, "same={} > similar={}", same, similar);
        assert!(similar <= diff, "similar={} > diff={}", similar, diff);
    }
}
