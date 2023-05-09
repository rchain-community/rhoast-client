pub mod deploy;
pub mod propose;
#[derive(Debug)]
pub struct Grpc {
    pub host: String,
}

pub fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}
