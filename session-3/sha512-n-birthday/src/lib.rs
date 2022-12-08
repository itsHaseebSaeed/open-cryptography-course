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

    for i in 0..=u64::MAX {
        let hash = sha512_n_bytes(n, &i.to_le_bytes());
        if let Some(j) = hash_map.insert(hash, i) {
            return (i, j);
        }
    }

    panic!("collision not found");
}

pub fn sha512_n_bytes_find_message(n: usize, bytes: &[u8], initial_value: Option<u64>) -> u64 {
    let initial_value = initial_value.unwrap_or(0);

    if bytes.len() != n {
        panic!("bytes length does not match n")
    }

    for i in initial_value..=u64::MAX {
        let hash = sha512_n_bytes(n, &i.to_le_bytes());
        if hash.eq(bytes) {
            return i;
        }
    }

    panic!("collision not found")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn birthday_attack_works() {
        for n in 1..=6_usize {
            birthday_attack_works_for_n_bytes(n);
        }
    }
    fn birthday_attack_works_for_n_bytes(n: usize) {
        let collision_at = sha512_n_bytes_bithday_attack(n);
        println!("Collision of sha-512-{} found at {:?}", n * 8, collision_at)
    }

    #[test]
    fn brute_force_works() {
        let bytes = b"\x3D\x4B";
        let mut past_val = 0;
        let mut num_trials = vec![];
        for _i in 0..5 {
            let val = brute_force_works_for_n_bytes(2, bytes, Some(past_val));
            num_trials.push(val - past_val);
            past_val = val + 1;
        }
        let average: f64 =
            (num_trials.iter().sum::<u64>() as f64).div_euclid(num_trials.len() as f64);
        println!("Average num of trials: {}", average);
    }

    fn brute_force_works_for_n_bytes(n: usize, bytes: &[u8], initial_value: Option<u64>) -> u64 {
        let message = sha512_n_bytes_find_message(n, bytes, initial_value);
        println!(
            "A mmssage that would produce {:x?} with sha-512-{} is found at {:?}",
            bytes,
            n * 8,
            message
        );
        message
    }
}
