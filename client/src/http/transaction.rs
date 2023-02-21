use crate::{error::Error, http::get_method_str};

pub async fn get_transaction(host: &String, hash: &String) -> Result<String, Error> {
    let url = format!("{}/api/transactions/{}", host, hash);
    let req = reqwest::get(url).await;
    get_method_str(req, &"Error getting transaction details".to_string()).await
}
