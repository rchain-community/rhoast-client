use crate::error::ErrCode;
use crate::models::model::DeployData;
use core::time::Duration;

async fn deploy(
    host: String,
    options: DeployData,
    timeout: Option<Duration>,
) -> Result<String, reqwest::Error> {
    //append endpoint
    let url = format!("{}/api/deploy", host);
    if !options.term.contains("(`rho:rchain:deployId`)") {
        println!("term does not include (`rho:rchain:deployId`), data-at-name may not work'");
    }

    match timeout {
        Some(timeout) => {
            let client = reqwest::ClientBuilder::new().timeout(timeout).build()?;
            let response: String = client.post(url).json(&options).send().await?.json().await?;
            Ok(response)
        }

        None => {
            let response: String = reqwest::Client::new()
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
