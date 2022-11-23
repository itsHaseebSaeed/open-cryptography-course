use criterion::{black_box, criterion_group, criterion_main, Criterion};

use rsa::{
    errors::Error,
    rand_core::{CryptoRng, RngCore},
    PaddingScheme, PublicKey, RsaPrivateKey, RsaPublicKey,
};

use rsa_oaep_pss::{generate_rsa_keys, RsaError, RsaOaep, RsaPublicKey as RsaOaepPublicKey};
use sha2::{Digest, Sha256};

/// encrypt function from rsa crate
fn rsa_encrypt<R: RngCore + CryptoRng>(
    rng: &mut R,
    pub_key: &RsaPublicKey,
    data: &[u8],
) -> Result<Vec<u8>, Error> {
    Ok(pub_key
        .encrypt(rng, PaddingScheme::new_oaep::<Sha256>(), data)
        .expect("failed to encrypt"))
}

/// encrypt function from rsa_oaep_pss crate
fn rsa_oaep_pss_encrypt<R: RngCore + CryptoRng>(
    rng: &mut R,
    pub_key: &RsaOaepPublicKey,
    data: &[u8],
) -> Result<Vec<u8>, RsaError> {
    let mut oaep_encryption = RsaOaep::new(rng, &Sha256::new());
    oaep_encryption.encrypt(pub_key, data)
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let bits = 2048;
    let priv_key = RsaPrivateKey::new(&mut rng.clone(), bits).expect("failed to generate a key");
    let pub_key = RsaPublicKey::from(&priv_key);
    let data = b"hello world";

    c.bench_function("rsa-encrypt 'oaep-sha256'", |b| {
        b.iter(|| rsa_encrypt(&mut rng, &pub_key, black_box(data)))
    });

    //let mut oaep = rsa_oaep_pss::RsaOaep::new(rand::rngs::OsRng, &Sha256::new());
    let (public_key, _private_key) = generate_rsa_keys(&mut rng.clone(), bits).unwrap();
    c.bench_function("rsa_oaep_pss-encrypt 'oaep-sha256'", |b| {
        b.iter(|| rsa_oaep_pss_encrypt(&mut rng, &public_key, black_box(data)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
