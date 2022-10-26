//! Cryptographic calculations for ZKP Chaum Pedersen protocol.
//! 

use rand::Rng;
use primes::{Sieve,PrimeSet,is_prime};

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
pub fn generate_keys() -> (i64,i64,i64) {
    let mut rng = rand::thread_rng();
    // This is the lower limit for the prime p.
    // For simplicity we generate random numbers up to 100. 
    let lower: u64 = rng.gen_range(7..200);

    // Generate a prime number using Eratostenes's Sieve
    let mut pset = Sieve::new();
    let mut q: i64 = 0;
    let mut p = 0;
    while p == 0 {
        q = pset.find(lower).1 as i64;
        p = find_modulus(q);
    }

    let g = find_key(p,q);
    let mut h: i64 = q;
    while h == q {
        h = find_key(p,q);
    }

    (g,h,q as i64)
}

fn find_key(p: i64, q: i64) -> i64 {
    println!("Finding Key for p:{} and q:{}",p,q);
    let mut rng = rand::thread_rng();
    let mut random: i64;
    let mut key: i64 = 1;
    let mut overflow;
    let quotient = ((p-1) / q) as u32;
    let mut try_count = 0;

    while key == 1 && try_count < p/2 {
        random = rng.gen_range(2..(p-1) as i64);
        (key, overflow) = random.overflowing_pow(quotient);
        try_count+=1;
        if overflow {
            key = 1;
        } else if key.overflowing_pow(11).1 {
            key = 1;
        }
    }
    println!("Found key: {}", key);
    key
}

fn find_modulus(q: i64) -> i64 {
    let mut p;
    for n in 2..100 {  
        p = n*q +1;  
        if is_prime(p as u64) {
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
        println!("Found keys: {},{},{}", keys.0,keys.1,keys.2);
        assert_ne!(0,keys.0);
        assert_ne!(0,keys.1);
        assert_ne!(0,keys.2);
    }

}

