mod error;
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

    use crate::utils::pubFromPrivate::*;
    use secp256k1::Secp256k1;

    //how to recover pub key from private key string,
    let a = get_pri_key();
    //sign pub key with private key
    let b = get_pub_key(&a);

    //convert pri key to string
    println!("{}", a.display_secret().to_string());
    let secp = Secp256k1::new();

    let signature =
        sign_recovery(&secp, a.display_secret().to_string().as_bytes(), &a[..]).unwrap();

    let (recovery_id, serialize_sig) = signature.serialize_compact();

    let pub_key = recover(
        &secp,
        a.display_secret().to_string().as_bytes(),
        &serialize_sig,
        recovery_id.to_i32() as u8,
    );
    assert_eq!(pub_key.unwrap(), b);
}
