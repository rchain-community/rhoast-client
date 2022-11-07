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
        let mut hasher = Keccak256::new();

        //remove the first index of pub_key_byte
        pub_key_byte.remove(0);
        hasher.update(&pub_key_byte[..]);
        let hasher_result = hasher.finalize();
        let keccak_slice: Vec<u8> = hasher_result.into_iter().rev().take(40).collect::<Vec<_>>();
        //reverse to put back in place
        let keccak_slice_in_place: Vec<u8> = keccak_slice.into_iter().rev().collect();
        let result = format!("0x{}", encode_hex(&keccak_slice_in_place));

        return Ok(String::from(result));
    }
}

fn encode_hex(bytes: &Vec<u8>) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    for &b in bytes {
        write!(&mut s, "{:02x}", b).unwrap();
    }
    s
}
