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

        let mut g = Jacobian::from_ge(&AFFINE_G);
        context.ecmult_gen(&mut g, &seed);

        Self {
            max_iters,
            seed,
            context,
            init: g,
        }
    }
}

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
                let scalar = i_scalar + self.seed;
                let secret_key =
                    SecretKey::try_from_bytes(scalar.b32()).expect("should parse secret key");

                // compute public key
                let mut p = self.init.clone();
                self.context.ecmult_gen(&mut p, &scalar); // i + seed
                let p = Affine::from_gej(&p);

                // to bytes in compressed public key format
                let mut bytes: [u8; 33] = [0u8; 33];
                bytes[0] = if p.y.is_odd() { 0x02 } else { 0x03 };
                bytes[1..33].copy_from_slice(&p.x.b32());
                let public_key =
                    PublicKey::try_from_bytes(&bytes).expect("should parse public key");

                (secret_key, public_key)
            })
            .drive_unindexed(consumer)
    }

    fn opt_len(&self) -> Option<usize> {
        Some(self.max_iters)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_curve_add_mul() {
        let g = AFFINE_G;
        const N: usize = 5;

        // add G to itself many times
        let mut g_add = Jacobian::from_ge(&g);
        for _ in 1..N {
            g_add = g_add.add_ge(&g);
        }

        // multiply G by some scalar
        let context = ECMultGenContext::new_boxed();
        let mut g_mul = Jacobian::from_ge(&g);
        context.ecmult_gen(&mut g_mul, &Scalar::from_int(N as u32));

        // both should be equal
        assert_eq!(Affine::from_gej(&g_add), Affine::from_gej(&g_mul));
    }
}
