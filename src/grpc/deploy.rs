use crate::error::ErrCode;
use crate::proto::casper_msg::*;
use crate::proto::deploy::*;
use crate::proto::deployv1::*;
use crate::utils::base58::string_to_static_str;
use tonic::Request;

pub async fn do_deploy_util(
    host: String,
    payload: DeployDataProto,
) -> Result<tonic::Response<DeployResponse>, ErrCode> {
    match deploy_service_client::DeployServiceClient::connect(host).await {
        Ok(mut client) => {
            let request = Request::new(payload);
            match client.do_deploy(request).await {
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

pub async fn get_block_util(
    host: String,
    payload: BlockQuery,
) -> Result<tonic::Response<BlockResponse>, ErrCode> {
    match deploy_service_client::DeployServiceClient::connect(host).await {
        Ok(mut client) => {
            let request = Request::new(payload);
            match client.get_block(request).await {
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

pub async fn machine_verification_dag_util(
    host: String,
    payload: MachineVerifyQuery,
) -> Result<tonic::Response<MachineVerifyResponse>, ErrCode> {
    match deploy_service_client::DeployServiceClient::connect(host).await {
        Ok(mut client) => {
            let request = Request::new(payload);
            match client.machine_verifiable_dag(request).await {
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

pub async fn listen_for_date_at_name_util(
    host: String,
    payload: DataAtNameQuery,
) -> Result<tonic::Response<ListeningNameDataResponse>, ErrCode> {
    match deploy_service_client::DeployServiceClient::connect(host).await {
        Ok(mut client) => {
            let request = Request::new(payload);
            match client.listen_for_data_at_name(request).await {
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

pub async fn listen_for_continuation_at_name_util(
    host: String,
    payload: ContinuationAtNameQuery,
) -> Result<tonic::Response<ContinuationAtNameResponse>, ErrCode> {
    match deploy_service_client::DeployServiceClient::connect(host).await {
        Ok(mut client) => {
            let request = Request::new(payload);
            match client.listen_for_continuation_at_name(request).await {
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

pub async fn find_deploy_util(
    host: String,
    payload: FindDeployQuery,
) -> Result<tonic::Response<FindDeployResponse>, ErrCode> {
    match deploy_service_client::DeployServiceClient::connect(host).await {
        Ok(mut client) => {
            let request = Request::new(payload);
            match client.find_deploy(request).await {
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

pub async fn prview_private_names_util(
    host: String,
    payload: PrivateNamePreviewQuery,
) -> Result<tonic::Response<PrivateNamePreviewResponse>, ErrCode> {
    match deploy_service_client::DeployServiceClient::connect(host).await {
        Ok(mut client) => {
            let request = Request::new(payload);
            match client.preview_private_names(request).await {
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

pub async fn last_finalized_block_util(
    host: String,
    payload: LastFinalizedBlockQuery,
) -> Result<tonic::Response<LastFinalizedBlockResponse>, ErrCode> {
    match deploy_service_client::DeployServiceClient::connect(host).await {
        Ok(mut client) => {
            let request = Request::new(payload);
            match client.last_finalized_block(request).await {
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

pub async fn is_finalized_util(
    host: String,
    payload: IsFinalizedQuery,
) -> Result<tonic::Response<IsFinalizedResponse>, ErrCode> {
    match deploy_service_client::DeployServiceClient::connect(host).await {
        Ok(mut client) => {
            let request = Request::new(payload);
            match client.is_finalized(request).await {
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

pub async fn bond_status_util(
    host: String,
    payload: BondStatusQuery,
) -> Result<tonic::Response<BondStatusResponse>, ErrCode> {
    match deploy_service_client::DeployServiceClient::connect(host).await {
        Ok(mut client) => {
            let request = Request::new(payload);
            match client.bond_status(request).await {
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

pub async fn exploratory_deploy_util(
    host: String,
    payload: ExploratoryDeployQuery,
) -> Result<tonic::Response<ExploratoryDeployResponse>, ErrCode> {
    match deploy_service_client::DeployServiceClient::connect(host).await {
        Ok(mut client) => {
            let request = Request::new(payload);
            match client.exploratory_deploy(request).await {
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

pub async fn get_event_by_hash_util(
    host: String,
    payload: BlockQuery,
) -> Result<tonic::Response<EventInfoResponse>, ErrCode> {
    match deploy_service_client::DeployServiceClient::connect(host).await {
        Ok(mut client) => {
            let request = Request::new(payload);
            match client.get_event_by_hash(request).await {
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
