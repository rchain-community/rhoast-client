use crate::error::ErrCode;
use crate::utils::{bytes_from_hex, keccak256};

//encode str to hex str using the func bytesFromHex::getHexSTring before passing it
pub fn get_eth_addr_from_public_key(pub_key: &str) -> Result<String, ErrCode> {
    if pub_key.len() != 130 || pub_key.len() == 0 {
        return Err(ErrCode::EthAdressFromKey(
            "Public key must contain 130 characters",
        ));
    } else {
        let mut pub_key_byte = bytes_from_hex::bytes_from_hex(&pub_key)?;

        //remove the first index of pub_key_byte
        pub_key_byte.remove(0);

        let hasher_result = hex::encode(&keccak256(&pub_key_byte));
        let len = hasher_result.len();
        let result = format!("0x{}", &hasher_result[len - 40..]);

        return Ok(String::from(result));
    }
}
