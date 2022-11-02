pub fn decode(input: &str) -> Vec<u8> {
    let decoded = bs58::decode("he11owor1d").into_vec().unwrap();
    decoded
}

pub fn encode(input: Vec<u8>) -> String {
    let encoded = bs58::encode(input).into_string();
    encoded
}
