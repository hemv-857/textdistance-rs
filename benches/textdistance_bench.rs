use criterion::{black_box, criterion_group, criterion_main, Criterion};
use textdistance::algorithms::edit::*;
use textdistance::algorithms::token::*;
use textdistance::algorithms::sequence::*;
use textdistance::algorithms::compression::*;
use textdistance::algorithms::phonetic::*;
use textdistance::algorithms::simple::*;
use textdistance::Distance;
use textdistance::Similarity;

fn bench_all(c: &mut Criterion) {
    let pairs = vec![
        ("test", "text"),
        ("hello", "world"),
        ("algorithm", "altruistic"),
        ("sunday", "saturday"),
        ("kitten", "sitting"),
    ];

    // Edit algorithms
    let mut edit = c.benchmark_group("edit");
    for (s1, s2) in &pairs {
        let c1: Vec<char> = s1.chars().collect();
        let c2: Vec<char> = s2.chars().collect();
        let label = format!("{}_{}", s1, s2);
        edit.bench_function(format!("levenshtein_{}", &label), |b| {
            b.iter(|| black_box(levenshtein::Levenshtein.distance(&c1, &c2)))
        });
        edit.bench_function(format!("hamming_{}", &label), |b| {
            b.iter(|| black_box(hamming::Hamming.distance(&c1, &c2)))
        });
        edit.bench_function(format!("damerau_levenshtein_{}", &label), |b| {
            b.iter(|| black_box(damerau_levenshtein::DamerauLevenshtein::new().distance(&c1, &c2)))
        });
        edit.bench_function(format!("jaro_{}", &label), |b| {
            b.iter(|| black_box(jaro::Jaro::new().similarity(&c1, &c2)))
        });
        edit.bench_function(format!("jaro_winkler_{}", &label), |b| {
            b.iter(|| black_box(jaro_winkler::JaroWinkler::new().similarity(&c1, &c2)))
        });
        edit.bench_function(format!("strcmp95_{}", &label), |b| {
            b.iter(|| black_box(strcmp95::StrCmp95::new().similarity(&c1, &c2)))
        });
        edit.bench_function(format!("mlipns_{}", &label), |b| {
            b.iter(|| black_box(mlipns::MLIPNS::new().distance(&c1, &c2)))
        });
        edit.bench_function(format!("needleman_wunsch_{}", &label), |b| {
            b.iter(|| black_box(needleman_wunsch::NeedlemanWunsch::new().distance(&c1, &c2)))
        });
        edit.bench_function(format!("smith_waterman_{}", &label), |b| {
            b.iter(|| black_box(smith_waterman::SmithWaterman::new().distance(&c1, &c2)))
        });
        edit.bench_function(format!("gotoh_{}", &label), |b| {
            b.iter(|| black_box(gotoh::Gotoh::new().distance(&c1, &c2)))
        });
    }
    edit.finish();

    // Token algorithms
    let mut token = c.benchmark_group("token");
    for (s1, s2) in &pairs {
        let label = format!("{}_{}", s1, s2);
        token.bench_function(format!("jaccard_{}", &label), |b| {
            b.iter(|| black_box(jaccard::Jaccard::new().similarity(s1, s2)))
        });
        token.bench_function(format!("sorensen_{}", &label), |b| {
            b.iter(|| black_box(sorensen::Sorensen::new().similarity(s1, s2)))
        });
        token.bench_function(format!("tversky_{}", &label), |b| {
            b.iter(|| black_box(tversky::Tversky::new().similarity(s1, s2)))
        });
        token.bench_function(format!("overlap_{}", &label), |b| {
            b.iter(|| black_box(overlap::Overlap::new().similarity(s1, s2)))
        });
        token.bench_function(format!("cosine_{}", &label), |b| {
            b.iter(|| black_box(cosine::Cosine::new().similarity(s1, s2)))
        });
        token.bench_function(format!("tanimoto_{}", &label), |b| {
            b.iter(|| black_box(tanimoto::Tanimoto::new().similarity(s1, s2)))
        });
        token.bench_function(format!("monge_elkan_{}", &label), |b| {
            b.iter(|| black_box(monge_elkan::MongeElkan::new().similarity(s1, s2)))
        });
        token.bench_function(format!("bag_{}", &label), |b| {
            b.iter(|| black_box(bag::Bag::new().distance(s1, s2)))
        });
    }
    token.finish();

    // Sequence algorithms
    let mut seq = c.benchmark_group("sequence");
    for (s1, s2) in &pairs {
        let label = format!("{}_{}", s1, s2);
        seq.bench_function(format!("lcsseq_{}", &label), |b| {
            b.iter(|| black_box(lcsseq::LCSSeq::new().similarity(s1, s2)))
        });
        seq.bench_function(format!("lcsstr_{}", &label), |b| {
            b.iter(|| black_box(lcsstr::LCSStr::new().similarity(s1, s2)))
        });
        seq.bench_function(format!("ratcliff_obershelp_{}", &label), |b| {
            b.iter(|| black_box(ratcliff_obershelp::RatcliffObershelp::new().similarity(s1, s2)))
        });
    }
    seq.finish();

    // Simple algorithms
    let mut simple = c.benchmark_group("simple");
    for (s1, s2) in &pairs {
        let label = format!("{}_{}", s1, s2);
        simple.bench_function(format!("prefix_{}", &label), |b| {
            b.iter(|| black_box(prefix::Prefix::new().similarity(s1, s2)))
        });
        simple.bench_function(format!("postfix_{}", &label), |b| {
            b.iter(|| black_box(postfix::Postfix::new().similarity(s1, s2)))
        });
        simple.bench_function(format!("length_{}", &label), |b| {
            b.iter(|| black_box(length::Length::new().distance(s1, s2)))
        });
    }
    simple.finish();

    // Phonetic algorithms
    let mut phonetic = c.benchmark_group("phonetic");
    for (s1, s2) in &pairs {
        let label = format!("{}_{}", s1, s2);
        phonetic.bench_function(format!("mra_{}", &label), |b| {
            b.iter(|| black_box(mra::MRA::new().distance(s1, s2)))
        });
        phonetic.bench_function(format!("editex_{}", &label), |b| {
            b.iter(|| black_box(editex::Editex::new().distance(s1, s2)))
        });
    }
    phonetic.finish();

    // Compression algorithms
    let mut comp = c.benchmark_group("compression");
    for (s1, s2) in &pairs {
        let label = format!("{}_{}", s1, s2);
        comp.bench_function(format!("bz2_ncd_{}", &label), |b| {
            b.iter(|| black_box(bz2_ncd::BZ2NCD::new().distance(s1, s2)))
        });
        comp.bench_function(format!("lzma_ncd_{}", &label), |b| {
            b.iter(|| black_box(lzma_ncd::LZMANCD::new().distance(s1, s2)))
        });
        comp.bench_function(format!("zlib_ncd_{}", &label), |b| {
            b.iter(|| black_box(zlib_ncd::ZLIBNCD::new().distance(s1, s2)))
        });
    }
    comp.finish();
}

criterion_group!(benches, bench_all);
criterion_main!(benches);
