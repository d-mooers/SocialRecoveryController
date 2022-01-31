use shamirsecretsharing::*;
use aes_gcm::{Aes256Gcm, Key, Nonce}; 
use aes_gcm::aead::{Aead, NewAead};
use rsa::{PublicKey, RsaPrivateKey, PaddingScheme};
use rand::Rng;

fn encrypt_mnemonic(mnemonic: &str) {
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
    let encrypted_mnemonic = cipher.encrypt(nonce, mnemonic).expect("encryption failure!");
    return (key_and_nonce, encrypted_mnemonic);
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
    return 
}