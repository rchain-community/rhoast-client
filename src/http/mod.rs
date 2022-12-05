mod block;
mod data_at_name;
mod deploy;
mod explore;
mod status;

use crate::error::ErrCode;
use crate::models::model::HttpModel;
use crate::utils::base58::string_to_static_str;
use serde::{de::DeserializeOwned, Serialize};

pub async fn get_method<'a, T: HttpModel + Serialize>(
    res: std::result::Result<reqwest::Response, reqwest::Error>,
    error_str: &String,
) -> Result<T, ErrCode>
where
    T: DeserializeOwned,
{
    match res {
        Ok(res) => match res.json::<T>().await {
            Ok(value) => Ok(value),
            Err(err) => {
                let err = format!("{}: {:?}", error_str, err);
                Err(ErrCode::HttpUtil(string_to_static_str(err)))
            }
        },
        Err(err) => {
            let err = format!("{}: {:?}", error_str, err);
            Err(ErrCode::HttpUtil(string_to_static_str(err)))
        }
    }
}
