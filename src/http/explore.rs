use crate::models::model::{ExploreDataOptions, ExploreDeployResponse};

pub async fn explore_deploy(
    host: String,
    options: ExploreDataOptions,
) -> Result<ExploreDeployResponse, reqwest::Error> {
    let url = format!("{}/api/explore-deploy", host);
    let response: ExploreDeployResponse = reqwest::Client::new()
        .post(url)
        .json(&options)
        .send()
        .await?
        .json()
        .await?;
    Ok(response)
}
