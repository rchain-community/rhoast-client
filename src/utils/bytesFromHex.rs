pub fn getHexSTring(input: &str)->String{
    let hex_string = hex::encode(input);
    hex_string
}

pub fn bytesFromHex(hexStr: &str)->Vec<u8>{
    //decode hex tring to utf8
    let decoded_string = String::from_utf8(hex::decode(hexStr).unwrap()).unwrap();
    //write to byte
    let mut bytes = vec![0; decoded_string.len()];
    hex::decode_to_slice(hexStr, &mut bytes as &mut [u8]).unwrap();
    bytes
}