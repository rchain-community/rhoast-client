use csv;
use rhoast_utils::eth_address_from_public_key::get_eth_addr_from_public_key;
use rhoast_utils::pub_from_private::{get_pri_key, get_pub_key, get_seckey_from_string};
use rhoast_utils::rev_address_from_public_key::{
    get_new_rev_address, get_rev_addr_from_eth, get_rev_addr_from_private_key,
    rev_address_from_public_key, verify_rev_addr,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct TestKey {
    pk: String,
    pubk: String,
    eth: String,
    rev: String,
}

fn get_keys() -> Vec<TestKey> {
    let mut returned_vec: Vec<TestKey> = vec![];
    let mut rdr = csv::Reader::from_path("tests/test_keys/testing-keys.csv").unwrap();
    for result in rdr.deserialize() {
        let record: TestKey = result.unwrap();
        returned_vec.push(record);
    }
    returned_vec
}

#[test]
fn test_eth_from_public_key() {
    let keys: Vec<TestKey> = get_keys();
    let eth = get_eth_addr_from_public_key(&keys[0].pubk).unwrap();
    assert_eq!(eth, format!("0x{}", keys[0].eth))
}

#[test]
fn test_get_public_key_from_sec_key() {
    let keys: Vec<TestKey> = get_keys();
    let seckey = get_seckey_from_string(&keys[1].pk).unwrap();
    let pub_key = get_pub_key(&seckey);
    assert_eq!(pub_key, keys[1].pubk);
}

#[test]
fn test_get_private_key() {
    let key = get_pri_key();
    let key_string = &key.display_secret().to_string();
    assert_eq!(64, key_string.len())
}

#[test]
fn test_get_rev_from_eth() {
    let keys: Vec<TestKey> = get_keys();
    let rev_addr = get_rev_addr_from_eth(&keys[2].eth).unwrap();
    assert_eq!(rev_addr, keys[2].rev)
}

#[test]
fn test_get_rev_addr_pub_key() {
    let keys: Vec<TestKey> = get_keys();
    let rev_addr = rev_address_from_public_key(&keys[1].pubk).unwrap();
    assert_eq!(rev_addr, keys[1].rev)
}

#[test]
fn test_get_rev_from_pri_key() {
    let keys: Vec<TestKey> = get_keys();
    let seckey = get_seckey_from_string(&keys[1].pk).unwrap();
    let rev_addr = get_rev_addr_from_private_key(&seckey).unwrap();
    assert_eq!(rev_addr, keys[1].rev)
}

#[test]
fn test_generate_rev_addr() {
    let rev_addr = get_new_rev_address().unwrap();
    assert_eq!(64, rev_addr.pri_key.len());
    assert_eq!(130, rev_addr.pub_key.len());
}

#[test]
fn test_verify_rev_adr() {
    let keys: Vec<TestKey> = get_keys();
    let valid = verify_rev_addr(&keys[2].rev).unwrap();
    assert!(valid)
}
