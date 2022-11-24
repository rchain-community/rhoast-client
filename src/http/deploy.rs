use crate::http::{block::valid_after_block_number, status::status};
use crate::models::model::{DeployData, DeployDataPayload, EasyDeploy, ExploreDeployResponse};
use crate::utils::deploy_util::get_deploy_data;
use core::time::Duration;

async fn deploy(
    host: String,
    options: DeployData,
    timeout: Option<Duration>,
) -> Result<ExploreDeployResponse, reqwest::Error> {
    //append endpoint
    let url = format!("{}/api/deploy", host);
    if !options.term.contains("(`rho:rchain:deployId`)") && timeout.is_some() {
        println!("term does not include (`rho:rchain:deployId`), data-at-name may not work'");
    }

    match timeout {
        Some(timeout) => {
            let client = reqwest::ClientBuilder::new().timeout(timeout).build()?;
            let response: ExploreDeployResponse =
                client.post(url).json(&options).send().await?.json().await?;
            Ok(response)
        }

        None => {
            let response: ExploreDeployResponse = reqwest::Client::new()
                .post(url)
                .json(&options)
                .send()
                .await?
                .json()
                .await?;
            Ok(response)
        }
    }
}

async fn easy_deploy(
    host: String,
    options: EasyDeploy,
) -> Result<ExploreDeployResponse, reqwest::Error> {
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
        Some(timeout) => {
            let client = reqwest::ClientBuilder::new().timeout(timeout).build()?;
            let response: ExploreDeployResponse =
                client.post(url).json(&payload).send().await?.json().await?;
            Ok(response)
        }

        None => {
            let response: ExploreDeployResponse = reqwest::Client::new()
                .post(url)
                .json(&payload)
                .send()
                .await?
                .json()
                .await?;
            Ok(response)
        }
    }
}
