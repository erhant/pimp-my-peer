use clap::{Parser, ValueEnum};
use pimp_my_peer::strategy::*;
use pimp_my_peer::{pimp, Keywords};

#[derive(ValueEnum, Clone, PartialEq)]
enum Method {
    Info,
    Random,
    Linear,
    LinearMemo,
}

// https://docs.rs/clap/latest/clap/_derive/index.html#arg-attributes
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long, help = "Method to use")]
    method: Method,

    // #[arg(short, long, help = "A private key, can be used as seed")]
    // key: [u8; 32],
    #[arg(
        short,
        long,
        help = "Maximum number of iterations",
        default_value_t = 100_000_000
    )]
    iters: usize,

    #[arg(short, help = "A starting keyword")]
    starts_with: Vec<String>,

    #[arg(short, help = "An ending keyword")]
    ends_with: Vec<String>,

    #[arg(short, help = "A contained keyword")]
    contains: Vec<String>,
}

fn main() {
    let parsed = Cli::parse();

    // define keywrods
    let mut keywords = Keywords::new();
    for keyword in &parsed.starts_with {
        keywords = keywords.starts_with(keyword);
    }
    for keyword in &parsed.ends_with {
        keywords = keywords.ends_with(keyword);
    }
    for keyword in &parsed.contains {
        keywords = keywords.contains(keyword);
    }

    println!("Keywords: {:#?}", keywords);

    if parsed.method == Method::Info {
        todo!()
    } else {
        let strategy: Strategy = match parsed.method {
            Method::Linear => Strategy::Linear(LinearStrategy::new(parsed.iters, &[0; 32])),
            Method::Random => Strategy::Random(RandomStrategy::new(parsed.iters)),
            Method::LinearMemo => {
                Strategy::LinearMemo(LinearMemoStrategy::new(parsed.iters, &[0; 32]))
            }
            _ => todo!(),
        };

        let (elapsed, result) = pimp(strategy, &keywords);
        match result {
            Some((secret_key, _, peer_id)) => {
                println!("PeerID:     {}", peer_id);
                println!("Secret key: {}", hex::encode(secret_key.to_bytes()));
            }
            None => println!("Could not find a peer_id containing the keyword",),
        }
        println!("That took {:?}", elapsed);
    }
}
