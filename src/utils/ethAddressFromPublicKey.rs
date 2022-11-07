use crate::error::ErrCode;
use crate::utils::bytesFromHex;
use sha3::{Digest, Keccak256};
use std::fmt::Write;

//encode str to hex str using the func bytesFromHex::getHexSTring before passing it
pub fn getEthAddrFromPublicKey(pub_key: &str) -> Result<String, ErrCode> {
    if pub_key.len() != 130 || pub_key.len() == 0 {
        return Err(ErrCode::EthAdressFromKey(
            "Public key must contain 130 characters",
        ));
    } else {
        let mut pub_key_byte = bytesFromHex::bytesFromHex(&pub_key)?;

        //remove the first index of pub_key_byte
        pub_key_byte.remove(0);

        let hasher_result = encode_hex(&keccak256(&pub_key_byte));
        let len = hasher_result.len();
        let result = format!("0x{}", &hasher_result[len - 40..]);

        return Ok(String::from(result));
    }
}

pub fn encode_hex(bytes: &Vec<u8>) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    for &b in bytes {
        write!(&mut s, "{:02x}", b).unwrap();
    }
    s
}

pub fn keccak256(data: &Vec<u8>) -> Vec<u8> {
    let mut hasher = Keccak256::new();
    hasher.update(&data[..]);
    let hasher_result = hasher.finalize().to_vec();
    hasher_result
}
