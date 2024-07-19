use rayon::prelude::*;

fn main() {
    let keyword = "dria";

    const N: usize = 10_000_000;
    println!("Starting search for keyword: {}", keyword);
    let start_time = std::time::Instant::now();

    let ans = (0..N).into_par_iter().find_map_first(|_| {
        // generates a random keypair
        let secret_key = libp2p::identity::secp256k1::SecretKey::generate();
        let random_key: libp2p::identity::Keypair =
            libp2p::identity::secp256k1::Keypair::from(secret_key.clone()).into();
        let random_peer_id = libp2p::PeerId::from_public_key(&random_key.public())
            .to_string()
            .to_lowercase();
        // println!("{}", random_peer_id);

        if random_peer_id.ends_with(keyword) {
            Some((secret_key, random_peer_id))
        } else {
            None
        }
    });
    let elapsed = start_time.elapsed();

    match ans {
        Some((key, peer_id)) => {
            println!("PeerID:     {}", peer_id);
            println!("Secret key: {}", hex::encode(key.to_bytes()));
        }
        None => println!(
            "Could not find a peer_id containing the keyword: {}",
            keyword
        ),
    }
    println!("That took {:?}", elapsed);
}
