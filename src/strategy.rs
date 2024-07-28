use libp2p_identity::secp256k1::SecretKey;
use libsecp256k1::curve::{Affine, ECMultGenContext, Jacobian, Scalar, AFFINE_G};
use rayon::iter::plumbing::*;
use rayon::prelude::*;

/// Randomly generates keys in parallel.
pub struct RandomStrategy {
    pub max_iters: usize,
}

impl RandomStrategy {
    pub fn new(max_iters: usize) -> Self {
        Self { max_iters }
    }
}

impl ParallelIterator for RandomStrategy {
    type Item = SecretKey;

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: UnindexedConsumer<Self::Item>,
    {
        (0..self.max_iters)
            .into_par_iter()
            .map(|_| SecretKey::generate())
            .drive_unindexed(consumer)
    }
}

/// Creates keys one-by-one, starting from a given seed.
///
/// Let `G` be the generator point. First, we compute `P` from a given seed `d`` as `P = d * G`.
/// Then, we add `G` to `P` one by one, where the work is shared between threads. For example,
/// if `max_iters = 4` and we have 2 threads, the following work is done:
///
/// - Thread 1: `G, 2G`
/// - Thread 2: `3G, 4G`
pub struct LinearStrategy {
    pub max_iters: usize,
    seed: u32,
    init: Jacobian,
    context: Box<ECMultGenContext>,
}

impl LinearStrategy {
    pub fn new(max_iters: usize, seed: u32) -> Self {
        let context = ECMultGenContext::new_boxed();
        let mut g = Jacobian::from_ge(&AFFINE_G);
        context.ecmult_gen(&mut g, &Scalar::from_int(seed));

        Self {
            max_iters,
            seed,
            context,
            init: g,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_curve_add_mul() {
        let g = AFFINE_G;

        // add G to itself many times
        let g_add = Jacobian::from_ge(&g)
            .add_ge(&g)
            .add_ge(&g)
            .add_ge(&g)
            .add_ge(&g);
        println!("ADD: {:?}\n", Affine::from_gej(&g_add));

        // multiply G by some number, should be equal to above
        let context = ECMultGenContext::new_boxed();
        let mut g_mul = Jacobian::from_ge(&g);
        context.ecmult_gen(&mut g_mul, &Scalar::from_int(5));
        println!("MUL: {:?}", Affine::from_gej(&g_mul));
    }
}

// fn point_to_public_key(p: Projective) -> PublicKey {
//     PublicKey::from(p.to_string());
//     f
// }
