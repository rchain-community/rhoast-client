use crate::error::ErrCode;
use secp256k1::rand::thread_rng;
use secp256k1::{PublicKey, Secp256k1, SecretKey};

//use secret gotten from get_pri_pub_key_pair() to create new secret here
pub fn generate_secp256k1(input: &[u8]) -> Result<(SecretKey, PublicKey), ErrCode> {
    let secp = Secp256k1::new();
    match SecretKey::from_slice(input) {
        Ok(secret_key) => {
            let public_key = PublicKey::from_secret_key(&secp, &secret_key);
            return Ok((secret_key, public_key));
        }
        Err(_) => {
            return Err(ErrCode::PubFromPrivate(
                "Error generating private public key",
            ))
        }
    }
}

pub fn get_pri_pub_key_pair() -> (SecretKey, PublicKey) {
    let secp = Secp256k1::new();
    let mut rng = thread_rng();
    let (seckey, pubkey) = secp.generate_keypair(&mut rng);
    (seckey, pubkey)
}

pub fn decode_b16(input: &[u8]) -> Result<Vec<u8>, ErrCode> {
    match base16::decode(input) {
        Ok(val) => return Ok(val),
        Err(_) => return Err(ErrCode::PubFromPrivate("Error decoding b16")),
    }
}

pub fn encode_b16(input: &[u8]) -> String {
    let mut buffer = [0u8; 1024];
    let encoded = base16::encode_config_slice(input, base16::EncodeLower, &mut buffer);
    std::str::from_utf8(&buffer[..encoded]).unwrap().to_string()
}
