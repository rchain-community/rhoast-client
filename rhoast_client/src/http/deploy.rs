use crate::error::Error;
use crate::http::get_method;
use crate::http::{block::valid_after_block_number, status::status};
use crate::models::model::{
    DeployDataPayload, DeployDataRequest, DeployResponse, EasyDeploy, LightBlockInfo,
    PrepareDeployOptions, PrepareDeployResponse,
};
use crate::util::deploy_util::get_deploy_data;
use core::time::Duration;

pub async fn deploy(
    host: &String,
    options: DeployDataRequest,
    timeout: Option<Duration>,
) -> Result<DeployResponse, Error> {
    //append endpoint
    let url = format!("{}/api/deploy", host);
    if !options.data.term.contains("(`rho:rchain:deployId`)") && timeout.is_some() {
        println!("term does not include (`rho:rchain:deployId`), data-at-name may not work'");
    }

    match timeout {
        Some(timeout) => match reqwest::ClientBuilder::new().timeout(timeout).build() {
            Ok(client) => {
                let req = client.post(url).json(&options).send().await;
                get_method::<DeployResponse>(req, &String::from("Error on deploy")).await
            }
            Err(_) => Err(Error::HttpUtil("Error building HTTP client")),
        },

        None => {
            let req = reqwest::Client::new().post(url).json(&options).send().await;
            get_method::<DeployResponse>(req, &String::from("Error on deploy")).await
        }
    }
}

pub async fn deploy_with_deployid(
    host: &String,
    deploy_id: &String,
) -> Result<Vec<LightBlockInfo>, Error> {
    let url = format!("{}/api/deploy/{}", host, deploy_id);
    let req = reqwest::get(url).await;
    get_method::<Vec<LightBlockInfo>>(req, &String::from("Error getting deployid")).await
}

pub async fn easy_deploy(host: &String, options: EasyDeploy) -> Result<DeployResponse, Error> {
    let url = format!("{}/api/deploy", host);
    let mut phlo_price_ok = 0;
    if options.phlo_price_auto.is_some() {
        if options.phlo_price_auto.unwrap() == "auto" {
            phlo_price_ok = status(&host).await?.min_phlo_price;
        }
    } else {
        phlo_price_ok = options.phlo_price.unwrap_or(1) as i32
    }

    if !options.term.contains("(`rho:rchain:deployId`)") && options.timeout.is_some() {
        println!("term does not include (`rho:rchain:deployId`), data-at-name may not work'");
    }

    let vab = valid_after_block_number(&host).await?;
    let time = chrono::offset::Utc::now().timestamp();
    let deploy_data = DeployDataPayload {
        sig_algorithm: "secp256k1".to_string(),
        timestamp: time,
        private_key: options.private_key,
        term: options.term,
        shard_id: options.shard_id.unwrap_or("".to_string()),
        phlo_price: phlo_price_ok as u64,
        phlo_limit: options.phlo_limit,
        valid_after_block_number: vab,
    };
    let payload = get_deploy_data(&deploy_data).unwrap();

    match options.timeout {
        Some(timeout) => match reqwest::ClientBuilder::new().timeout(timeout).build() {
            Ok(client) => {
                let req = client.post(url).json(&payload).send().await;

                get_method::<DeployResponse>(req, &String::from("Error on easy deploy")).await
            }
            Err(_) => Err(Error::HttpUtil("Error building HTTP client")),
        },

        None => {
            let req = reqwest::Client::new().post(url).json(&payload).send().await;

            get_method::<DeployResponse>(req, &String::from("Error on easy deploy")).await
        }
    }
}

pub async fn prepare_deploy(
    host: &String,
    options: PrepareDeployOptions,
) -> Result<PrepareDeployResponse, Error> {
    let url = format!("{}/api/prepare-deploy", host);
    let req = reqwest::Client::new().post(url).json(&options).send().await;
    get_method::<PrepareDeployResponse>(req, &String::from("Error on prepare deploy")).await
}
