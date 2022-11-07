use crate::error::ErrCode;

pub fn getHexSTring(input: &str) -> String {
    let hex_string = hex::encode(input);
    hex_string
}

pub fn bytesFromHex(hexStr: &str) -> Result<Vec<u8>, ErrCode> {
    //decode hex string to utf8
    match hex::decode(hexStr) {
        Ok(hex_value) => {
            match String::from_utf8(hex_value) {
                Ok(decoded_string) => {
                    //write to byte
                    let mut bytes = vec![0; decoded_string.len()];
                    if let Err(_) = hex::decode_to_slice(hexStr, &mut bytes as &mut [u8]) {
                        return Err(ErrCode::ByteFromHex(
                            "Error decoding a hex string into a mutable bytes slice",
                        ));
                    };
                    return Ok(bytes);
                }
                Err(_) => {
                    return Err(ErrCode::ByteFromHex("Error converting vec byte string"));
                }
            }
        }
        Err(_) => {
            return Err(ErrCode::ByteFromHex("Error decoding hex string"));
        }
    }
}
