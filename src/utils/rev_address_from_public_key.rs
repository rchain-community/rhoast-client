use crate::error::ErrCode;
use crate::utils::{
    base58, decode_b16, eth_address_from_public_key::get_eth_addr_from_public_key,
    get_blake2_hash, keccak256, remove_0x, pub_from_private::get_pub_key
};
use secp256k1::SecretKey;

struct Prefix {
    coin_id: String,
    version: String,
}

impl Default for Prefix {
    fn default() -> Prefix {
        Prefix {
            coin_id: String::from("000000"),
            version: String::from("00"),
        }
    }
}

//get rev addr from eth addr
pub fn get_addr_from_eth(eth_addr_raw: &str) -> Result<String, ErrCode> {
    let eth_addr = remove_0x(eth_addr_raw);
    if eth_addr.len() != 40 {
        return Err(ErrCode::RevAddressFromKey(
            "ETH address must contain 40 characters",
        ));
    } else {
        let prefix = Prefix {
            ..Default::default()
        };
        //hash eth addr
        let eth_addr_byte = decode_b16(eth_addr.as_bytes())?;
        let eth_hash = hex::encode(keccak256(&eth_addr_byte));

        // Add prefix with hash and calculate checksum (blake2b-256 hash)
        let payload = format!("{}{}{}", prefix.coin_id, prefix.version, eth_hash);
        let payload_bytes = decode_b16(payload.as_bytes())?;
        let checksum = hex::encode(get_blake2_hash::get_blake2_hash(&payload_bytes, Some(32))?);

        let addr = format!("{}{}", payload, &checksum[..8]);
        // Return REV address
        Ok(base58::encode(decode_b16(&addr.as_bytes().to_vec())?))
    }
}

//get rev addr from pub key
pub fn rev_address_from_public_key(pub_key: &str) -> Result<String, ErrCode> {
    let eth_addr = get_eth_addr_from_public_key(pub_key)?;
    Ok(get_addr_from_eth(&eth_addr)?)
}

//get rev address from private key
pub fn get_rev_addr_from_private_key(key: &SecretKey)-> Result<String, ErrCode>{
    let pub_key = get_pub_key(key);
    rev_address_from_public_key(&hex::encode(pub_key.serialize_uncompressed()))
}