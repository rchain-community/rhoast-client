pub mod error;
pub mod rho;

use error::Error;
use rhoast_client::{http::explore::explore_deploy, models::model::ExploreDataOptions};

#[derive(Debug)]
pub struct Node {
    pub http_url: String,
    pub grpc_url: String,
    pub shard_id: String,
    pub network: String,
    pub http_admin_url: String,
}

#[derive(Debug)]
pub struct Account {
    pub from_acc_name: String,
    pub from_acc_addr: String,
    pub to_acc_name: String,
    pub to_acc_addr: String,
    pub pri_key: String,
    pub amount: u64,
}

impl Node {
    pub fn new(
        http_url: String,
        grpc_url: String,
        shard_id: String,
        network: String,
        http_admin_url: String,
    ) -> Self {
        Node {
            http_url,
            grpc_url,
            shard_id,
            network,
            http_admin_url,
        }
    }

    pub async fn check_balance(&self, rev_addr: String) -> Result<(), Error> {
        let term = rho::check_balance(rev_addr);
        let payload = ExploreDataOptions { term };
        let res = explore_deploy(&self.http_url, payload).await;

        match res {
            Ok(response) => Ok(()),
            Err(err) => {
                let err = format!("Error getting balance {}", err);
                Err(Error::CheckBlance(
                    rhoast_client::http::string_to_static_str(err),
                ))
            }
        }
    }
}
