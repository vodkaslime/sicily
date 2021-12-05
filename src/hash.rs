use num::bigint::BigUint;
use sha2::{ Sha256, Digest };

pub fn digest(input: &String) -> BigUint {
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    let arr = hasher.finalize();
    BigUint::from_bytes_be(&arr)
}