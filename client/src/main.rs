use client::grpc::deploy::*;
use client::http::{block::block_call, status::status};
use client::models::model::*;
use client::proto::casper::IsFinalizedQuery;

#[tokio::main]
async fn main() {
    // let opt = BlockOptions { position: 1 };
    // let data = block_call(&"http://rnode-06.test.r-pub.com:40403".to_string(), opt).await;
    // println!("{:?}", data);

    // let status = status(&"http://rnode-06.test.r-pub.com:40403".to_string()).await;
    // println!("{:?}", status);
    let bond_status = IsFinalizedQuery {
        hash: "UmVmaW5lZCBzdGF0aWM=".to_string(),
    };

    let block = is_finalized_util(
        "http://rnode-06.test.r-pub.com:40401".to_string(),
        bond_status,
    )
    .await
    .unwrap();
    println!("{:?}", block)
}
