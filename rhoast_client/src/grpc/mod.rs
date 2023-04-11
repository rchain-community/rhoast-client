#![allow(non_camel_case_types)]
pub mod deploy;
pub mod propose;

#[derive(Debug)]
pub enum GrpcVersion {
    v0_12_x(GrpcV0_12),
    v0_13_x(GrpcV0_13),
}

#[derive(Debug)]
pub struct GrpcV0_12 {
    pub host: String,
}

#[derive(Debug)]
pub struct GrpcV0_13 {
    pub host: String,
}

pub fn create_grpc_client(version: &str, host: &str) -> GrpcVersion {
    if version.contains("0.13") {
        GrpcVersion::v0_13_x(GrpcV0_13 {
            host: host.to_string(),
        })
    } else {
        GrpcVersion::v0_12_x(GrpcV0_12 {
            host: host.to_string(),
        })
    }
}
