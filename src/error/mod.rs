use std::fmt;

#[derive(Debug)]
pub enum ErrCode {
    Bs58(&'static str),
    Blake2(&'static str),
    EthAdressFromKey(&'static str),
    RevAddressFromKey(&'static str),
    PubFromPrivate(&'static str),
    DeployUtil(&'static str),
    HttpUtil(&'static str),
    GrpcUtil(&'static str),
}

#[allow(unreachable_patterns)]
// trait Display, allows Errcode enum to be displayed by:
//      println!("{}", error);
impl fmt::Display for ErrCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Define what behaviour for each variant of the enum
        match &self {
            ErrCode::DeployUtil(ele) => write!(f, "Deploy util error: {}", ele),
            ErrCode::Bs58(ele) => write!(f, "BS58 error: {}", ele),
            ErrCode::Blake2(ele) => write!(f, "Blake2 error: {}", ele),
            ErrCode::HttpUtil(ele) => write!(f, "HTTP error: {}", ele),
            ErrCode::EthAdressFromKey(ele) => write!(f, "EthAdressFromKey error: {}", ele),
            ErrCode::RevAddressFromKey(ele) => write!(f, "RevAddressFromKey error: {}", ele),
            ErrCode::PubFromPrivate(ele) => write!(f, "PubFromPrivate error: {}", ele),
            ErrCode::GrpcUtil(ele) => write!(f, "GRPC error: {}", ele),
            _ => write!(f, "{:?}", self), // For any variant not previously covered
        }
    }
}
