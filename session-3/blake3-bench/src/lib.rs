pub fn sha2_256(message: &[u8]) -> Vec<u8> {
    use sha2::{Digest, Sha256};
    // create a Sha256 object
    let mut hasher = Sha256::new();
    // write input message
    hasher.update(message);
    // read hash digest and consume hasher
    let result = hasher.finalize();

    result.to_vec()
}

pub fn sha3_256(message: &[u8]) -> Vec<u8> {
    use sha3::{Digest, Sha3_256};
    // create a SHA3-256 object
    let mut hasher = Sha3_256::new();
    // write input message
    hasher.update(message);
    // read hash digest
    let result = hasher.finalize();

    result.to_vec()
}

pub fn blake3(message: &[u8]) -> Vec<u8> {
    let mut hasher = blake3::Hasher::new();
    hasher.update(message);
    let result = hasher.finalize();
    result.as_bytes().to_vec()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
