use crate::error::ErrCode;
use secp256k1::rand::thread_rng;
use secp256k1::{PublicKey, Error, Secp256k1, Message, SecretKey, Signing, Verification, ecdsa};
use bitcoin_hashes::{sha256, Hash};

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

pub fn recover<C: Verification>(secp: &Secp256k1<C>,msg: &[u8],sig: &[u8],recovery_id: u8) -> Result<PublicKey, Error> {
    let msg = sha256::Hash::hash(msg);
    let msg = Message::from_slice(&msg)?;
    let id = ecdsa::RecoveryId::from_i32(recovery_id as i32)?;
    let sig = ecdsa::RecoverableSignature::from_compact(&sig, id)?;

    secp.recover_ecdsa(&msg, &sig)
}

pub fn sign_recovery<C: Signing>(secp: &Secp256k1<C>, msg: &[u8], seckey: &[u8]) -> Result<ecdsa::RecoverableSignature, Error> {
    let msg = sha256::Hash::hash(msg);
    let msg = Message::from_slice(&msg)?;
    let seckey = SecretKey::from_slice(&seckey)?;
    Ok(secp.sign_ecdsa_recoverable(&msg, &seckey))
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
