use crate::error::Error;
use crate::http::string_to_static_str;
use crate::proto::v0_12::casper::v1::{
    propose_service_client, PrintUnmatchedSendsQuery, ProposeResponse,
};
use tonic::Request;

use super::GrpcV0_12;

impl GrpcV0_12 {
    pub fn new(host: &str) -> Self {
        GrpcV0_12 {
            host: host.to_string(),
        }
    }
    pub async fn propose(&self, payload: bool) -> Result<ProposeResponse, Error> {
        match propose_service_client::ProposeServiceClient::connect(self.host.to_string()).await {
            Ok(mut client) => {
                let request = Request::new(PrintUnmatchedSendsQuery {
                    print_unmatched_sends: payload,
                });
                match client.propose(request).await {
                    Ok(respose) => Ok(respose.into_inner()),
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
