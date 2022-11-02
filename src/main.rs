mod models;
mod proto;
mod utils;
// use crate::proto::{casper, casper::casper_v1, routing, scalapb};
use crate::proto::{
    casper::casper_v1::*, casper::*, casper_msg::*, deploy::*, deployv1::ServiceError, routing::*,
    scalapb::*,
};

use crate::proto::deployv1::PrivateNamePreviewPayload;

fn main() {
    println!("hello world!");
    ServiceError {
        messages: vec!["hello".to_string()],
    };
    HeartbeatResponse {};
    PCost { cost: 34 };
    HasBlockProto { hash: vec![] };
    PrintUnmatchedSendsQuery {
        print_unmatched_sends: false,
    };
    IsFinalizedQuery {
        hash: "".to_string(),
    };
    PrivateNamePreviewPayload { ids: vec![] };
}
