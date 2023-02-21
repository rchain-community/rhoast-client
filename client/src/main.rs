use client::grpc::deploy::*;
use client::grpc::propose::propose_util;
use client::http::{block::is_finalized_block, transaction::get_transaction};
use client::models::model::*;
use client::proto::casper::IsFinalizedQuery;

#[tokio::main]
async fn main() {
    let a = DeployData {
        phlo_price: 1,
        phlo_limit: 250000,
        timestamp: 234567654,
        shard_id: "test".to_string(),
        valid_after_block_number: 420299,
        term: "new world in {\n  world!(\"Hello!\")\n}\n".to_string(),
    };
    let k = ExploreDataOptions {
        term: "new world in {\n  world!(\"Hello!\")\n}\n".to_string(),
    };

    let block = get_transaction(
        &"http://167.235.8.107:40403".to_string(),
        &"857c96855f6f5b6eadda6278225c49ee00b2cef96c83cd81bc93501c16f3658c".to_string(),
    )
    .await
    .unwrap();
    println!("{:?}", block);

    // let block = propose_util(
    //     "http://167.235.8.107:40402".to_string(),
    //     true,
    // )
    // .await
    // .unwrap();
    // println!("{:?}", block)
}
