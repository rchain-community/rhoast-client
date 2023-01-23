use crate::proto::{
    casper::casper_v1::*, casper::*, casper_msg::*, deploy::*, deployv1::ServiceError, routing::*,
    scalapb,
};

use crate::proto::deployv1::PrivateNamePreviewPayload;
use crate::utils::get_blake2_hash;

pub fn run() {
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

    b.to_string();

    //
    let k = get_seckey_from_string(&a.display_secret().to_string()).unwrap();

    assert_eq!(&a[..], &k[..]);

    let secp = Secp256k1::new();
    //use recoverd se key to recover pub key
    let signature = sign_recovery(&secp, b"hi", &k[..]).unwrap();

    let (recovery_id, serialize_sig) = signature.serialize_compact();

    let pub_key = recover(&secp, b"hi", &serialize_sig, recovery_id.to_i32() as u8);
    assert_eq!(pub_key.unwrap(), b);

    use crate::utils::{
        eth_address_from_public_key::get_eth_addr_from_public_key, rev_address_from_public_key::*,
    };

    println!(
        "rev addr: {:?}",
        get_addr_from_eth("d7510a2b3761de03c8697cf2fc48dc86e5359462").unwrap()
    );

    //to use pub key, first uncompress it, then encode to hex format
    println!(
        "uncompressed private key {:?}",
        hex::encode(&b.serialize_uncompressed())
    );
    println!(
        "etth addr from pub key {}",
        get_eth_addr_from_public_key(&hex::encode(&b.serialize_uncompressed())).unwrap()
    );

    //get rev address from pub key
    println!(
        "rev from pub key {:?}",
        rev_address_from_public_key(&hex::encode(&b.serialize_uncompressed())).unwrap()
    );

    //get rev addr frorm private key
    println!(
        "rev from private key {:?}",
        get_rev_addr_from_private_key(
            &get_seckey_from_string(
                "ff845d703de76008c3c807282b73682fb93d10eac223dd4c71ff6defe8b76523"
            )
            .unwrap()
        )
    );

    //get new rev addr
    println!("new rev addr {:?}", get_new_rev_address());

    //verify rev addr
    println!(
        "verify rev addr {:?}",
        verify_rev_addr("11112DHYtii3vQBmzZdozoTwprdomN3rhNFvb77DPiviEwFAR8zGNX").unwrap()
    );
}
