use criterion::{black_box, criterion_group, criterion_main, Criterion};
use textdistance::algorithms::edit::*;
use textdistance::algorithms::token::*;
use textdistance::algorithms::sequence::*;
use textdistance::algorithms::compression::*;
use textdistance::algorithms::phonetic::*;
use textdistance::Distance;
use textdistance::Similarity;

fn bench_edit_algorithms(c: &mut Criterion) {
    let mut group = c.benchmark_group("edit");
    let pairs = vec![
        ("test", "text"),
        ("hello", "world"),
        ("algorithm", "altruistic"),
        ("sunday", "saturday"),
        ("kitten", "sitting"),
    ];

    for (s1, s2) in &pairs {
        let c1: Vec<char> = s1.chars().collect();
        let c2: Vec<char> = s2.chars().collect();
        let label = format!("{}_{}", s1, s2);

        group.bench_function(format!("levenshtein_{}", &label), |b| {
            b.iter(|| black_box(levenshtein::Levenshtein.distance(&c1, &c2)))
        });
        group.bench_function(format!("hamming_{}", &label), |b| {
            b.iter(|| black_box(hamming::Hamming.distance(&c1, &c2)))
        });
        group.bench_function(format!("damerau_levenshtein_{}", &label), |b| {
            b.iter(|| black_box(damerau_levenshtein::DamerauLevenshtein::new().distance(&c1, &c2)))
        });
        group.bench_function(format!("jaro_{}", &label), |b| {
            b.iter(|| black_box(jaro_winkler::JaroWinkler::new().similarity(&c1, &c2)))
        });
        group.bench_function(format!("jaro_winkler_{}", &label), |b| {
            b.iter(|| black_box(jaro_winkler::JaroWinkler::new().similarity(&c1, &c2)))
        });
    }
    group.finish();
}

fn bench_token_algorithms(c: &mut Criterion) {
    let mut group = c.benchmark_group("token");
    let pairs = vec![
        ("test", "text"),
        ("nelson", "neilsen"),
        ("hello world", "world hello"),
    ];

    for (s1, s2) in &pairs {
        let label = format!("{}_{}", s1, s2);
        group.bench_function(format!("jaccard_{}", &label), |b| {
            b.iter(|| black_box(jaccard::Jaccard::new().similarity(s1, s2)))
        });
        group.bench_function(format!("cosine_{}", &label), |b| {
            b.iter(|| black_box(cosine::Cosine::new().similarity(s1, s2)))
        });
        group.bench_function(format!("monge_elkan_{}", &label), |b| {
            b.iter(|| black_box(monge_elkan::MongeElkan::new().similarity(s1, s2)))
        });
    }
    group.finish();
}

fn bench_compression(c: &mut Criterion) {
    let mut group = c.benchmark_group("compression");
    let pairs = vec![("test", "text"), ("hello", "world")];

    for (s1, s2) in &pairs {
        let label = format!("{}_{}", s1, s2);
        group.bench_function(format!("bz2_ncd_{}", &label), |b| {
            b.iter(|| black_box(bz2_ncd::BZ2NCD::new().distance(s1, s2)))
        });
        group.bench_function(format!("zlib_ncd_{}", &label), |b| {
            b.iter(|| black_box(zlib_ncd::ZLIBNCD::new().distance(s1, s2)))
        });
    }
    group.finish();
}

fn bench_phonetic(c: &mut Criterion) {
    let mut group = c.benchmark_group("phonetic");
    let pairs = vec![("MARTHA", "MARHTA"), ("test", "text")];

    for (s1, s2) in &pairs {
        let label = format!("{}_{}", s1, s2);
        group.bench_function(format!("mra_{}", &label), |b| {
            b.iter(|| black_box(mra::MRA::new().distance(s1, s2)))
        });
        group.bench_function(format!("editex_{}", &label), |b| {
            b.iter(|| black_box(editex::Editex::new().distance(s1, s2)))
        });
    }
    group.finish();
}

fn bench_large_strings(c: &mut Criterion) {
    let mut group = c.benchmark_group("large_strings");

    let large1: String = (0..1000).map(|i| (b'a' + (i % 26) as u8) as char).collect();
    let large2: String = (0..1000).map(|i| (b'a' + ((i + 5) % 26) as u8) as char).collect();
    let c1: Vec<char> = large1.chars().collect();
    let c2: Vec<char> = large2.chars().collect();

    group.bench_function("levenshtein_1000", |b| {
        b.iter(|| black_box(levenshtein::Levenshtein.distance(&c1, &c2)))
    });
    group.bench_function("hamming_1000", |b| {
        b.iter(|| black_box(hamming::Hamming.distance(&c1, &c2)))
    });
    group.bench_function("jaro_winkler_1000", |b| {
        b.iter(|| black_box(jaro_winkler::JaroWinkler::new().similarity(&c1, &c2)))
    });
    group.bench_function("jaccard_1000", |b| {
        b.iter(|| black_box(jaccard::Jaccard::new().similarity(&large1, &large2)))
    });
    group.bench_function("bz2_ncd_1000", |b| {
        b.iter(|| black_box(bz2_ncd::BZ2NCD::new().distance(&large1, &large2)))
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_edit_algorithms,
    bench_token_algorithms,
    bench_compression,
    bench_phonetic,
    bench_large_strings,
);
criterion_main!(benches);
