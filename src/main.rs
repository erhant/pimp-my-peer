use pimp_my_peer::PimpMyPeer;

/// Maximum number of iterations allowed.
const MAX_ITERS: usize = 10_000_000;

fn main() {
    const ENDS_WITH: &str = "ac";

    // setup
    let mut pmp = PimpMyPeer::new();
    pmp.ends_with(ENDS_WITH);
    println!("{:#?}", pmp);

    let (elapsed, result) = pmp.crunch(MAX_ITERS);
    match result {
        Some((secret_key, _, peer_id)) => {
            println!("PeerID:     {}", peer_id);
            println!("Secret key: {}", hex::encode(secret_key.to_bytes()));
        }
        None => println!(
            "Could not find a peer_id containing the keyword: {}",
            ENDS_WITH
        ),
    }
    println!("That took {:?}", elapsed);
}

#[cfg(test)]
mod tests {

    /// this can be added as extra
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

// 106e1a92b129016c77a42e32937f5b18abbd86cbd1efa1ac1411fb2be1a48a79
// cd7359c362d4d829fd03b6ace4a9c8728d5f0b150ee93c76d5a29741f98c765f
