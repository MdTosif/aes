use aes_gcm::{
    aead::{generic_array::GenericArray, Payload, Aead, KeyInit},
    Aes128Gcm,
};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn encrypt(key:&str,nonce:&str, data:&[u8]) -> Vec<u8> {
    let nonce = GenericArray::from_slice(nonce.as_bytes());
    let payload = Payload {
        msg: data,
        aad: "".as_bytes(),
    };

    let cipher = Aes128Gcm::new_from_slice(key.as_bytes()).unwrap();
    let ciphertext = cipher.encrypt(nonce, payload).unwrap();
    return ciphertext;
}

#[wasm_bindgen]
pub fn decrypt(key:&str,nonce:&str, data:&[u8]) -> Vec<u8> {
    let nonce = GenericArray::from_slice(nonce.as_bytes());
    let payload = Payload {
        msg: data,
        aad: "".as_bytes(),
    };

    let cipher = Aes128Gcm::new_from_slice(key.as_bytes()).unwrap();
    let ciphertext = cipher.decrypt(nonce, payload).unwrap();
    return ciphertext;
}
