use crate::error::Error;
use crate::http::get_method;
use crate::models::model::{
    DataAtNameByBlockHashUnforgDeployOptions, DataAtNameByBlockHashUnforgDeployerOptions,
    DataAtNameByBlockHashUnforgPrivateOptions, DataAtNameUnforgDeployOptions,
    DataAtNameUnforgDeployerOptions, DataAtNameUnforgPrivateOptions,
};
use serde::Serialize;
pub trait DataAtName {}

impl DataAtName for DataAtNameUnforgDeployerOptions {}
impl DataAtName for DataAtNameUnforgDeployOptions {}
impl DataAtName for DataAtNameUnforgPrivateOptions {}

pub async fn data_at_name<C: DataAtName + Serialize>(
    host: String,
    options: C,
) -> Result<String, Error> {
    let url = format!("{}/api/data-at-name", host);
    let req = reqwest::Client::new().post(url).json(&options).send().await;
    get_method::<String>(req, &String::from("Error on data at name")).await
}

pub trait DataAtNameBlockHash {}

impl DataAtNameBlockHash for DataAtNameByBlockHashUnforgPrivateOptions {}
impl DataAtNameBlockHash for DataAtNameByBlockHashUnforgDeployOptions {}
impl DataAtNameBlockHash for DataAtNameByBlockHashUnforgDeployerOptions {}
pub async fn data_at_name_by_block_hash<C: DataAtNameBlockHash + Serialize>(
    host: String,
    options: C,
) -> Result<String, Error> {
    let url = format!("{}/api/data-at-name-by-block-hash", host);
    let req = reqwest::Client::new().post(url).json(&options).send().await;
    get_method::<String>(req, &String::from("Error on data at name by block hash")).await
}
