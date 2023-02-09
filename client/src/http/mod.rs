mod block;
mod data_at_name;
mod deploy;
mod explore;
mod status;

use crate::error::Error;
use crate::models::model::HttpModel;
use serde::{de::DeserializeOwned, Serialize};
use rhoast_utils::base58::string_to_static_str;

pub async fn get_method<T: HttpModel + Serialize>(
    res: std::result::Result<reqwest::Response, reqwest::Error>,
    error_str: &String,
) -> Result<T, Error>
where
    T: DeserializeOwned,
{
    match res {
        Ok(res) => match res.json::<T>().await {
            Ok(value) => Ok(value),
            Err(err) => {
                let err = format!("{}: {:?}", error_str, err);
                Err(Error::HttpUtil(string_to_static_str(err)))
            }
        },
        Err(err) => {
            let err = format!("{}: {:?}", error_str, err);
            Err(Error::HttpUtil(string_to_static_str(err)))
        }
    }
}
