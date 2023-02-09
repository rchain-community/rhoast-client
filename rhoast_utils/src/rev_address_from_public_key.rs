use crate::error::Error;
use crate::{
    base58, decode_b16, encode_b16, eth_address_from_public_key::get_eth_addr_from_public_key,
    get_blake2_hash, keccak256, pub_from_private::*, remove_0x,
};
use secp256k1::SecretKey;

struct Prefix {
    coin_id: String,
    version: String,
}

#[derive(Debug)]
pub struct RevAddress {
    pub rev_addr: String,
    pub eth_addr: String,
    pub pub_key: String,
    pub pri_key: String,
}

impl RevAddress {
    fn new(rev_addr: &str, eth_addr: &str, pub_key: &str, pri_key: &SecretKey) -> Self {
        //extract secret key
        let pri_key = pri_key.display_secret().to_string();
        //uncompress pub key and encode it
        let pub_key = pub_key;
        //return rev adress details
        RevAddress {
            rev_addr: rev_addr.to_string(),
            eth_addr: eth_addr.to_string(),
            pub_key: pub_key.to_string(),
            pri_key,
        }
    }
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
/// Get rev addr from Eth addr
///
/// ```no_run
/// use rhoast_utils::rev_address_from_public_key::get_rev_addr_from_eth;
///
/// let rev_addr =get_rev_addr_from_eth("Eth addr").unwrap();
/// print!("{rev_addr}");
/// ````
///
pub fn get_rev_addr_from_eth(eth_addr_raw: &str) -> Result<String, Error> {
    let eth_addr = remove_0x(eth_addr_raw);
    if eth_addr.len() != 40 {
        Err(Error::RevAddressFromKey(
            "ETH address must contain 40 characters",
        ))
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
        Ok(base58::encode(decode_b16(addr.as_bytes())?))
    }
}

//get rev addr from pub key
/// Get rev addr from public key
/// ```no_run
/// use rhoast_utils::rev_address_from_public_key::rev_address_from_public_key;
///
/// let rev=rev_address_from_public_key("public key").unwrap();
/// print!("{rev}");
/// ````
///
pub fn rev_address_from_public_key(pub_key: &str) -> Result<String, Error> {
    let eth_addr = get_eth_addr_from_public_key(pub_key)?;
    get_rev_addr_from_eth(&eth_addr)
}

/// Get rev address from private key
///
///```no_run
/// use rhoast_utils::{rev_address_from_public_key::get_rev_addr_from_private_key, pub_from_private::get_seckey_from_string};
///
/// let sec_key=get_seckey_from_string("private key").unwrap();
/// let pub_key=get_rev_addr_from_private_key(&sec_key).unwrap();
///```
///
pub fn get_rev_addr_from_private_key(key: &SecretKey) -> Result<String, Error> {
    let pub_key = get_pub_key(key);
    rev_address_from_public_key(&pub_key)
}

/// Generate new rev address
///
///
/// ```no_run
/// use rhoast_utils::rev_address_from_public_key::get_new_rev_address;
///
/// let rev_details = get_new_rev_address().unwrap();
/// println!("rev details: {:?}", rev_details);
/// ```
///
///
pub fn get_new_rev_address() -> Result<RevAddress, Error> {
    let private_key = get_pri_key();
    //use private key to sign pub key
    let publick_key = get_pub_key(&private_key);
    let pub_key_hex = &publick_key;
    //get eth addr from public key
    let eth_addr = get_eth_addr_from_public_key(pub_key_hex)?;
    //get rev addr from pub key
    let rev_addr = rev_address_from_public_key(pub_key_hex)?;
    Ok(RevAddress::new(
        &rev_addr,
        &eth_addr,
        &publick_key,
        &private_key,
    ))
}

/// Validate a rev address
///
/// ```no_run
/// use rhoast_utils::rev_address_from_public_key::verify_rev_addr;
///
/// let valid = verify_rev_addr("valid_rev_addr").unwrap();
/// ```
///
///
pub fn verify_rev_addr(rev_addr_raw: &str) -> Result<bool, Error> {
    let rev_byte = base58::decode(rev_addr_raw)?;
    if rev_byte.is_empty() {
        Ok(false)
    } else {
        let rev_hex = encode_b16(&rev_byte);
        let len = rev_hex.len();

        let payload = &rev_hex[0..len - 8];
        let checksum = &rev_hex[len - 8..];

        let payload_bytes = decode_b16(payload.as_bytes())?;
        let checksum_cal =
            &hex::encode(get_blake2_hash::get_blake2_hash(&payload_bytes, Some(32))?)[..8];

        Ok(checksum_cal == checksum)
    }
}
