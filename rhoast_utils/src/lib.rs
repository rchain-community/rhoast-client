//! This crate contains utility methods for the rhoast client
//! 1. Get Eth addr from public key
//! 2. Get private key
//! 3. Get public key
//! 4. Get rev address
//! 5. Get rev address from ETH address
//! 6. Get rev address from public key
//! 7. Get rev address from private key
//! 8. Verify rev address

use crate::error::Error;
use sha3::{Digest, Keccak256};

mod base58;
pub mod error;
pub mod eth_address_from_public_key;
pub mod get_blake2_hash;
pub mod pub_from_private;
pub mod rev_address_from_public_key;
pub const SIG_ALGORITHM: &str = "secp256k1";

fn remove_0x(input: &str) -> String {
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

fn decode_b16(input: &[u8]) -> Result<Vec<u8>, Error> {
    match base16::decode(input) {
        Ok(val) => Ok(val),
        Err(_) => Err(Error::PubFromPrivate("Error decoding b16")),
    }
}

fn encode_b16(input: &[u8]) -> String {
    base16::encode_config(input, base16::EncodeLower)
}

fn keccak256(data: &[u8]) -> Vec<u8> {
    let mut hasher = Keccak256::new();
    hasher.update(&data[..]);
    hasher.finalize().to_vec()
}
