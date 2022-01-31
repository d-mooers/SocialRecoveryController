use shamirsecretsharing::*;
use aes_gcm::{Aes256Gcm, Key, Nonce}; 
use aes_gcm::aead::{Aead, NewAead};
use rsa::{PublicKey, RsaPrivateKey, PaddingScheme};
use rand::Rng;
// DATA_SIZE = 64 (in shamirsecretsharing)
const NONCE_SIZE = 12;
const KEY_SIZE = DATA_SIZE - NONCE_SIZE;

fn encrypt_mnemonic(mnemonic: &str) {
    let mut rng = rand::thread_rng();
    let key_and_nonce: Vec<u8> = Vec::with_capacity(DATA_SIZE);
    let nonce: Nonce = Nonce::from_slice()

    for i in (0..DATA_SIZE) {
        key.push(rng.gen());
    }
    let key = Key::from_slice(&key_and_nonce[0..KEY_SIZE]);
    let none = Nonce::from_slice(&key_and_nonce[KEY_SIZE..DATA_SIZE]);
    let cipher = Aes256Gcm::new(key);
    let encrypted_mnemonic = cipher.encrypt(nonce, mnemonic).expect("encryption failure!");
    return (key_and_nonce, encrypted_mnemonic);
}

fn decrypt_mnemonic(encrypted_mnemonic: &[u8], key_and_nonce: &[u8]) {
    let key = Key::from_slice(key_and_nonce[0..KEY_SIZE]);
    let nonce = Nonce::from_slice(key_and_nonce[KEY_SIZE..DATA_SIZE]);
    let cipher = Aes256Gcm::new(key);
    return cipher.decrypt(nonce, encrypted_mnemonic).expect("decryption failure!")
}

fn create_key_shares(secret: &[u8], n: u8, k: u8) -> Vector<Vector<u8>> {
    let shares = create_shares(secret, n, k);
    match (shares) {
        Ok(shares) => return shares,
        Err(err) => panic!("Oh no! Something went wrong in create_key_shares: {}", err)
    }
}

fn encrypt_share(share: &[u8], pub_key: &[u8]) {
    let mut rng = rand::thread_rng();
    let key = RsaPublicKey::from_pkcs1_der(pub_key);
    return key.encrypt(&mut rng, PaddingScheme::new_pkcs1v15(), &share[..]);
}

fn convertHashToBytes(hash: H160) {
    return hash.as_bytes();
}

fn encrypt_shares(shares: Vector<Vector<u8>>, pub_keys: Vector<H160>) {
    return shares.iter().zip(pub_keys.iter()).map(|(&s, &k)| encrypt_share(s, convertHashToBytes(k)));
}