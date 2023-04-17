#![allow(non_camel_case_types)]
pub mod deploy;
pub mod propose;

#[derive(Debug)]
pub struct GrpcV0_12 {
    pub host: String,
}

#[derive(Debug)]
pub struct GrpcV0_13 {
    pub host: String,
}
