use libp2p_identity::secp256k1::{Keypair, PublicKey, SecretKey};
use libp2p_identity::PeerId;
use rayon::prelude::*;
use std::time::Duration;

use crate::{keyword::Keyword, strategy::*};

#[derive(Debug)]
pub struct PimpMyPeer {
    keywords: Vec<Keyword>,
}

impl PimpMyPeer {
    pub fn new() -> Self {
        Self {
            keywords: Vec::new(),
        }
    }

    /// Adds a keyword to be prefix of the peer id.
    #[inline]
    pub fn starts_with(&mut self, keyword: &str) -> &mut Self {
        self.keywords
            .push(Keyword::StartsWith(keyword.to_string().to_lowercase()));
        self
    }

    /// Adds a keyword to be suffix of the peer id.
    #[inline]
    pub fn ends_with(&mut self, keyword: &str) -> &mut Self {
        self.keywords
            .push(Keyword::EndsWith(keyword.to_string().to_lowercase()));
        self
    }

    /// Adds a keyword to be contained within the peer id.
    #[inline]
    pub fn contains(&mut self, keyword: &str) -> &mut Self {
        self.keywords
            .push(Keyword::Contains(keyword.to_string().to_lowercase()));
        self
    }

    /// Crunches to find a matching peer id.
    pub fn crunch(&self, max_iters: usize) -> (Duration, Option<(SecretKey, PublicKey, PeerId)>) {
        let start_time = std::time::Instant::now();

        // TODO: take strategy from outside
        // let strategy = RandomStrategy::new(max_iters);
        // TODO: take seed from outside
        let strategy = LinearStrategy::new(max_iters, &[0x61u8; 32]);
        let result = strategy
            .into_par_iter()
            .find_map_first(|result| self.is_valid(result));

        (start_time.elapsed(), result)
    }

    /// Creates the peerId from a given secret key, and checks for matches.
    #[inline]
    pub fn is_valid(
        &self,
        result: (SecretKey, PublicKey),
    ) -> Option<(SecretKey, PublicKey, PeerId)> {
        // TODO: only in debug mode
        // let keypair = Keypair::from(result.0.clone());
        // debug_assert!(keypair.public().eq(&result.1));

        // type-casting required here
        let pk: libp2p_identity::PublicKey = result.1.clone().into();
        let peer_id = PeerId::from_public_key(&pk);

        // TODO: is it worth to do the par_iter here?
        if self
            .keywords
            .par_iter()
            .all(|keyword| keyword.check(&peer_id.to_string().to_lowercase()))
        {
            Some((result.0, result.1, peer_id))
        } else {
            None
        }
    }
}
