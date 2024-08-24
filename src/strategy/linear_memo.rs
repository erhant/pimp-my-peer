use super::*;
use crate::utils::{jacobian_to_public_key, scalar_to_secret_key};
use dashmap::DashMap;
use rayon::{current_num_threads, current_thread_index};

pub struct LinearMemoStrategy {
    pub max_iters: usize,
    seed: Scalar,
    init: Jacobian,
    context: Box<ECMultGenContext>,
}

impl LinearMemoStrategy {
    pub fn new(max_iters: usize, seed_bytes: &[u8; 32]) -> Self {
        let context = ECMultGenContext::new_boxed();
        let mut seed = Scalar::default();
        let choice = seed.set_b32(seed_bytes);
        assert_eq!(choice.unwrap_u8(), 0, "overflow");

        // initial is `seed * G`
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

impl IsStrategy for LinearMemoStrategy {}

impl ParallelIterator for LinearMemoStrategy {
    type Item = (SecretKey, PublicKey);

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: UnindexedConsumer<Self::Item>,
    {
        // memoization is over Scalars (secret key) and their corresponding Jacobians (public key)
        // if a memo exists, the scalar should be incremented and `g` should be added to the jacobian
        let memo = DashMap::<usize, (Scalar, Jacobian)>::default();

        // create one
        let mut one = Scalar::default();
        one.cadd_bit(0, true);
        debug_assert!(one.is_one());

        let work_size = self.max_iters / current_num_threads();

        (0..self.max_iters)
            .into_par_iter()
            .map_with(memo, |memo, _| {
                let thread_idx = current_thread_index().unwrap();

                // get latest scalar & jacobian
                // FIXME: this is not working, `memo` is not updated
                memo.entry(thread_idx)
                    .and_modify(|(sk, pk)| {
                        *sk = *sk + one; // increment scalar
                        *pk = pk.add_ge(&AFFINE_G); // increment jacobian
                    })
                    .or_insert_with(|| {
                        // compute scalar as work_size * thread_idx
                        let sk = Scalar::from_int((work_size * thread_idx) as u32);

                        // compute public key
                        let mut pk = self.init.clone();
                        self.context.ecmult_gen(&mut pk, &sk);

                        (sk, pk)
                    });

                let (sk, pk) = *memo.get(&thread_idx).unwrap();

                println!("Thread {} - sk: {:x}", thread_idx, sk);
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

impl Debug for LinearMemoStrategy {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Linear Memoized Strategy {{ max_iters: {}, seed:{:x} }}",
            self.max_iters, self.seed
        )
    }
}
