use std::fmt::{Debug, Formatter};

use libp2p_identity::secp256k1::{Keypair, PublicKey, SecretKey};

use libsecp256k1::curve::{ECMultGenContext, Jacobian, Scalar, AFFINE_G};
use rayon::iter::plumbing::*;
use rayon::prelude::*;

/// A strategy to generate keys in parallel.
///
/// - Implements parallel iterator and display.
pub trait IsStrategy: ParallelIterator<Item = (SecretKey, PublicKey)> + Debug + Sized {}

mod linear;
pub use linear::LinearStrategy;

mod random;
pub use random::RandomStrategy;

mod identity;
pub use identity::IdentityStrategy;

mod linear_memo;
pub use linear_memo::LinearMemoStrategy;

pub enum Strategy {
    Linear(LinearStrategy),
    Random(RandomStrategy),
    LinearMemo(LinearMemoStrategy),
}
