use clap::{Parser, ValueEnum};
use pimp_my_peer::strategy::*;
use pimp_my_peer::{pimp, Keywords};

fn parse_hex(s: &str) -> Result<[u8; 32], String> {
    let mut bytes = [0u8; 32];
    hex::decode_to_slice(s, &mut bytes).map_err(|_| "Invalid hex string".to_string())?;
    Ok(bytes)
}

#[derive(ValueEnum, Clone, PartialEq)]
enum Method {
    Identity,
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

    #[arg(
        long,
        help = "Maximum number of iterations",
        default_value_t = 100_000_000
    )]
    iters: usize,

    #[arg(long, help = "A private key, can be used as seed", value_parser = parse_hex, default_value = "0000000000000000000000000000000000000000000000000000000000000001")]
    seed: [u8; 32],

    #[arg(short, help = "A starting keyword")]
    starts_with: Vec<String>,

    #[arg(short, help = "A contained keyword")]
    contains: Vec<String>,

    #[arg(short, help = "An ending keyword")]
    ends_with: Vec<String>,
}

fn main() {
    let parsed = Cli::parse();

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

    let (elapsed, result) = match parsed.method {
        Method::Linear => pimp(LinearStrategy::new(parsed.iters, &parsed.seed), &keywords),
        Method::Random => pimp(RandomStrategy::new(parsed.iters), &keywords),
        Method::LinearMemo => pimp(
            LinearMemoStrategy::new(parsed.iters, &parsed.seed),
            &keywords,
        ),
        Method::Identity => pimp(IdentityStrategy::new(&parsed.seed), &keywords),
    };

    match result {
        Some((secret_key, _, peer_id)) => {
            println!("PeerID:     {}", peer_id);
            println!("Secret key: {}", hex::encode(secret_key.to_bytes()));
        }
        None => println!("Could not find a peer_id containing the keyword",),
    }
    println!("That took {:?}", elapsed);
}
