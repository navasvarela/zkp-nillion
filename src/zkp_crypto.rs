//! Cryptographic calculations for ZKP Chaum Pedersen protocol.
//! 
//! This is a very naive module that is produced to generate
//! values needed for a Chaum-Pedersen protocol run. Just for the purpose of testing.
//! 
//! It relies on the crate `primes` to generate and validate primes.
//! 
//! These values are:
//! - The modulus and order of a group G of prime order. Both modulus and order must be prime numbers.
//! - Two generators for the group G.

use rand::Rng;
use primes::{Sieve,PrimeSet,is_prime};

#[derive(Debug, Default)]
pub struct ChaumPedersenAttrs {
    pub modulus: u64,
    pub order: u64,
    pub first_generator: u64,
    pub second_generator: u64
}

/// Generates the keys required by the Chaum Pedersen protocol. 
/// 
/// Returns a tuple with (g,h,q)
/// 
/// The Chaum Pedersen protocol requires two keys g, h that generate
/// a group of prime order q. This function returns g, h and q. 
/// 
/// Starting with a random prime p, we find q so that q:
/// - is prime
/// - divides p-1 evenly.
/// 
/// The keys g and h can be calculated using that for any integer X that is not a multiple of p,
/// then X^(p-1/q) generates a group of order 1 or q.
pub fn generate_keys() -> ChaumPedersenAttrs {
    let mut rng = rand::thread_rng();
    // This is the lower limit for the prime p.
    // For simplicity we generate random numbers up to 100. 
    let lower: u64 = rng.gen_range(7..200);

    // Generate a prime number using Eratostenes's Sieve
    let mut pset = Sieve::new();
    let mut q: u64 = 0;
    let mut p = 0;
    while p == 0 {
        q = pset.find(lower).1;
        p = find_modulus(q);
    }

    let g = find_key(p,q);
    let mut h: u64 = 1;
    while h == 1 {
        h = find_key(p,q);
    }

    ChaumPedersenAttrs{
        modulus: p,
        order: q,
        first_generator: g,
        second_generator: h
    }
}

fn find_key(p: u64, q: u64) -> u64 {
    println!("Finding Key for p:{} and q:{}",p,q);
    let mut rng = rand::thread_rng();
    let mut random: u64;
    let mut key: u64 = 1;
    let mut overflow;
    let quotient = ((p-1) / q) as u32;
    let mut try_count = 0;

    while key == 1 && try_count < p/2 {
        random = rng.gen_range(2..(p-1));
        (key, overflow) = random.overflowing_pow(quotient);
        try_count+=1;
        if overflow {
            key = 1;
        } 
    }
    println!("Found key: {}", key);
    key
}

fn find_modulus(q: u64) -> u64 {
    let mut p;
    for n in 2..100 {  
        p = n*q +1;  
        if is_prime(p) {
            println!("Found p: {}", p);
            return p;
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_keys() {
        let keys = generate_keys();
        println!("Found keys: {}, {}, {}, {}", keys.modulus,keys.order,keys.first_generator,keys.second_generator);
        assert_ne!(0,keys.order);
        assert_ne!(0,keys.first_generator);
        assert_ne!(0,keys.second_generator);
    }

}

