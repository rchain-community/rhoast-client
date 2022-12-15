use std::fmt;

#[derive(Debug)]
pub enum Error {
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
// trait Display, allows Error enum to be displayed by:
//      println!("{}", error);
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Define what behaviour for each variant of the enum
        match &self {
            Error::DeployUtil(ele) => write!(f, "Deploy util error: {}", ele),
            Error::Bs58(ele) => write!(f, "BS58 error: {}", ele),
            Error::Blake2(ele) => write!(f, "Blake2 error: {}", ele),
            Error::HttpUtil(ele) => write!(f, "HTTP error: {}", ele),
            Error::EthAdressFromKey(ele) => write!(f, "EthAdressFromKey error: {}", ele),
            Error::RevAddressFromKey(ele) => write!(f, "RevAddressFromKey error: {}", ele),
            Error::PubFromPrivate(ele) => write!(f, "PubFromPrivate error: {}", ele),
            Error::GrpcUtil(ele) => write!(f, "GRPC error: {}", ele),
            _ => write!(f, "{:?}", self), // For any variant not previously covered
        }
    }
}
