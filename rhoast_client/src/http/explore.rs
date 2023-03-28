use crate::models::model::ExploreDataOptions;
use crate::{error::Error, http::get_method_str};

pub async fn explore_deploy(host: &String, options: ExploreDataOptions) -> Result<String, Error> {
    let url = format!("{}/api/explore-deploy", host);
    let req = reqwest::Client::new()
        .post(url)
        .body(options.term)
        .send()
        .await;
    get_method_str(req, &"Error getting explore deploy details".to_string()).await
}
