pub mod base58;
pub mod eth_address_from_public_key;
pub mod get_blake2_hash;
pub mod pub_from_private;
pub mod rev_address_from_public_key;
pub const SIG_ALGORITHM: &str = "secp256k1";

use crate::error::ErrCode;
use sha3::{Digest, Keccak256};

pub fn remove_0x(input: &str) -> String {
    let mut a = input.to_string();
    if input.chars().nth(0).unwrap().to_string() == "0"
        && input.chars().nth(1).unwrap().to_string() == "x"
    {
        a.remove(0);
        a.remove(0);
        a
    } else {
        a
    }
}

pub fn decode_b16(input: &[u8]) -> Result<Vec<u8>, ErrCode> {
    match base16::decode(input) {
        Ok(val) => Ok(val),
        Err(_) => Err(ErrCode::PubFromPrivate("Error decoding b16")),
    }
}

pub fn encode_b16(input: &[u8]) -> String {
    base16::encode_config(input, base16::EncodeLower)
}

pub fn keccak256(data: &[u8]) -> Vec<u8> {
    let mut hasher = Keccak256::new();
    hasher.update(&data[..]);
    hasher.finalize().to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 5);
    }
}
