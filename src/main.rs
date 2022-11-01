mod models;
mod proto;
mod utils;
// use crate::proto::{casper, casper::casper_v1, routing, scalapb};
use crate::proto::{
    casper_msg::*, casper::casper_v1::*, casper::*, routing::*, scalapb::*,
    deploy::*, deployv1::{ServiceError as serv_err}
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
    PrintUnmatchedSendsQuery{
        print_unmatched_sends: false
    };
    IsFinalizedQuery{
        hash: "".to_string()
    };
    PrivateNamePreviewPayload{
       ids: vec![]
    };

}
