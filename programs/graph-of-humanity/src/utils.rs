use sha2::{Digest, Sha256};

pub fn sha256_hash(input: u64, seed: u64) -> u64 {
    let mut hasher = Sha256::new();
    hasher.update(input.to_le_bytes());
    hasher.update(seed.to_le_bytes());
    let result = hasher.finalize();
    u64::from_le_bytes(result[0..8].try_into().unwrap())
}
