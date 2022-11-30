use openssl::symm::Crypter;

use openssl::symm::{Cipher, Mode};
use rustc_serialize::hex::ToHex;

fn main() {
    let cipher = Cipher::aes_256_ecb();
    let key_hex = b"\x80\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01";

    // --------------------- Answer to 3.8 ----------------------------
    let ciphertext = b"\x53\x9B\x33\x3B\x39\x70\x6D\x14\x90\x28\xCF\xE1\xD9\xD4\xA4\x07";
    let mut output: Vec<u8> = vec![0x0; 32];

    let mut decrypter = Crypter::new(cipher, Mode::Decrypt, key_hex, None).unwrap();
    decrypter.pad(false);

    decrypter.update(ciphertext, &mut output).unwrap();

    let plaintext = &output[0..16];

    println!("Answer to 3.8: {:?}", plaintext.to_hex());

    // -------------------- Answer to 3.9(a) --------------------------

    let mut encrypter = Crypter::new(cipher, Mode::Encrypt, key_hex, None).unwrap();
    encrypter.pad(false);

    let new_plaintext = b"\x29\x6C\x93\xFD\xF4\x99\xAA\xEB\x41\x94\xBA\xBC\x2E\x63\x56\x1D";

    let mut new_output: Vec<u8> = vec![0x0; 32];

    encrypter.update(new_plaintext, &mut new_output).unwrap();

    let new_ciphertext = &new_output[0..16];

    println!("Answer to 3.9(a): {:?}", new_ciphertext.to_hex());
}
