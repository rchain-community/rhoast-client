use crate::error::ErrCode;
use crate::utils::{
    base58, bytes_from_hex,
    eth_address_from_public_key::{encode_hex, get_eth_addr_from_public_key, keccak256},
    get_blake2_hash,
};

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
//encode str to hex str using the func bytesFromHex::getHexSTring before passing it
fn hex_to_base58(hex_str: &str) -> Result<String, ErrCode> {
    let byte = bytes_from_hex::bytes_from_hex(hex_str)?;
    Ok(base58::encode(byte))
}

fn get_addr_from_eth(eth_addr: &str) -> Result<String, ErrCode> {
    if eth_addr.len() != 130 || eth_addr.len() == 0 {
        return Err(ErrCode::RevAddressFromKey(
            "Public key must contain 130 characters",
        ));
    } else {
        let prefix = Prefix {
            ..Default::default()
        };

        //hash eth key
        let eth_addr_bytes = bytes_from_hex::bytes_from_hex(eth_addr)?;
        let mut eth_hash = encode_hex(&keccak256(&eth_addr_bytes));
        eth_hash.drain(0..2);

        let payload = format!("{}{}{}", prefix.coin_id, prefix.version, eth_hash);
        let payload_byte = bytes_from_hex::bytes_from_hex(&payload)?;
        let checksum = get_blake2_hash::get_blake2_hash(&payload_byte[..], Some(32))?;

        //get the first 8 items in the str
        let checksum_str = &encode_hex(&checksum)[..8];

        return Ok(hex_to_base58(&checksum_str)?);
    }
}

pub fn rev_address_from_public_key(pub_key: &str) -> Result<String, ErrCode> {
    let mut eth_addr_without_prefix = get_eth_addr_from_public_key(pub_key)?;
    eth_addr_without_prefix.drain(0..2);
    Ok(get_addr_from_eth(&eth_addr_without_prefix)?)
}
