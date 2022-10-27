//! Verifier implementation
//! 
//! This crate defines structures and implementation of Chaum Pedersen
//! Verification for exponents / discrete logarithms.
//! 
//! The trait `ZKPVerifier` can be used to implement other verifiers.
//! For instance, using elliptical curve cryptography.
//! 
use num::BigInt;
use crate::store::{RegistrationSecret, Authentication};
use crate::zkp_crypto::ChaumPedersenAttrs;

#[derive(Debug, Default)]
pub struct ChaumPedersenVerifier {
    pub attrs: ChaumPedersenAttrs,
}

pub trait ZKPVerifier {
    fn verify(&self, secret: RegistrationSecret, authentication: &Authentication, challenge: i64) -> bool;
}

fn _calculate_product(secret: i64,  c: i64, generator: &u64,s: i64, modulus: &u64) -> BigInt {
    // Convert to BigInt
    let exponent1 = BigInt::from(c);
    let bmod = BigInt::from(*modulus);
    let exponent2 = BigInt::from(s);
    BigInt::from(secret).modpow(&exponent1, &bmod) *  BigInt::from(*generator).modpow(&exponent2, &bmod)
}

impl ZKPVerifier for ChaumPedersenVerifier {
    /// Verify a Chaum Pedersen proof
    /// 
    fn verify(&self, secret: RegistrationSecret, authentication: &Authentication, challenge: i64) -> bool{
        let first = _calculate_product(secret.y1, authentication.c, &self.attrs.first_generator, challenge, &self.attrs.modulus);
        let second = _calculate_product(secret.y2, authentication.c, &self.attrs.second_generator, challenge, &self.attrs.modulus);

        BigInt::from(authentication.r1) == first && BigInt::from(authentication.r2) == second

    }

}