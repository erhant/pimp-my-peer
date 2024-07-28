use libp2p_identity::{
    secp256k1::{Keypair as Secp256k1Keypair, SecretKey},
    Keypair, PeerId,
};
use rayon::prelude::*;
use std::time::Duration;

use crate::{keyword::Keyword, strategy::RandomStrategy};

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
    pub fn crunch(&self, max_iters: usize) -> (Duration, Option<(SecretKey, Keypair, PeerId)>) {
        let start_time = std::time::Instant::now();

        // TODO: take strategy from outside
        let result = RandomStrategy::new(max_iters)
            .into_par_iter()
            .find_map_first(|key| self.is_valid(key));

        (start_time.elapsed(), result)
    }

    /// Creates the peerId from a given secret key, and checks for matches.
    #[inline]
    pub fn is_valid(&self, secret_key: SecretKey) -> Option<(SecretKey, Keypair, PeerId)> {
        let keypair: Keypair = Secp256k1Keypair::from(secret_key.clone()).into();
        let peer_id = PeerId::from_public_key(&keypair.public());

        // TODO: is it worth to do the par_iter here?
        if self
            .keywords
            .par_iter()
            .all(|keyword| keyword.check(&peer_id.to_string().to_lowercase()))
        {
            Some((secret_key, keypair, peer_id))
        } else {
            None
        }
    }
}
