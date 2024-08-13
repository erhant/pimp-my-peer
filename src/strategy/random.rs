use super::*;

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
    type Item = (SecretKey, PublicKey);

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: UnindexedConsumer<Self::Item>,
    {
        (0..self.max_iters)
            .into_par_iter()
            .map(|_| {
                let secret_key = SecretKey::generate();
                let keypair = Keypair::from(secret_key.clone());

                (secret_key, keypair.public().to_owned())
            })
            .drive_unindexed(consumer)
    }

    fn opt_len(&self) -> Option<usize> {
        Some(self.max_iters)
    }
}
