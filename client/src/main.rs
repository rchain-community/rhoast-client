use client::http::{block::block_call, status::status};
use client::models::model::*;

#[tokio::main]
async fn main() {
    // let opt = BlockOptions { position: 1 };
    // let data = block_call(&"http://rnode-06.test.r-pub.com:40403".to_string(), opt).await;
    // println!("{:?}", data);

    let status = status(&"http://rnode-06.test.r-pub.com:40403".to_string()).await;
    println!("{:?}", status);
}
