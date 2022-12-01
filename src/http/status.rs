use crate::models::model::StatusRespoonse;

pub async fn status(host: &String) -> Result<StatusRespoonse, reqwest::Error> {
    let url = format!("{}/api/status", host);
    let response: StatusRespoonse = reqwest::get(url).await?.json().await?;
    Ok(response)
}
