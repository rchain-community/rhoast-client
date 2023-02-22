use crate::error::Error;
use crate::http::get_method;
use crate::models::model::{BlockInfo, BlockOptions, BlockPostion, LightBlockInfo};

pub async fn block_call(
    host: &String,
    options: BlockOptions,
) -> Result<Vec<LightBlockInfo>, Error> {
    let url = format!("{}/api/blocks/{}", host, options.position);
    let req = reqwest::get(url).await;
    get_method::<Vec<LightBlockInfo>>(req, &String::from("Error getting block")).await
}

pub async fn latest_block_call(host: &String) -> Result<Vec<LightBlockInfo>, Error> {
    let url = format!("{}/api/blocks", host);
    let req = reqwest::get(url).await;
    get_method::<Vec<LightBlockInfo>>(req, &String::from("Error getting block")).await
}

pub async fn limit_block_call(
    host: &String,
    options: BlockPostion,
) -> Result<Vec<LightBlockInfo>, Error> {
    let url = format!("{}/api/blocks/{}/{}", host, options.start, options.end);
    let req = reqwest::get(url).await;
    get_method::<Vec<LightBlockInfo>>(req, &String::from("Error getting block")).await
}

pub async fn hash_block_call(host: &String, hash: &String) -> Result<BlockInfo, Error> {
    let url = format!("{}/api/block/{}", host, hash);
    let req = reqwest::get(url).await;
    get_method::<BlockInfo>(req, &String::from("Error getting block")).await
}

pub async fn last_finalized_block(host: &String) -> Result<BlockInfo, Error> {
    let url = format!("{}/api/last-finalized-block", host);
    let req = reqwest::get(url).await;
    get_method::<BlockInfo>(req, &String::from("Error getting block")).await
}

pub async fn is_finalized_block(host: &String, hash: &String) -> Result<bool, Error> {
    let url = format!("{}/api/is-finalized/{}", host, hash);
    let req = reqwest::get(url).await;
    get_method::<bool>(req, &String::from("Error getting block")).await
}

pub async fn valid_after_block_number(url: &String) -> Result<i32, Error> {
    let block = BlockOptions { position: 1 };
    let res = block_call(url, block).await?;
    Ok(res[0].block_number)
}
