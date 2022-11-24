use crate::models::model::{BlockOptions, BlockResponse};

async fn block_call(host: String, options: BlockOptions) -> Result<BlockResponse, reqwest::Error> {
    let url = format!("{}/api/blocks/", host);
    let response: BlockResponse = reqwest::get(url).await?.json().await?;
    Ok(response)
}

async fn valid_after_block_number(url: String) -> Result<i32, reqwest::Error> {
    let block = BlockOptions { position: 1 };
    let res = block_call(url, block).await?;
    Ok(res.blocks[0].block_number)
}
