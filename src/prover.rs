//! Prover for ZKP Chaum Pedersen protocol
//! 
//! 
//! 
use num::{BigInt, ToPrimitive};

pub struct ChaumPedersenProver {
    // First generator
    pub g: u64,
    // Second Generator
    pub h: u64,
    // Group order
    pub q: u64,
    // Group modulus
    pub p: u64

}

pub trait ZKPProver {
    fn generate_registration_keys(&self,secret: i64) -> (i64,i64);
    fn generate_commitment(&self,k: u32) -> (i64,i64);
    fn generate_challenge(&self,k:u32,secret:i64, c: i64) -> i64;
}

impl ZKPProver for ChaumPedersenProver {
    fn generate_registration_keys(&self,secret: i64) -> (i64,i64) {
        let exponent = BigInt::from(secret);
        let modulus = BigInt::from(self.p);
        let first = BigInt::from(self.g).modpow(&exponent,&modulus);
        let second = BigInt::from(self.h).modpow(&exponent,&modulus);      

        (first.to_i64().unwrap(),second.to_i64().unwrap())
    }

    fn generate_commitment(&self,k: u32) -> (i64,i64) {
        let exponent = BigInt::from(k);
        let modulus = BigInt::from(self.p);
        let first = BigInt::from(self.g).modpow(&exponent,&modulus);
        let second = BigInt::from(self.h).modpow(&exponent,&modulus);      

        (first.to_i64().unwrap(),second.to_i64().unwrap())
    }

    fn generate_challenge(&self,k:u32, secret:i64, c: i64) -> i64 {
        (k as i64 - c*(secret)) % self.q as i64
    }


}

