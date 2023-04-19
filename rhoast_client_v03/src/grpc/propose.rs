use super::string_to_static_str;
use super::Grpc;
use crate::error::Error;
use crate::proto::casper::v1::{propose_service_client, ProposeResponse, ProposeResultResponse};
use crate::proto::casper::{ProposeQuery, ProposeResultQuery};
use tonic::Request;

impl Grpc {
    pub fn new(host: &str) -> Self {
        Grpc {
            host: host.to_string(),
        }
    }

    pub async fn propose(&self, payload: bool) -> Result<ProposeResponse, Error> {
        match propose_service_client::ProposeServiceClient::connect(self.host.to_string()).await {
            Ok(mut client) => {
                let req = Request::new(ProposeQuery { is_async: payload });
                match client.propose(req).await {
                    Ok(res) => Ok(res.into_inner()),
                    Err(err) => {
                        let err = format!("Error getting response {}", err);
                        Err(Error::GrpcUtil(string_to_static_str(err)))
                    }
                }
            }
            Err(err) => {
                let err = format!("Error creating GRPC connection {}", err);
                Err(Error::GrpcUtil(string_to_static_str(err)))
            }
        }
    }
    pub async fn propose_result(
        &self,
        payload: ProposeResultQuery,
    ) -> Result<ProposeResultResponse, Error> {
        match propose_service_client::ProposeServiceClient::connect(self.host.to_string()).await {
            Ok(mut client) => {
                let req = Request::new(payload);
                match client.propose_result(req).await {
                    Ok(res) => Ok(res.into_inner()),
                    Err(err) => {
                        let err = format!("Error getting response {}", err);
                        Err(Error::GrpcUtil(string_to_static_str(err)))
                    }
                }
            }
            Err(err) => {
                let err = format!("Error creating GRPC connection {}", err);
                Err(Error::GrpcUtil(string_to_static_str(err)))
            }
        }
    }
}
