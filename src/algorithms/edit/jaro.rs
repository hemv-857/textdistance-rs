pub use super::jaro_winkler::JaroWinkler as Jaro;
use crate::Similarity;

pub fn similarity_str(s1: &str, s2: &str) -> f64 {
    let c1: Vec<char> = s1.chars().collect();
    let c2: Vec<char> = s2.chars().collect();
    Jaro::jaro().similarity(&c1, &c2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Similarity;

    #[test]
    fn test_jaro() {
        let alg = Jaro::jaro();
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
