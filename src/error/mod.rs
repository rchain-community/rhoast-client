use std::fmt;

#[derive(Debug)]
pub enum ErrCode {
    ByteFromHex(&'static str),
    Bs58(&'static str),
    Blake2(&'static str),
    EthAdressFromKey(&'static str),
    RevAddressFromKey(&'static str),
}

#[allow(unreachable_patterns)]
// trait Display, allows Errcode enum to be displayed by:
//      println!("{}", error);
impl fmt::Display for ErrCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Define what behaviour for each variant of the enum
        match &self {
            ErrCode::ByteFromHex(ele) => write!(f, "Byte From Hex error: {}", ele),
            ErrCode::Bs58(ele) => write!(f, "BS58 error: {}", ele),
            ErrCode::Blake2(ele) => write!(f, "Blake2 error: {}", ele),
            ErrCode::EthAdressFromKey(ele) => write!(f, "EthAdressFromKey error: {}", ele),
            ErrCode::RevAddressFromKey(ele) => write!(f, "RevAddressFromKey error: {}", ele),
            _ => write!(f, "{:?}", self), // For any variant not previously covered
        }
    }
}
