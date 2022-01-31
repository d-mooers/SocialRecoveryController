use shamirsecretsharing::*;
use aes_gcm::{Aes256Gcm, Key, Nonce}; 
use aes_gcm::aead::{Aead, NewAead};
use rand::Rng;

fn encrypt_mnemonic(mnemonic: str) {
    const NONCE_SIZE = 12;
    const KEY_SIZE = DATA_SIZE - NONCE_SIZE;
    let mut rng = rand::thread_rng();
    let key_and_nonce: Vec<u8> = Vec::with_capacity(DATA_SIZE);
    let nonce: Nonce = Nonce::from_slice()

    for i in (0..DATA_SIZE) {
        key.push(rng.gen());
    }
    let key = Key::from_slice(&key_and_nonce[0..KEY_SIZE]);
    let none = Nonce::from_slice(&key_and_nonce[KEY_SIZE..DATA_SIZE]);
    let cipher = Aes256Gcm::new(key);
    let encrypted_mnemonic = cipher.encrypt(nonce, mnemonic);
}