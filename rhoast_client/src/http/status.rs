use super::Http;
use crate::error::Error;
use crate::http::get_method;
use crate::models::model::StatusResponse;

impl Http {
    pub fn new(host: &str) -> Self {
        Http {
            host: host.to_string(),
        }
    }
    pub async fn status(&self) -> Result<StatusResponse, Error> {
        let url = format!("{}/api/status", &self.host);
        let req = reqwest::get(url).await;
        get_method::<StatusResponse>(req, &String::from("Error getting status")).await
    }
}
