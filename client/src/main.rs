use client::grpc::deploy::*;
use client::http::{block::block_call, status::status};
use client::models::model::*;
use client::proto::casper::BondStatusQuery;

#[tokio::main]
async fn main() {
    // let opt = BlockOptions { position: 1 };
    // let data = block_call(&"http://rnode-06.test.r-pub.com:40403".to_string(), opt).await;
    // println!("{:?}", data);

    // let status = status(&"http://rnode-06.test.r-pub.com:40403".to_string()).await;
    // println!("{:?}", status);
    let bond_status = BondStatusQuery {
        public_key: "UmVmaW5lZCBzdGF0aWM=".as_bytes().to_vec(),
    };

    let block = bond_status_util(
        "http://rnode-06.test.r-pub.com:40401".to_string(),
        bond_status,
    )
    .await
    .unwrap();
    println!("{:?}", block)
}
