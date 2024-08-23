use libp2p_identity::{
    secp256k1::{PublicKey, SecretKey},
    DecodingError,
};
use libsecp256k1::{
    curve::{Affine, Jacobian, Scalar},
    util::{TAG_PUBKEY_EVEN, TAG_PUBKEY_ODD},
};

#[inline]
pub fn jacobian_to_public_key(j: &Jacobian) -> Result<PublicKey, DecodingError> {
    let mut j = Affine::from_gej(j);
    j.x.normalize();
    j.y.normalize();

    let mut bytes: [u8; 33] = [0u8; 33];
    bytes[0] = if j.y.is_odd() {
        TAG_PUBKEY_ODD
    } else {
        TAG_PUBKEY_EVEN
    };
    bytes[1..33].copy_from_slice(&j.x.b32());
    PublicKey::try_from_bytes(&bytes)
}

#[inline]
pub fn scalar_to_secret_key(s: &Scalar) -> Result<SecretKey, DecodingError> {
    SecretKey::try_from_bytes(s.b32())
}

#[cfg(test)]
mod tests {
    use libp2p_identity::secp256k1::Keypair;
    use libsecp256k1::curve::{Affine, ECMultGenContext, AFFINE_G};

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

    #[test]
    fn test_scalar_sk_jacobian_pk() {
        let context = ECMultGenContext::new_boxed();

        // create random secret key
        let random_num = rand::random::<u32>();
        let scalar = Scalar::from_int(random_num);
        let secret_key = scalar_to_secret_key(&scalar).unwrap();
        let expected_public_key = Keypair::from(secret_key).public().clone();

        // compute public key
        let mut jacobian = Jacobian::from_ge(&AFFINE_G);
        context.ecmult_gen(&mut jacobian, &scalar);
        let public_key = jacobian_to_public_key(&jacobian).unwrap();

        assert_eq!(public_key, expected_public_key)
    }
}
