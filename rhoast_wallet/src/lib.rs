pub mod error;
pub mod rho;

use error::Error;
use rhoast_client::http::Http;
use rhoast_client::models::model::ExploreDataOptions;
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
        http_url: &String,
        grpc_url: &String,
        shard_id: &String,
        network: &String,
        http_admin_url: &String,
    ) -> Self {
        Node {
            http_url: http_url.to_string(),
            grpc_url: grpc_url.to_string(),
            shard_id: shard_id.to_string(),
            network: network.to_string(),
            http_admin_url: http_admin_url.to_string(),
        }
    }

    pub async fn check_balance(&self, rev_addr: &String) -> Result<u64, Error> {
        match verify_rev_addr(&rev_addr) {
            Ok(addr) => {
                if !addr {
                    return Err(Error::CheckBlance("invalid rev addr"));
                }
                let http = Http::new(&self.http_url);
                let term = rho::check_balance(rev_addr.to_string());
                let payload = ExploreDataOptions { term };
                let res = http.explore_deploy(payload).await;

                match res {
                    Ok(response) => {
                        let value: std::result::Result<serde_json::Value, serde_json::Error> =
                            serde_json::from_str(&response);
                        match value {
                            Ok(json) => {
                                let expr = &json.get("expr");
                                match expr {
                                    Some(expr_val) => {
                                        if expr_val.as_array().unwrap().len() < 1 {
                                            return Err(Error::CheckBlance(
                                                "Error getting balance",
                                            ));
                                        }
                                        let expr_index = &expr_val[0];
                                        let expr_int = &expr_index["ExprInt"]["data"];

                                        match expr_int.as_u64() {
                                            Some(balance) => Ok(balance),
                                            None => match expr_index.get("ExprString") {
                                                Some(error_msg) => {
                                                    let expr_error =
                                                        &error_msg["data"].as_str().unwrap();
                                                    Err(Error::CheckBlance(string_to_static_str(
                                                        expr_error.to_string(),
                                                    )))
                                                }
                                                None => {
                                                    Err(Error::CheckBlance("Error getting balance"))
                                                }
                                            },
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
