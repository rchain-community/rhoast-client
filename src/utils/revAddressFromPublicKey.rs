use crate::error::ErrCode;
use crate::utils::{
    base58, bytesFromHex,
    ethAddressFromPublicKey::{encode_hex, getEthAddrFromPublicKey, keccak256},
    getBlake2Hash,
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
fn hexToBase58(hex_str: &str) -> Result<String, ErrCode> {
    let byte = bytesFromHex::bytesFromHex(hex_str)?;
    Ok(base58::encode(byte))
}

fn getAddrFromEth(eth_addr: &str) -> Result<String, ErrCode> {
    if eth_addr.len() != 130 || eth_addr.len() == 0 {
        return Err(ErrCode::RevAddressFromKey(
            "Public key must contain 130 characters",
        ));
    } else {
        let prefix = Prefix {
            ..Default::default()
        };

        //hash eth key
        let eth_addr_bytes = bytesFromHex::bytesFromHex(eth_addr)?;
        let mut eth_hash = encode_hex(&keccak256(&eth_addr_bytes));
        eth_hash.drain(0..2);

        let payload = format!("{}{}{}", prefix.coin_id, prefix.version, eth_hash);
        let payload_byte = bytesFromHex::bytesFromHex(&payload)?;
        let checksum = getBlake2Hash::getBlake2Hash(&payload_byte[..], Some(32))?;

        //get the first 8 items in the str
        let checksum_str = &encode_hex(&checksum)[..8];

        return Ok(hexToBase58(&checksum_str)?);
    }
}

pub fn revAddressFromPublicKey(pub_key: &str) -> Result<String, ErrCode> {
    let mut eth_addr_without_prefix = getEthAddrFromPublicKey(pub_key)?;
    eth_addr_without_prefix.drain(0..2);
    Ok(getAddrFromEth(&eth_addr_without_prefix)?)
}
