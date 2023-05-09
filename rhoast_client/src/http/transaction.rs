use super::Http;
use crate::{error::Error, http::get_method_str};

impl Http {
    pub async fn get_transaction(&self, hash: &String) -> Result<String, Error> {
        let url = format!("{}/api/transactions/{}", &self.host, hash);
        let req = reqwest::get(url).await;
        get_method_str(req, &"Error getting transaction details".to_string()).await
    }
}
