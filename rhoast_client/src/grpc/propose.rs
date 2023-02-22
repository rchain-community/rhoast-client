use crate::error::Error;
use crate::http::string_to_static_str;
use crate::proto::casper::v1::{propose_service_client, PrintUnmatchedSendsQuery, ProposeResponse};
use tonic::Request;

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
