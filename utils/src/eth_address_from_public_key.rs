use crate::error::Error;
use crate::{decode_b16, keccak256};

//get eth address provided the uncompressed pub key
/// to convert from public key to eth addr
///
///
///
/// ```no_run
/// use utils::eth_address_from_public_key::get_eth_addr_from_public_key;
///
/// let eth= get_eth_addr_from_public_key("").unwrap();
/// println!("{eth}");
/// ```
///

pub fn get_eth_addr_from_public_key(pub_key: &str) -> Result<String, Error> {
    if pub_key.len() != 130 {
        Err(Error::EthAdressFromKey(
            "Public key must contain 130 characters",
        ))
    } else {
        let mut pub_key_byte = decode_b16(pub_key.as_bytes())?;

        //remove the first index of pub_key_byte
        pub_key_byte.remove(0);

        let hasher_result = hex::encode(&keccak256(&pub_key_byte));
        let len = hasher_result.len();
        let result = format!("0x{}", &hasher_result[len - 40..]);

        Ok(result)
    }
}
