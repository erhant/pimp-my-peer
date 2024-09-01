use std::fmt::Debug;

use super::*;

use crate::utils::{jacobian_to_public_key, scalar_to_secret_key};

/// Only generates one key from the given seed.
pub struct IdentityStrategy {
    seed: Scalar,
    init: Jacobian,
    context: Box<ECMultGenContext>,
}

impl IdentityStrategy {
    pub fn new(seed_bytes: &[u8; 32]) -> Self {
        let context = ECMultGenContext::new_boxed();
        let mut seed = Scalar::default();
        let choice = seed.set_b32(seed_bytes);
        assert_eq!(choice.unwrap_u8(), 0, "overflow");

        let mut init = Jacobian::from_ge(&AFFINE_G);
        context.ecmult_gen(&mut init, &seed);

        Self {
            seed,
            context,
            init,
        }
    }
}

impl IsStrategy for IdentityStrategy {}

impl ParallelIterator for IdentityStrategy {
    type Item = (SecretKey, PublicKey);

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: UnindexedConsumer<Self::Item>,
    {
        (0..1)
            .into_par_iter()
            .map(|_| {
                // compute public key
                let mut pk = self.init.clone();
                self.context.ecmult_gen(&mut pk, &self.seed); // i + seed

                (
                    scalar_to_secret_key(&self.seed).expect("should parse secret key"),
                    jacobian_to_public_key(&pk).expect("should parse public key"),
                )
            })
            .drive_unindexed(consumer)
    }

    fn opt_len(&self) -> Option<usize> {
        Some(1)
    }
}

impl Debug for IdentityStrategy {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Identity Strategy {{ seed:{:x} }}", self.seed)
    }
}
