mod models;
mod proto;
mod utils;
// use crate::proto::{casper, casper::casper_v1, routing, scalapb};
use crate::proto::{casper::casper_v1::*, routing::*, casper::*, scalapb::*};

fn main() {
    println!("hello world!");
    ServiceError{
        messages:vec!["hello".to_string()]
    };
    HeartbeatResponse{};
    PCost{
        cost:34
    };
}
