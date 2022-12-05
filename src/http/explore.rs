use crate::models::model::{ExploreDataOptions, ExploreDeployResponse};
use crate::{error::ErrCode, http::get_method};

pub async fn explore_deploy(
    host: String,
    options: ExploreDataOptions,
) -> Result<ExploreDeployResponse, ErrCode> {
    let url = format!("{}/api/explore-deploy", host);
    let req = reqwest::Client::new().post(url).json(&options).send().await;
    get_method::<ExploreDeployResponse>(req, &String::from("Error on explor data")).await
}
