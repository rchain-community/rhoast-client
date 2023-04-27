use super::Http;
use crate::models::model::ExploreDataOptions;
use crate::{error::Error, http::get_method_str};

impl Http {
    pub async fn explore_deploy(&self, options: ExploreDataOptions) -> Result<String, Error> {
        let url = format!("{}/api/explore-deploy", &self.host);
        let req = reqwest::Client::new()
            .post(url)
            .body(options.term)
            .send()
            .await;
        get_method_str(req, &"Error getting explore deploy details".to_string()).await
    }
}
