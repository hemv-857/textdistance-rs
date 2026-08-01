use std::time::Instant;
use textdistance::algorithms::edit::*;
use textdistance::algorithms::token::*;
use textdistance::algorithms::sequence::*;
use textdistance::algorithms::phonetic::*;
use textdistance::algorithms::simple::*;
use textdistance::Distance;
use textdistance::Similarity;

fn bench<F: Fn() -> R, R>(name: &str, f: F, iterations: u32) {
    for _ in 0..100 { f(); }
    let mut times = Vec::new();
    for _ in 0..iterations {
        let start = Instant::now();
        for _ in 0..1000 { f(); }
        let elapsed = start.elapsed().as_nanos() as f64 / 1000.0;
        times.push(elapsed);
    }
    times.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let p50 = times[times.len() / 2];
    println!("  {:40} {:>10.1} ns", name, p50);
}

fn main() {
    let pairs = vec![
        ("test", "text"),
        ("hello", "world"),
        ("algorithm", "altruistic"),
        ("sunday", "saturday"),
        ("kitten", "sitting"),
    ];
    
    println!("Rust Algorithm Benchmarks (p50 per call, ns)");
    println!("=============================================");
    
    // Edit
    println!("\nEdit-based:");
    for (s1, s2) in &pairs {
        let c1: Vec<char> = s1.chars().collect();
        let c2: Vec<char> = s2.chars().collect();
        let label = format!("{}/{}", s1, s2);
        bench(&format!("levenshtein_{}", label), || levenshtein::Levenshtein.distance(&c1, &c2), 200);
        bench(&format!("hamming_{}", label), || hamming::Hamming.distance(&c1, &c2), 200);
        bench(&format!("damerau_levenshtein_{}", label), || damerau_levenshtein::DamerauLevenshtein::new().distance(&c1, &c2), 200);
        bench(&format!("jaro_{}", label), || jaro::Jaro::new().similarity(&c1, &c2), 200);
        bench(&format!("jaro_winkler_{}", label), || jaro_winkler::JaroWinkler::new().similarity(&c1, &c2), 200);
        bench(&format!("strcmp95_{}", label), || strcmp95::StrCmp95::new().similarity(&c1, &c2), 200);
        bench(&format!("mlipns_{}", label), || mlipns::MLIPNS::new().similarity(&c1, &c2), 200);
        bench(&format!("needleman_wunsch_{}", label), || needleman_wunsch::NeedlemanWunsch::new().similarity(&c1, &c2), 200);
        bench(&format!("smith_waterman_{}", label), || smith_waterman::SmithWaterman::new().similarity(&c1, &c2), 200);
        bench(&format!("gotoh_{}", label), || gotoh::Gotoh::new().similarity(&c1, &c2), 200);
    }
    
    // Token
    println!("\nToken-based:");
    for (s1, s2) in &pairs {
        let label = format!("{}/{}", s1, s2);
        bench(&format!("jaccard_{}", label), || jaccard::Jaccard::new().similarity(s1, s2), 200);
        bench(&format!("sorensen_{}", label), || sorensen::Sorensen::new().similarity(s1, s2), 200);
        bench(&format!("cosine_{}", label), || cosine::Cosine::new().similarity(s1, s2), 200);
        bench(&format!("monge_elkan_{}", label), || monge_elkan::MongeElkan::new().similarity(s1, s2), 200);
        bench(&format!("tversky_{}", label), || tversky::Tversky::new().similarity(s1, s2), 200);
        bench(&format!("overlap_{}", label), || overlap::Overlap::new().similarity(s1, s2), 200);
        bench(&format!("tanimoto_{}", label), || tanimoto::Tanimoto::default().similarity(s1, s2), 200);
        bench(&format!("bag_{}", label), || bag::Bag::new().distance(s1, s2), 200);
    }
    
    // Sequence
    println!("\nSequence-based:");
    for (s1, s2) in &pairs {
        let label = format!("{}/{}", s1, s2);
        bench(&format!("lcsseq_{}", label), || lcsseq::LCSSeq::new().similarity(s1, s2), 200);
        bench(&format!("lcsstr_{}", label), || lcsstr::LCSStr::new().similarity(s1, s2), 200);
        bench(&format!("ratcliff_obershelp_{}", label), || ratcliff_obershelp::RatcliffObershelp::new().similarity(s1, s2), 200);
    }
    
    // Simple
    println!("\nSimple:");
    for (s1, s2) in &pairs {
        let label = format!("{}/{}", s1, s2);
        bench(&format!("length_{}", label), || length::Length::new().distance(s1, s2), 200);
    }
    
    // Phonetic
    println!("\nPhonetic:");
    for (s1, s2) in &pairs {
        let label = format!("{}/{}", s1, s2);
        bench(&format!("mra_{}", label), || mra::MRA::new().distance(s1, s2), 200);
        bench(&format!("editex_{}", label), || editex::Editex::new().distance(s1, s2), 200);
    }
}
