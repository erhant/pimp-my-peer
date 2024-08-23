use libp2p_identity::secp256k1::{PublicKey, SecretKey};
use libp2p_identity::PeerId;
use rayon::prelude::*;
use std::time::Duration;

use crate::keyword::Keywords;
use crate::strategy::Strategy;

/// Crunches to find a matching peer id.
///
/// Pimp is a recursive acronym for "(PI)mp (M)y (P)eer".
///
/// Returns the first matching secret key, its public key and peer id.
pub fn pimp(
    strategy: impl Strategy,
    keywords: &Keywords,
) -> (Duration, Option<(SecretKey, PublicKey, PeerId)>) {
    let start_time = std::time::Instant::now();

    let result = strategy.into_par_iter().find_map_first(|(sk, pk)| {
        let peer_id = PeerId::from_public_key(&(libp2p_identity::PublicKey::from(pk.clone())));

        if keywords.is_valid(&peer_id.to_string().to_lowercase()) {
            Some((sk, pk, peer_id))
        } else {
            None
        }
    });

    (start_time.elapsed(), result)
}
