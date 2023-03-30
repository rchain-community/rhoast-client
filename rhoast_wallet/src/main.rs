pub mod rho;
#[tokio::main]
async fn main() {
    let node = rhoast_wallet::Node::new(
        &"https://observer.services.mainnet.rchain.coop".to_string(),
        &"".to_string(),
        &"".to_string(),
        &"".to_string(),
        &"".to_string(),
    );
    let balance = node
        .check_balance(&"1111Zfr2UB1YXuD6zccsPhqthiik4jfQV8W8aoncbvVGtfU8QJQei".to_string())
        .await
        .unwrap();
    println!("{}", balance)
}
