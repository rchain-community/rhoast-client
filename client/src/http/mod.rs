pub mod block;
pub mod data_at_name;
pub mod deploy;
pub mod explore;
pub mod status;
pub mod transaction;

use std::fmt::Debug;

use crate::error::Error;
use crate::models::model::HttpModel;
use rhoast_utils::base58::string_to_static_str;
use serde::{de::DeserializeOwned, Serialize};

pub async fn get_method<T: HttpModel + Serialize + Debug>(
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

pub async fn get_method_str(
    res: std::result::Result<reqwest::Response, reqwest::Error>,
    error_str: &String,
) -> Result<String, Error> {
    match res {
        Ok(res) => match res.text().await {
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
