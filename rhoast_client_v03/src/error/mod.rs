use std::fmt;

#[derive(Debug)]
pub enum Error {
    GrpcUtil(&'static str),
}

#[allow(unreachable_patterns)]
// trait Display, allows Error enum to be displayed by:
//      println!("{}", error);
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Define what behaviour for each variant of the enum
        match &self {
            Error::GrpcUtil(ele) => write!(f, "GRPC error: {}", ele),
            _ => write!(f, "{:?}", self), // For any variant not previously covered
        }
    }
}
