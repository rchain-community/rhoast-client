pub mod error;
pub mod rho;

use error::Error;
use rhoast_client::{http::explore::explore_deploy, models::model::ExploreDataOptions};
use rhoast_utils::rev_address_from_public_key::verify_rev_addr;

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

fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
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

    pub async fn check_balance(&self, rev_addr: String) -> Result<Option<u64>, Error> {
        match verify_rev_addr(&rev_addr) {
            Ok(addr) => {
                if !addr {
                    return Err(Error::CheckBlance("invalid rev addr"));
                } 
                let term = rho::check_balance(rev_addr);
                let payload = ExploreDataOptions { term };
                let res = explore_deploy(&self.http_url, payload).await;

                match res {
                    Ok(response) => {
                        let value: std::result::Result<serde_json::Value, serde_json::Error> =
                            serde_json::from_str(&response);
                        match value {
                            Ok(json) => {
                                let expr = &json.get("expr");
                                match expr {
                                    Some(expr_val) => {
                                        let expr_index = &expr_val[0];
                                        let expr_int = expr_index.get("ExprInt");
                                        match expr_int {
                                            Some(expr_int) => {
                                                let data = expr_int.get("data");
                                                match data {
                                                    Some(data) => Ok(data.as_u64()),
                                                    None => {
                                                        let err = format!(
                                                            "Error getting balance {:?}",
                                                            expr_val["ExprString"]["data"]
                                                        );
                                                        Err(Error::CheckBlance(
                                                            string_to_static_str(err),
                                                        ))
                                                    }
                                                }
                                            }
                                            None => {
                                                let err = format!(
                                                    "Error getting balance {:?}",
                                                    expr_val["ExprString"]["data"]
                                                );
                                                Err(Error::CheckBlance(string_to_static_str(err)))
                                            }
                                        }
                                    }
                                    None => Err(Error::CheckBlance("No expresssion found")),
                                }
                            }
                            Err(error) => {
                                let err = format!("Error parsing json {}", error);
                                Err(Error::CheckBlance(
                                    rhoast_client::http::string_to_static_str(err),
                                ))
                            }
                        }
                    }
                    Err(err) => {
                        let err = format!("Error getting balance {}", err);
                        Err(Error::CheckBlance(
                            rhoast_client::http::string_to_static_str(err),
                        ))
                    }
                }
            }
            Err(_) => Err(Error::CheckBlance("Error verifying rev address")),
        }
    }
}
