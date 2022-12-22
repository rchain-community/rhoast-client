use crate::error::Error;
use bitcoin_hashes::{sha256, Hash};
use hex::FromHex;
use secp256k1::rand::thread_rng;
use secp256k1::{
    constants::SECRET_KEY_SIZE, ecdsa, Message, PublicKey, Secp256k1, SecretKey, Signing,
    Verification,
};

//use secret gotten from get_pri_pub_key_pair() to create new secret here
///
/// ```
/// use crate::utils::pub_from_private::{get_seckey_from_string,get_pub_key};
/// let seckey=get_seckey_from_string("secret key").unwrap();
/// let pub_key=get_pub_key(&seckey)
/// ```
/// 
pub fn get_pub_key(secret_key: &SecretKey) -> PublicKey {
    let secp = Secp256k1::new();

    PublicKey::from_secret_key(&secp, secret_key)
}
///
/// ```
/// use crate::utils::pub_from_private::{get_seckey_from_string,get_pri_key};
/// let seckey=get_seckey_from_string("secret key").unwrap();
/// let pri_key=get_pri_key(&seckey)
/// ````
/// 
pub fn get_pri_key() -> SecretKey {
    let secp = Secp256k1::new();
    let mut rng = thread_rng();
    let (seckey, _) = secp.generate_keypair(&mut rng);
    seckey
}

pub fn get_seckey_from_string(input: &str) -> Result<SecretKey, Error> {
    match <[u8; SECRET_KEY_SIZE]>::from_hex(input) {
        Ok(buffer) => match SecretKey::from_slice(&buffer) {
            Ok(sec_key) => return Ok(sec_key),
            Err(_) => Err(Error::PubFromPrivate("error writing slice to secret key ")),
        },
        Err(_) => Err(Error::PubFromPrivate("error writing hex to buffer ")),
    }
}

pub fn recover<C: Verification>(
    secp: &Secp256k1<C>,
    msg: &[u8],
    sig: &[u8],
    recovery_id: u8,
) -> Result<PublicKey, Error> {
    let msg = sha256::Hash::hash(msg);
    match Message::from_slice(&msg) {
        Ok(msg) => match ecdsa::RecoveryId::from_i32(recovery_id as i32) {
            Ok(id) => match ecdsa::RecoverableSignature::from_compact(&sig, id) {
                Ok(sig) => {
                    if let Ok(pub_key) = secp.recover_ecdsa(&msg, &sig) {
                        Ok(pub_key)
                    } else {
                        Err(Error::PubFromPrivate("error getting pub key "))
                    }
                }
                Err(_) => Err(Error::PubFromPrivate(
                    "error converting compact-encoded byte slice to a signature ",
                )),
            },
            Err(_) => return Err(Error::PubFromPrivate("error creating recovery id ")),
        },
        Err(_) => return Err(Error::PubFromPrivate("error getting message from slice ")),
    }
}

pub fn sign_recovery<C: Signing>(
    secp: &Secp256k1<C>,
    msg: &[u8],
    seckey: &[u8],
) -> Result<ecdsa::RecoverableSignature, Error> {
    let msg = sha256::Hash::hash(msg);
    match Message::from_slice(&msg) {
        Ok(msg) => match SecretKey::from_slice(&seckey) {
            Ok(seckey) => return Ok(secp.sign_ecdsa_recoverable(&msg, &seckey)),
            Err(_) => Err(Error::PubFromPrivate(
                "error getting secret key from slice ",
            )),
        },
        Err(_) => return Err(Error::PubFromPrivate("error getting message from slice ")),
    }
}
