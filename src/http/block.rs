use crate::error::Error;
use crate::http::get_method;
use crate::models::model::{BlockOptions, BlockResponse};

pub async fn block_call(host: &String, options: BlockOptions) -> Result<BlockResponse, Error> {
    let url = format!("{}/api/blocks/{}", host, options.position);
    let req = reqwest::get(url).await;
    get_method::<BlockResponse>(req, &String::from("Error getting block")).await
}

pub async fn valid_after_block_number(url: &String) -> Result<i32, Error> {
    let block = BlockOptions { position: 1 };
    let res = block_call(url, block).await?;
    Ok(res.blocks[0].block_number)
}
