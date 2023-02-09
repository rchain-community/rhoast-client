use crate::error::Error;

pub fn decode(input: &str) -> Result<Vec<u8>, Error> {
    match bs58::decode(input).into_vec() {
        Ok(decoded) => Ok(decoded),
        Err(_) => {
            let err_msg = format!("Error decoding {} to vec", input);
            Err(Error::Bs58(string_to_static_str(err_msg)))
        }
    }
}

pub fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}

pub fn encode(input: Vec<u8>) -> String {
    let encoded = bs58::encode(input).into_string();
    encoded
}
