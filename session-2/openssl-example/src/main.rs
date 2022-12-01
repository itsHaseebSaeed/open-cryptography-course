use openssl::symm::Crypter;

use openssl::symm::{Cipher, Mode};
use rustc_serialize::hex::ToHex;

fn main() {
    let cipher = Cipher::aes_256_cbc();
    let key_hex = b"\x80\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01";

    // --------------------- Answer to 3.8 ----------------------------
    let ciphertext = b"\x53\x9B\x33\x3B\x39\x70\x6D\x14\x90\x28\xCF\xE1\xD9\xD4\xA4\x07";
    let mut output: Vec<u8> = vec![0x0; 32];

    let mut decrypter = Crypter::new(cipher, Mode::Decrypt, key_hex, None).unwrap();
    decrypter.pad(false);

    decrypter.update(ciphertext, &mut output).unwrap();

    let plaintext = &output[..16];

    println!("Answer to 3.8: {:?}", plaintext.to_hex());

    // -------------------- Answer to 3.9(a) --------------------------

    let mut encrypter = Crypter::new(cipher, Mode::Encrypt, key_hex, None).unwrap();
    encrypter.pad(false);

    let new_plaintext = b"\x29\x6C\x93\xFD\xF4\x99\xAA\xEB\x41\x94\xBA\xBC\x2E\x63\x56\x1D";

    let mut new_output: Vec<u8> = vec![0x0; 32];

    encrypter.update(new_plaintext, &mut new_output).unwrap();

    let new_ciphertext = &new_output[0..16];

    println!("Answer to 3.9(a): {:?}", new_ciphertext.to_hex());

    // -------------------- Answer to 3.10 ----------------------------

    let des_key = b"\x80\x0e\x00\x20\x00\x0f\x00\x00";
    let des_cipher = Cipher::des_cbc();

    let des_plaintext = b"\x29\x6C\x93\xFD\xF4\x99\xAA\xEB";
    let mut des_output: Vec<u8> = vec![0x0; 16];

    let mut des_encrypter = Crypter::new(des_cipher, Mode::Decrypt, des_key, None).unwrap();
    des_encrypter.pad(false);

    des_encrypter
        .update(des_plaintext, &mut des_output)
        .unwrap();

    let des_ciphertext = Vec::from(&des_output[..8]);

    let comp_key: Vec<u8> = des_key.iter().map(|x| !x).collect();
    let comp_plaintext: Vec<u8> = des_plaintext.iter().map(|x| !x).collect();

    let mut comp_encrypter = Crypter::new(des_cipher, Mode::Decrypt, &comp_key, None).unwrap();
    comp_encrypter.pad(false);

    comp_encrypter
        .update(&comp_plaintext, &mut des_output)
        .unwrap();

    let comp_ciphertext = Vec::from(&des_output[..8]);

    assert_eq!(
        comp_ciphertext.iter().map(|x| !x).collect::<Vec<u8>>(),
        des_ciphertext
    );

    println!("(3.10) DES complementation property test passed.");
    // -------------------- Answer to 4.4 -----------------------------

    //let iv = b"\x87\xF3\x48\xFF\x79\xB8\x11\xAF\x38\x57\xD6\x71\x8E\x5F\x0F\x91";
    let new_key = b"\x80\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01";
    let raw_ciphertext = b"\x87\xF3\x48\xFF\x79\xB8\x11\xAF\x38\x57\xD6\x71\x8E\x5F\x0F\x91\x7C\x3D\x26\xF7\x73\x77\x63\x5A\x5E\x43\xE9\xB5\xCC\x5D\x05\x92\x6E\x26\xFF\xC5\x22\x0D\xC7\xD4\x05\xF1\x70\x86\x70\xE6\xE0\x17";
    let iv = &raw_ciphertext[..16];
    let real_ciphertext = &raw_ciphertext[16..48];
    let mut output: Vec<u8> = vec![0x0; 64];

    let mut iv_decrypter = Crypter::new(cipher, Mode::Decrypt, new_key, Some(iv)).unwrap();
    iv_decrypter.pad(false);

    iv_decrypter.update(real_ciphertext, &mut output).unwrap();

    let iv_plaintext = &output[..32];

    println!("Answer to 4.4: {:?}", iv_plaintext.to_hex());
}
