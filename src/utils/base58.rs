use crate::error::ErrCode;

pub fn decode(input: &str) -> Result<Vec<u8>, ErrCode> {
    match bs58::decode(input).into_vec() {
        Ok(decoded) => {
            return Ok(decoded);
        }
        Err(_) => {
            return Err(ErrCode::Bs58("Error decoding to vec"));
        }
    }
}

pub fn encode(input: Vec<u8>) -> String {
    let encoded = bs58::encode(input).into_string();
    encoded
}
