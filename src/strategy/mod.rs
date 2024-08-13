use libp2p_identity::secp256k1::{Keypair, PublicKey, SecretKey};

use libsecp256k1::curve::{Affine, ECMultGenContext, Jacobian, Scalar, AFFINE_G};
use rayon::iter::plumbing::*;
use rayon::prelude::*;

mod linear;
pub use linear::LinearStrategy;

mod random;
pub use random::RandomStrategy;
