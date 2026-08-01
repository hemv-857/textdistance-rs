use clap::{Parser, Subcommand};
use textdistance::{Distance, Similarity};

mod edit {
    pub use textdistance::algorithms::edit::*;
}
mod token {
    pub use textdistance::algorithms::token::*;
}
mod seq {
    pub use textdistance::algorithms::sequence::*;
}
mod simple {
    pub use textdistance::algorithms::simple::*;
}
mod phonetic {
    pub use textdistance::algorithms::phonetic::*;
}
mod compression {
    pub use textdistance::algorithms::compression::*;
}

#[derive(Parser)]
#[command(name = "textdistance")]
#[command(about = "Compute distance between sequences")]
#[command(version = "4.6.2")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Hamming { s1: String, s2: String },
    Levenshtein { s1: String, s2: String },
    DamerauLevenshtein { s1: String, s2: String },
    Jaro { s1: String, s2: String },
    JaroWinkler { s1: String, s2: String },
    StrCmp95 { s1: String, s2: String },
    Mlipns { s1: String, s2: String },
    NeedlemanWunsch { s1: String, s2: String },
    SmithWaterman { s1: String, s2: String },
    Gotoh { s1: String, s2: String },
    Jaccard { s1: String, s2: String },
    Sorensen { s1: String, s2: String },
    Tversky { s1: String, s2: String },
    Overlap { s1: String, s2: String },
    Cosine { s1: String, s2: String },
    Tanimoto { s1: String, s2: String },
    MongeElkan { s1: String, s2: String },
    Bag { s1: String, s2: String },
    Lcsseq { s1: String, s2: String },
    Lcsstr { s1: String, s2: String },
    RatcliffObershelp { s1: String, s2: String },
    Prefix { s1: String, s2: String },
    Postfix { s1: String, s2: String },
    Length { s1: String, s2: String },
    Identity { s1: String, s2: String },
    Matrix { s1: String, s2: String },
    Mra { s1: String, s2: String },
    Editex { s1: String, s2: String },
    ArithNcd { s1: String, s2: String },
    RleNcd { s1: String, s2: String },
    BwtrleNcd { s1: String, s2: String },
    SqrtNcd { s1: String, s2: String },
    EntropyNcd { s1: String, s2: String },
    Bz2Ncd { s1: String, s2: String },
    LzmaNcd { s1: String, s2: String },
    ZlibNcd { s1: String, s2: String },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Hamming { s1, s2 } => {
            let c1: Vec<char> = s1.chars().collect();
            let c2: Vec<char> = s2.chars().collect();
            println!("{}", edit::hamming::Hamming.distance(&c1, &c2));
        }
        Commands::Levenshtein { s1, s2 } => {
            let c1: Vec<char> = s1.chars().collect();
            let c2: Vec<char> = s2.chars().collect();
            println!("{}", edit::levenshtein::Levenshtein.distance(&c1, &c2));
        }
        Commands::DamerauLevenshtein { s1, s2 } => {
            let c1: Vec<char> = s1.chars().collect();
            let c2: Vec<char> = s2.chars().collect();
            println!("{}", edit::damerau_levenshtein::DamerauLevenshtein::new().distance(&c1, &c2));
        }
        Commands::Jaro { s1, s2 } => {
            let c1: Vec<char> = s1.chars().collect();
            let c2: Vec<char> = s2.chars().collect();
            println!("{}", edit::jaro::Jaro::jaro().similarity(&c1, &c2));
        }
        Commands::JaroWinkler { s1, s2 } => {
            let c1: Vec<char> = s1.chars().collect();
            let c2: Vec<char> = s2.chars().collect();
            println!("{}", edit::jaro_winkler::JaroWinkler::new().similarity(&c1, &c2));
        }
        Commands::StrCmp95 { s1, s2 } => {
            let c1: Vec<char> = s1.chars().collect();
            let c2: Vec<char> = s2.chars().collect();
            println!("{}", edit::strcmp95::StrCmp95::new().similarity(&c1, &c2));
        }
        Commands::Mlipns { s1, s2 } => {
            let c1: Vec<char> = s1.chars().collect();
            let c2: Vec<char> = s2.chars().collect();
            println!("{}", edit::mlipns::MLIPNS::new().similarity(&c1, &c2));
        }
        Commands::NeedlemanWunsch { s1, s2 } => {
            let c1: Vec<char> = s1.chars().collect();
            let c2: Vec<char> = s2.chars().collect();
            println!("{}", edit::needleman_wunsch::NeedlemanWunsch::new().similarity(&c1, &c2));
        }
        Commands::SmithWaterman { s1, s2 } => {
            let c1: Vec<char> = s1.chars().collect();
            let c2: Vec<char> = s2.chars().collect();
            println!("{}", edit::smith_waterman::SmithWaterman::new().similarity(&c1, &c2));
        }
        Commands::Gotoh { s1, s2 } => {
            let c1: Vec<char> = s1.chars().collect();
            let c2: Vec<char> = s2.chars().collect();
            println!("{}", edit::gotoh::Gotoh::new().similarity(&c1, &c2));
        }
        Commands::Jaccard { s1, s2 } => {
            println!("{}", token::jaccard::Jaccard::new().similarity(&s1, &s2));
        }
        Commands::Sorensen { s1, s2 } => {
            println!("{}", token::sorensen::Sorensen::new().similarity(&s1, &s2));
        }
        Commands::Tversky { s1, s2 } => {
            println!("{}", token::tversky::Tversky::new().similarity(&s1, &s2));
        }
        Commands::Overlap { s1, s2 } => {
            println!("{}", token::overlap::Overlap::new().similarity(&s1, &s2));
        }
        Commands::Cosine { s1, s2 } => {
            println!("{}", token::cosine::Cosine::new().similarity(&s1, &s2));
        }
        Commands::Tanimoto { s1, s2 } => {
            println!("{}", token::tanimoto::Tanimoto::new().similarity(&s1, &s2));
        }
        Commands::MongeElkan { s1, s2 } => {
            println!("{}", token::monge_elkan::MongeElkan::new().similarity(&s1, &s2));
        }
        Commands::Bag { s1, s2 } => {
            println!("{}", token::bag::Bag::new().distance(&s1, &s2));
        }
        Commands::Lcsseq { s1, s2 } => {
            println!("{}", seq::lcsseq::LCSSeq::new().similarity(&s1, &s2));
        }
        Commands::Lcsstr { s1, s2 } => {
            println!("{}", seq::lcsstr::LCSStr::new().similarity(&s1, &s2));
        }
        Commands::RatcliffObershelp { s1, s2 } => {
            println!("{}", seq::ratcliff_obershelp::RatcliffObershelp::new().similarity(&s1, &s2));
        }
        Commands::Prefix { s1, s2 } => {
            println!("{}", simple::prefix::Prefix::new().similarity(&s1, &s2));
        }
        Commands::Postfix { s1, s2 } => {
            println!("{}", simple::postfix::Postfix::new().similarity(&s1, &s2));
        }
        Commands::Length { s1, s2 } => {
            println!("{}", simple::length::Length::new().distance(&s1, &s2));
        }
        Commands::Identity { s1, s2 } => {
            println!("{}", simple::identity::Identity::new().similarity(&s1, &s2));
        }
        Commands::Matrix { s1, s2 } => {
            println!("{}", simple::matrix::Matrix::new().similarity(&s1, &s2));
        }
        Commands::Mra { s1, s2 } => {
            println!("{}", phonetic::mra::MRA::new().distance(&s1, &s2));
        }
        Commands::Editex { s1, s2 } => {
            println!("{}", phonetic::editex::Editex::new().distance(&s1, &s2));
        }
        Commands::ArithNcd { s1, s2 } => {
            println!("{}", compression::arith_ncd::ArithNCD::new().distance(&s1, &s2));
        }
        Commands::RleNcd { s1, s2 } => {
            println!("{}", compression::rle_ncd::RLENCD::new().distance(&s1, &s2));
        }
        Commands::BwtrleNcd { s1, s2 } => {
            println!("{}", compression::bwtrle_ncd::BWTRLENCD::new().distance(&s1, &s2));
        }
        Commands::SqrtNcd { s1, s2 } => {
            println!("{}", compression::sqrt_ncd::SqrtNCD::new().distance(&s1, &s2));
        }
        Commands::EntropyNcd { s1, s2 } => {
            println!("{}", compression::entropy_ncd::EntropyNCD::new().distance(&s1, &s2));
        }
        Commands::Bz2Ncd { s1, s2 } => {
            println!("{}", compression::bz2_ncd::BZ2NCD::new().distance(&s1, &s2));
        }
        Commands::LzmaNcd { s1, s2 } => {
            println!("{}", compression::lzma_ncd::LZMANCD::new().distance(&s1, &s2));
        }
        Commands::ZlibNcd { s1, s2 } => {
            println!("{}", compression::zlib_ncd::ZLIBNCD::new().distance(&s1, &s2));
        }
    }
}
