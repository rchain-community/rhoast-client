use crate::error::ErrCode;
use crate::proto::casper::casper_v1::*;
use crate::utils::base58::string_to_static_str;
use tonic::Request;

pub async fn propose_util(
    host: String,
    payload: bool,
) -> Result<tonic::Response<ProposeResponse>, ErrCode> {
    match propose_service_client::ProposeServiceClient::connect(host).await {
        Ok(mut client) => {
            let request = Request::new(PrintUnmatchedSendsQuery {
                print_unmatched_sends: payload,
            });
            match client.propose(request).await {
                Ok(respose) => Ok(respose),
                Err(err) => {
                    let err = format!("Error getting response {}", err);
                    Err(ErrCode::GrpcUtil(string_to_static_str(err)))
                }
            }
        }
        Err(err) => {
            let err = format!("Error creating GRPC connection {}", err);
            Err(ErrCode::GrpcUtil(string_to_static_str(err)))
        }
    }
}
