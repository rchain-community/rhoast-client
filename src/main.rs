mod models;
mod proto;
mod utils;
use crate::proto::{
    casper::casper_v1::*, casper::*, casper_msg::*, deploy::*, deployv1::ServiceError, routing::*,
    scalapb::*,
};

use crate::proto::deployv1::PrivateNamePreviewPayload;
use crate::utils::getBlake2Hash;

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
    //sample usage
    println!(
        "{:?}",
        getBlake2Hash::getBlake2Hash(&[1, 4, 54, 67], Some(9))
    );
}
