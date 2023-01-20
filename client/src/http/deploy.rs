use crate::error::Error;
use crate::grpc::deploy_util::get_deploy_data;
use crate::http::get_method;
use crate::http::{block::valid_after_block_number, status::status};
use crate::models::model::{
    DeployData, DeployDataPayload, DeployResponse, EasyDeploy, PrepareDeployOptions,
    PrepareDeployResponse,
};
use core::time::Duration;

pub async fn deploy(
    host: String,
    options: DeployData,
    timeout: Option<Duration>,
) -> Result<DeployResponse, Error> {
    //append endpoint
    let url = format!("{}/api/deploy", host);
    if !options.term.contains("(`rho:rchain:deployId`)") && timeout.is_some() {
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

pub async fn easy_deploy(host: String, options: EasyDeploy) -> Result<DeployResponse, Error> {
    let url = format!("{}/api/deploy", host);
    let mut phlo_price_ok = 0;
    if options.phlo_price_auto.is_some() {
        if options.phlo_price_auto.unwrap() == "auto" {
            phlo_price_ok = status(&host).await?.min_phlo_price.parse::<u64>().unwrap();
        }
    } else {
        phlo_price_ok = options.phlo_price.unwrap()
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
        shard_id: options.shard_id.unwrap(),
        phlo_price: phlo_price_ok,
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
    host: String,
    options: PrepareDeployOptions,
) -> Result<PrepareDeployResponse, Error> {
    let url = format!("{}/api/prepare-deploy", host);
    let req = reqwest::Client::new().post(url).json(&options).send().await;
    get_method::<PrepareDeployResponse>(req, &String::from("Error on prepare deploy")).await
}
