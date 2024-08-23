use std::fmt::Debug;

use crate::utils::{jacobian_to_public_key, scalar_to_secret_key};

use super::*;

/// Generates keys with increasing scalars for the secret key.
pub struct LinearStrategy {
    pub max_iters: usize,
    seed: Scalar,
    init: Jacobian,
    context: Box<ECMultGenContext>,
}

impl LinearStrategy {
    pub fn new(max_iters: usize, seed_bytes: &[u8; 32]) -> Self {
        let context = ECMultGenContext::new_boxed();
        let mut seed = Scalar::default();
        let choice = seed.set_b32(seed_bytes);
        assert_eq!(choice.unwrap_u8(), 0, "overflow");

        let mut init = Jacobian::from_ge(&AFFINE_G);
        context.ecmult_gen(&mut init, &seed);

        Self {
            max_iters,
            seed,
            context,
            init,
        }
    }
}

impl Strategy for LinearStrategy {}

impl ParallelIterator for LinearStrategy {
    type Item = (SecretKey, PublicKey);

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: UnindexedConsumer<Self::Item>,
    {
        (0..self.max_iters)
            .into_par_iter()
            .map(|i| {
                let i_scalar = Scalar::from_int(i as u32);

                // create secret key from scalar
                let sk = i_scalar + self.seed;

                // compute public key
                let mut pk = self.init.clone();
                self.context.ecmult_gen(&mut pk, &sk); // i + seed

                (
                    scalar_to_secret_key(&sk).expect("should parse secret key"),
                    jacobian_to_public_key(&pk).expect("should parse public key"),
                )
            })
            .drive_unindexed(consumer)
    }

    fn opt_len(&self) -> Option<usize> {
        Some(self.max_iters)
    }
}

impl Debug for LinearStrategy {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Linear Strategy {{ max_iters: {}, seed:{:x} }}",
            self.max_iters, self.seed
        )
    }
}
