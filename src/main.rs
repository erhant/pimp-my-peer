use pimp_my_peer::strategy::*;
use pimp_my_peer::{pimp, Keywords};

/// Maximum number of iterations allowed.
const MAX_ITERS: usize = 20; // 10_000_000;

fn main() {
    // define keywrods
    let keywords = Keywords::new().ends_with("61");

    // choose strategy
    let _random_strategy = RandomStrategy::new(MAX_ITERS);
    let _linear_strategy = LinearStrategy::new(MAX_ITERS, &[0x61u8; 32]);
    let _linear_memo_strategy = LinearMemoStrategy::new(MAX_ITERS, &[0x61u8; 32]);
    let strategy = _linear_memo_strategy;
    println!("Using strategy: {:#?}", strategy);

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

#[cfg(test)]
mod tests {

    /// TODO: add as CLI function
    #[test]
    fn test_pub_to_peer() {
        let public_key_raw =
            hex_literal::hex!("03106e1a92b129016c77a42e32937f5b18abbd86cbd1efa1ac1411fb2be1a48a79");
        let peer_id = libp2p_identity::PublicKey::from(
            libp2p_identity::secp256k1::PublicKey::try_from_bytes(&public_key_raw).unwrap(),
        )
        .to_peer_id();
        println!("{}", peer_id);
    }
}
