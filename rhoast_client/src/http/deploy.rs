use super::{Http, get_method_str};
use crate::error::Error;
use crate::http::get_method;
use crate::models::model::{
    DeployDataPayload, DeployDataRequest, EasyDeploy, LightBlockInfo,
    PrepareDeployOptions, PrepareDeployResponse,
};
use crate::util::deploy_util::get_deploy_data;
use core::time::Duration;

impl Http {
    ///uccess: request was correctly formatted and signed. Note that this does not indicate successful execution.\nUse the signature as deployId in
    ///  /deploy/ or `deploy_with_deployid` method to find the hash of the block in which this deploy was attempted and then GET /block/{hash} or `hash_block_call` to find `DeployInfo` such as `systemDeployError`, `errored` and `cost`.\n
    pub async fn deploy(
        &self,
        options: DeployDataRequest,
        timeout: Option<Duration>,
    ) -> Result<String, Error> {
        //append endpoint
        let url = format!("{}/api/deploy", &self.host);
        if !options.data.term.contains("(`rho:rchain:deployId`)") && timeout.is_some() {
            println!("term does not include (`rho:rchain:deployId`), data-at-name may not work'");
        }

        match timeout {
            Some(timeout) => match reqwest::ClientBuilder::new().timeout(timeout).build() {
                Ok(client) => {
                    let req = client.post(url).json(&options).send().await;
                    get_method_str(req, &String::from("Error on deploy")).await
                }
                Err(_) => Err(Error::HttpUtil("Error building HTTP client")),
            },

            None => {
                let req = reqwest::Client::new().post(url).json(&options).send().await;
                get_method_str(req, &String::from("Error on deploy")).await
            }
        }
    }

    pub async fn deploy_with_deployid(
        &self,
        deploy_id: &String,
    ) -> Result<Vec<LightBlockInfo>, Error> {
        let url = format!("{}/api/deploy/{}", &self.host, deploy_id);
        let req = reqwest::get(url).await;
        get_method::<Vec<LightBlockInfo>>(req, &String::from("Error getting deployid")).await
    }

    pub async fn easy_deploy(&self, options: EasyDeploy) -> Result<String, Error> {
        let url = format!("{}/api/deploy", &self.host);
        let mut phlo_price_ok = 0;
        if options.phlo_price_auto.is_some() {
            if options.phlo_price_auto.unwrap() == "auto" {
                phlo_price_ok = self.status().await?.min_phlo_price;
            }
        } else {
            phlo_price_ok = options.phlo_price.unwrap_or(1) as i32
        }

        if !options.term.contains("(`rho:rchain:deployId`)") && options.timeout.is_some() {
            println!("term does not include (`rho:rchain:deployId`), data-at-name may not work'");
        }

        let vab = self.valid_after_block_number().await?;
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

                    get_method_str(req, &String::from("Error on easy deploy")).await
                }
                Err(_) => Err(Error::HttpUtil("Error building HTTP client")),
            },

            None => {
                let req = reqwest::Client::new().post(url).json(&payload).send().await;

                get_method_str(req, &String::from("Error on easy deploy")).await
            }
        }
    }

    pub async fn prepare_deploy(
        &self,
        options: PrepareDeployOptions,
    ) -> Result<PrepareDeployResponse, Error> {
        let url = format!("{}/api/prepare-deploy", &self.host);
        let req = reqwest::Client::new().post(url).json(&options).send().await;
        get_method::<PrepareDeployResponse>(req, &String::from("Error on prepare deploy")).await
    }
}
