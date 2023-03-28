use rhoast_wallet::rho::pos_bond_rho;
use rhoast_client::{http::explore::explore_deploy, models::model::ExploreDataOptions};
use rhoast_utils::rev_address_from_public_key::rev_address_from_public_key;

pub mod rho;
#[tokio::main]
async fn main() {
    let term = rho::check_balance("1111Zfr2UB1YXuD6zccsPhqthiik4jfQV8W8aoncbvVGtfU8QJQei".to_string());
    let payload = ExploreDataOptions { term };
    let res = explore_deploy(&"https://observer.services.mainnet.rchain.coop".to_string(), payload).await;
    println!("{:?}", res.unwrap())
}
