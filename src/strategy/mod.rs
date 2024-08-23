use std::fmt::{Debug, Formatter};

use libp2p_identity::secp256k1::{Keypair, PublicKey, SecretKey};

use libsecp256k1::curve::{ECMultGenContext, Jacobian, Scalar, AFFINE_G};
use rayon::iter::plumbing::*;
use rayon::prelude::*;

/// A strategy to generate keys in parallel.
///
/// - Implements parallel iterator and display.
pub trait Strategy: ParallelIterator<Item = (SecretKey, PublicKey)> + Debug {}

mod linear;
pub use linear::LinearStrategy;

mod random;
pub use random::RandomStrategy;

mod linear_memo;
pub use linear_memo::LinearMemoStrategy;
