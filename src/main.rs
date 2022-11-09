mod error;
mod models;
mod proto;
mod utils;
use crate::proto::{
    casper::casper_v1::*, casper::*, casper_msg::*, deploy::*, deployv1::ServiceError, routing::*,
    scalapb,
};

use crate::proto::deployv1::PrivateNamePreviewPayload;
use crate::utils::get_blake2_hash;

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
        get_blake2_hash::get_blake2_hash(&[1, 4, 54, 67], Some(9))
    );

    use crate::utils::pub_from_private::*;
    use secp256k1::Secp256k1;

    //how to recover pub key from private key string,
    let a = get_pri_key();
    //sign pub key with private key
    let b = get_pub_key(&a);

    //
    let k = get_seckey_from_string(&a.display_secret().to_string()).unwrap();

    println!("buffers {:?}---{:?}", &a[..], &k[..]);

    let secp = Secp256k1::new();
    //use recoverd se key to recover pub key
    let signature = sign_recovery(&secp, b"hi", &k[..]).unwrap();

    let (recovery_id, serialize_sig) = signature.serialize_compact();

    let pub_key = recover(&secp, b"hi", &serialize_sig, recovery_id.to_i32() as u8);
    assert_eq!(pub_key.unwrap(), b);
}
