use crate::error::Error;
use crate::http::get_method;
use crate::models::model::StatusRespoonse;

pub async fn status(host: &String) -> Result<StatusRespoonse, Error> {
    let url = format!("{}/api/status", host);
    let req = reqwest::get(url).await;
    get_method::<StatusRespoonse>(req, &String::from("Error getting status")).await
}
