use crate::models::model::{DeployData, DeployDataPayload, DeployDataRequest};
use crate::proto::{casper::DataWithBlockInfo, casper::Par};
use bitcoin_hashes::{sha256, Hash};
use rhoast_utils::error::Error;
use rhoast_utils::pub_from_private::{get_pub_key, get_seckey_from_string};
use rhoast_utils::{get_blake2_hash::get_blake2_hash, SIG_ALGORITHM};
use secp256k1::{ecdsa, Message, PublicKey, Secp256k1, SecretKey, Signing, Verification};

pub fn get_first_block(block_info: &Vec<DataWithBlockInfo>) -> &DataWithBlockInfo {
    &block_info[0]
}

pub fn get_value_from_block(block_info: &Vec<DataWithBlockInfo>) -> Option<&Par> {
    for i in 0..block_info.len() {
        let block = &block_info[i];
        if block.post_block_data.len() > 0 {
            for j in 0..block.post_block_data.len() {
                let data = &block.post_block_data[j];
                return Some(data);
            }
        }
    }
    None
}

//pass in secret key as a byte slice
pub fn sign_secp_256k1<C: Signing + Verification>(
    secp: &Secp256k1<C>,
    msg: &[u8],
    seckey: &[u8],
) -> Result<ecdsa::SerializedSignature, Error> {
    let msg = sha256::Hash::hash(msg);
    match Message::from_slice(&msg) {
        Ok(msg) => match SecretKey::from_slice(&seckey) {
            Ok(seckey) => {
                let sig = secp.sign_ecdsa(&msg, &seckey);
                let pubkey = PublicKey::from_secret_key(secp, &seckey);
                match secp.verify_ecdsa(&msg, &sig, &pubkey) {
                    Ok(_) => Ok(sig.serialize_der()),
                    Err(_) => Err(Error::DeployUtil("Failed to verify signature")),
                }
            }
            Err(_) => Err(Error::DeployUtil("error writing secret to slice")),
        },
        Err(_) => Err(Error::DeployUtil("error writing message to slice")),
    }
}

unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    ::std::slice::from_raw_parts((p as *const T) as *const u8, ::std::mem::size_of::<T>())
}

pub fn get_deploy_data(payload: &DeployDataPayload) -> Result<DeployDataRequest, Error> {
    //sign sec key
    let sec_key_hash = get_seckey_from_string(&payload.private_key)?;
    //get public key
    let pub_key = get_pub_key(&sec_key_hash);

    let deploy_data = DeployData {
        timestamp: payload.timestamp,
        term: payload.term.to_string(),
        shard_id: payload.shard_id.to_string(),
        phlo_price: payload.phlo_price,
        phlo_limit: payload.phlo_limit,
        valid_after_block_number: payload.valid_after_block_number,
    };
    let to_sign = unsafe { any_as_u8_slice(&deploy_data) };
    let hash = get_blake2_hash(&to_sign, Some(32))?;
    let secp = Secp256k1::new();
    let signature = sign_secp_256k1(&secp, &hash, &sec_key_hash[..])?.to_string();

    Ok(DeployDataRequest {
        data: deploy_data,
        deployer: pub_key.to_string(),
        signature: signature,
        sig_algorithm: SIG_ALGORITHM.to_string(),
    })
}
