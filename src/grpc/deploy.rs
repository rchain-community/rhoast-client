use crate::error::ErrCode;
use crate::proto::casper_msg::*;
use crate::proto::deploy::*;
use crate::proto::deployv1::*;
use crate::utils::base58::string_to_static_str;
use futures::StreamExt;
use tonic::Request;

pub async fn do_deploy_util(
    host: String,
    payload: DeployDataProto,
) -> Result<DeployResponse, ErrCode> {
    match deploy_service_client::DeployServiceClient::connect(host).await {
        Ok(mut client) => {
            let request = Request::new(payload);
            match client.do_deploy(request).await {
                Ok(respose) => Ok(respose.into_inner()),
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

pub async fn get_block_util(host: String, payload: BlockQuery) -> Result<BlockResponse, ErrCode> {
    match deploy_service_client::DeployServiceClient::connect(host).await {
        Ok(mut client) => {
            let request = Request::new(payload);
            match client.get_block(request).await {
                Ok(respose) => Ok(respose.into_inner()),
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
) -> Result<MachineVerifyResponse, ErrCode> {
    match deploy_service_client::DeployServiceClient::connect(host).await {
        Ok(mut client) => {
            let request = Request::new(payload);
            match client.machine_verifiable_dag(request).await {
                Ok(respose) => Ok(respose.into_inner()),
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
) -> Result<ListeningNameDataResponse, ErrCode> {
    match deploy_service_client::DeployServiceClient::connect(host).await {
        Ok(mut client) => {
            let request = Request::new(payload);
            match client.listen_for_data_at_name(request).await {
                Ok(respose) => Ok(respose.into_inner()),
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
) -> Result<ContinuationAtNameResponse, ErrCode> {
    match deploy_service_client::DeployServiceClient::connect(host).await {
        Ok(mut client) => {
            let request = Request::new(payload);
            match client.listen_for_continuation_at_name(request).await {
                Ok(respose) => Ok(respose.into_inner()),
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
) -> Result<FindDeployResponse, ErrCode> {
    match deploy_service_client::DeployServiceClient::connect(host).await {
        Ok(mut client) => {
            let request = Request::new(payload);
            match client.find_deploy(request).await {
                Ok(respose) => Ok(respose.into_inner()),
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
) -> Result<PrivateNamePreviewResponse, ErrCode> {
    match deploy_service_client::DeployServiceClient::connect(host).await {
        Ok(mut client) => {
            let request = Request::new(payload);
            match client.preview_private_names(request).await {
                Ok(respose) => Ok(respose.into_inner()),
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
) -> Result<LastFinalizedBlockResponse, ErrCode> {
    match deploy_service_client::DeployServiceClient::connect(host).await {
        Ok(mut client) => {
            let request = Request::new(payload);
            match client.last_finalized_block(request).await {
                Ok(respose) => Ok(respose.into_inner()),
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
) -> Result<IsFinalizedResponse, ErrCode> {
    match deploy_service_client::DeployServiceClient::connect(host).await {
        Ok(mut client) => {
            let request = Request::new(payload);
            match client.is_finalized(request).await {
                Ok(respose) => Ok(respose.into_inner()),
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
) -> Result<BondStatusResponse, ErrCode> {
    match deploy_service_client::DeployServiceClient::connect(host).await {
        Ok(mut client) => {
            let request = Request::new(payload);
            match client.bond_status(request).await {
                Ok(respose) => Ok(respose.into_inner()),
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
) -> Result<ExploratoryDeployResponse, ErrCode> {
    match deploy_service_client::DeployServiceClient::connect(host).await {
        Ok(mut client) => {
            let request = Request::new(payload);
            match client.exploratory_deploy(request).await {
                Ok(respose) => Ok(respose.into_inner()),
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
) -> Result<EventInfoResponse, ErrCode> {
    match deploy_service_client::DeployServiceClient::connect(host).await {
        Ok(mut client) => {
            let request = Request::new(payload);
            match client.get_event_by_hash(request).await {
                Ok(respose) => Ok(respose.into_inner()),
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

pub async fn visualize_dag_util_stream(
    host: String,
    payload: VisualizeDagQuery,
    output: &mut Vec<VisualizeBlocksResponse>,
    num: usize,
) -> Result<(), ErrCode> {
    match deploy_service_client::DeployServiceClient::connect(host).await {
        Ok(mut client) => {
            let request = Request::new(payload);
            match client.visualize_dag(request).await {
                Ok(stream) => {
                    let mut stream = stream.into_inner().take(num);
                    while let Some(item) = stream.next().await {
                        output.push(item.unwrap())
                    }
                    Ok(())
                }
                Err(err) => {
                    let err = format!("Error getting stream {}", err);
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

pub async fn show_main_chain_util_stream(
    host: String,
    payload: BlocksQuery,
    output: &mut Vec<BlockInfoResponse>,
    num: usize,
) -> Result<(), ErrCode> {
    match deploy_service_client::DeployServiceClient::connect(host).await {
        Ok(mut client) => {
            let request = Request::new(payload);
            match client.show_main_chain(request).await {
                Ok(stream) => {
                    let mut stream = stream.into_inner().take(num);
                    while let Some(item) = stream.next().await {
                        output.push(item.unwrap())
                    }
                    Ok(())
                }
                Err(err) => {
                    let err = format!("Error getting stream {}", err);
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

pub async fn show_blocks_util_stream(
    host: String,
    payload: BlocksQuery,
    output: &mut Vec<BlockInfoResponse>,
    num: usize,
) -> Result<(), ErrCode> {
    match deploy_service_client::DeployServiceClient::connect(host).await {
        Ok(mut client) => {
            let request = Request::new(payload);
            match client.get_blocks(request).await {
                Ok(stream) => {
                    let mut stream = stream.into_inner().take(num);
                    while let Some(item) = stream.next().await {
                        output.push(item.unwrap())
                    }
                    Ok(())
                }
                Err(err) => {
                    let err = format!("Error getting stream {}", err);
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

pub async fn get_blocks_by_height_util_stream(
    host: String,
    payload: BlocksQueryByHeight,
    output: &mut Vec<BlockInfoResponse>,
    num: usize,
) -> Result<(), ErrCode> {
    match deploy_service_client::DeployServiceClient::connect(host).await {
        Ok(mut client) => {
            let request = Request::new(payload);
            match client.get_blocks_by_heights(request).await {
                Ok(stream) => {
                    let mut stream = stream.into_inner().take(num);
                    while let Some(item) = stream.next().await {
                        output.push(item.unwrap())
                    }
                    Ok(())
                }
                Err(err) => {
                    let err = format!("Error getting stream {}", err);
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
