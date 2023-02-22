use crate::error::Error;
use crate::http::get_method;
use crate::models::model::StatusResponse;

pub async fn status(host: &String) -> Result<StatusResponse, Error> {
    let url = format!("{}/api/status", host);
    let req = reqwest::get(url).await;
    get_method::<StatusResponse>(req, &String::from("Error getting status")).await
}
