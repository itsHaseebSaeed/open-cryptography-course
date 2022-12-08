use std::collections::HashMap;

use sha2::{Digest, Sha512};

pub fn sha512_n_bytes(n: usize, message: &[u8]) -> Vec<u8> {
    // create a Sha256 object
    let mut hasher = Sha512::new();
    // write input message
    hasher.update(message);
    // read hash digest and consume hasher
    let result = hasher.finalize();
    // restrict to first n bytes
    let first_n_bytes = &result[..n];
    // return the first n bytes
    first_n_bytes.to_vec()
}

pub fn sha512_n_bytes_bithday_attack(n: usize) -> (u64, u64) {
    let mut hash_map: HashMap<Vec<u8>, u64> = HashMap::new();

    for i in 0..u64::MAX {
        let hash = sha512_n_bytes(n, &i.to_le_bytes());
        if let Some(j) = hash_map.insert(hash, i) {
            return (i, j);
        }
    }

    panic!("collision not found");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        for n in 1..=6_usize {
            it_works_for_n_bytes(n);
        }
    }
    fn it_works_for_n_bytes(n: usize) {
        let collision_at = sha512_n_bytes_bithday_attack(n);
        println!("Collision of sha-512-{} found at {:?}", n * 8, collision_at)
    }
}
