use crate::error::Error;
use crate::http::get_method;
use crate::models::model::{BlockOptions, LightBlockInfo};

pub async fn block_call(
    host: &String,
    options: BlockOptions,
) -> Result<Vec<LightBlockInfo>, Error> {
    let url = format!("{}/api/blocks/{}", host, options.position);
    let req = reqwest::get(url).await;
    get_method::<Vec<LightBlockInfo>>(req, &String::from("Error getting block")).await
}

pub async fn valid_after_block_number(url: &String) -> Result<i32, Error> {
    let block = BlockOptions { position: 1 };
    let res = block_call(url, block).await?;
    Ok(res[0].block_number)
}
