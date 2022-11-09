use crate::error::ErrCode;
use bitcoin_hashes::{sha256, Hash};
use hex::FromHex;
use secp256k1::rand::thread_rng;
use secp256k1::{ecdsa, constants::SECRET_KEY_SIZE, Message, PublicKey, Secp256k1, SecretKey, Signing, Verification};

//use secret gotten from get_pri_pub_key_pair() to create new secret here
pub fn get_pub_key(secret_key: &SecretKey) -> PublicKey {
    let secp = Secp256k1::new();

    let public_key = PublicKey::from_secret_key(&secp, &secret_key);
    public_key
}

pub fn get_pri_key() -> SecretKey {
    let secp = Secp256k1::new();
    let mut rng = thread_rng();
    let (seckey, _) = secp.generate_keypair(&mut rng);
    seckey
}

pub fn get_seckey_from_string(input: &str) -> Result<SecretKey, ErrCode> {
    match <[u8; SECRET_KEY_SIZE]>::from_hex(input) {
        Ok(buffer) =>{
            return Ok(SecretKey::from_slice(&buffer).unwrap())
        },
        Err(_) => {
            return Err(ErrCode::PubFromPrivate("error writing hex to buffer "));
        }
    }
}

pub fn recover<C: Verification>(
    secp: &Secp256k1<C>,
    msg: &[u8],
    sig: &[u8],
    recovery_id: u8,
) -> Result<PublicKey, ErrCode> {
    let msg = sha256::Hash::hash(msg);
    match Message::from_slice(&msg) {
        Ok(msg) => match ecdsa::RecoveryId::from_i32(recovery_id as i32) {
            Ok(id) => match ecdsa::RecoverableSignature::from_compact(&sig, id) {
                Ok(sig) => {
                    if let Ok(pub_key) = secp.recover_ecdsa(&msg, &sig) {
                        return Ok(pub_key);
                    } else {
                        return Err(ErrCode::PubFromPrivate("error getting pub key "));
                    }
                }
                Err(_) => {
                    return Err(ErrCode::PubFromPrivate(
                        "error converting compact-encoded byte slice to a signature ",
                    ))
                }
            },
            Err(_) => return Err(ErrCode::PubFromPrivate("error creating recovery id ")),
        },
        Err(_) => return Err(ErrCode::PubFromPrivate("error getting message from slice ")),
    }
}

pub fn sign_recovery<C: Signing>(
    secp: &Secp256k1<C>,
    msg: &[u8],
    seckey: &[u8],
) -> Result<ecdsa::RecoverableSignature, ErrCode> {
    let msg = sha256::Hash::hash(msg);
    match Message::from_slice(&msg) {
        Ok(msg) => match SecretKey::from_slice(&seckey) {
            Ok(seckey) => return Ok(secp.sign_ecdsa_recoverable(&msg, &seckey)),
            Err(_) => {
                return Err(ErrCode::PubFromPrivate(
                    "error getting secret key from slice ",
                ))
            }
        },
        Err(_) => return Err(ErrCode::PubFromPrivate("error getting message from slice ")),
    }
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
