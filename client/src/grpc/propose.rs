use crate::error::Error;
use crate::proto::casper::casper_v1::*;
use tonic::Request;
use utils::base58::string_to_static_str;

pub async fn propose_util(host: String, payload: bool) -> Result<ProposeResponse, Error> {
    match propose_service_client::ProposeServiceClient::connect(host).await {
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