use crate::error::Error;
use crate::proto::casper::v1::*;
use crate::proto::casper::*;
use futures::StreamExt;
use rhoast_utils::base58::string_to_static_str;
use tonic::Request;

pub async fn do_deploy_util(
    host: String,
    payload: DeployDataProto,
) -> Result<DeployResponse, Error> {
    match deploy_service_client::DeployServiceClient::connect(host).await {
        Ok(mut client) => {
            let request = Request::new(payload);
            match client.do_deploy(request).await {
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

pub async fn get_block_util(host: String, payload: BlockQuery) -> Result<BlockResponse, Error> {
    match deploy_service_client::DeployServiceClient::connect(host).await {
        Ok(mut client) => {
            let request = Request::new(payload);
            match client.get_block(request).await {
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

pub async fn machine_verification_dag_util(
    host: String,
    payload: MachineVerifyQuery,
) -> Result<MachineVerifyResponse, Error> {
    match deploy_service_client::DeployServiceClient::connect(host).await {
        Ok(mut client) => {
            let request = Request::new(payload);
            match client.machine_verifiable_dag(request).await {
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

pub async fn listen_for_date_at_name_util(
    host: String,
    payload: DataAtNameQuery,
) -> Result<ListeningNameDataResponse, Error> {
    match deploy_service_client::DeployServiceClient::connect(host).await {
        Ok(mut client) => {
            let request = Request::new(payload);
            match client.listen_for_data_at_name(request).await {
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

pub async fn listen_for_continuation_at_name_util(
    host: String,
    payload: ContinuationAtNameQuery,
) -> Result<ContinuationAtNameResponse, Error> {
    match deploy_service_client::DeployServiceClient::connect(host).await {
        Ok(mut client) => {
            let request = Request::new(payload);
            match client.listen_for_continuation_at_name(request).await {
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

pub async fn find_deploy_util(
    host: String,
    payload: FindDeployQuery,
) -> Result<FindDeployResponse, Error> {
    match deploy_service_client::DeployServiceClient::connect(host).await {
        Ok(mut client) => {
            let request = Request::new(payload);
            match client.find_deploy(request).await {
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

pub async fn prview_private_names_util(
    host: String,
    payload: PrivateNamePreviewQuery,
) -> Result<PrivateNamePreviewResponse, Error> {
    match deploy_service_client::DeployServiceClient::connect(host).await {
        Ok(mut client) => {
            let request = Request::new(payload);
            match client.preview_private_names(request).await {
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

pub async fn last_finalized_block_util(
    host: String,
    payload: LastFinalizedBlockQuery,
) -> Result<LastFinalizedBlockResponse, Error> {
    match deploy_service_client::DeployServiceClient::connect(host).await {
        Ok(mut client) => {
            let request = Request::new(payload);
            match client.last_finalized_block(request).await {
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

pub async fn is_finalized_util(
    host: String,
    payload: IsFinalizedQuery,
) -> Result<IsFinalizedResponse, Error> {
    match deploy_service_client::DeployServiceClient::connect(host).await {
        Ok(mut client) => {
            let request = Request::new(payload);
            match client.is_finalized(request).await {
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

pub async fn bond_status_util(
    host: String,
    payload: BondStatusQuery,
) -> Result<BondStatusResponse, Error> {
    match deploy_service_client::DeployServiceClient::connect(host).await {
        Ok(mut client) => {
            let request = Request::new(payload);
            match client.bond_status(request).await {
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

pub async fn exploratory_deploy_util(
    host: String,
    payload: ExploratoryDeployQuery,
) -> Result<ExploratoryDeployResponse, Error> {
    match deploy_service_client::DeployServiceClient::connect(host).await {
        Ok(mut client) => {
            let request = Request::new(payload);
            match client.exploratory_deploy(request).await {
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

pub async fn get_event_by_hash_util(
    host: String,
    payload: BlockQuery,
) -> Result<EventInfoResponse, Error> {
    match deploy_service_client::DeployServiceClient::connect(host).await {
        Ok(mut client) => {
            let request = Request::new(payload);
            match client.get_event_by_hash(request).await {
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

pub async fn visualize_dag_util_stream<T>(
    host: String,
    payload: VisualizeDagQuery,
    func: fn(VisualizeBlocksResponse) -> T,
    num: Option<usize>,
) -> Result<(), Error> {
    match deploy_service_client::DeployServiceClient::connect(host).await {
        Ok(mut client) => {
            let request = Request::new(payload);
            match client.visualize_dag(request).await {
                Ok(stream) => {
                    match num {
                        Some(num) => {
                            let mut stream = stream.into_inner().take(num);
                            while let Some(item) = stream.next().await {
                                func(item.unwrap());
                            }
                        }
                        None => {
                            let mut stream = stream.into_inner();
                            while let Some(item) = stream.next().await {
                                func(item.unwrap());
                            }
                        }
                    }

                    Ok(())
                }
                Err(err) => {
                    let err = format!("Error getting stream {}", err);
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

pub async fn show_main_chain_util_stream<T>(
    host: String,
    payload: BlocksQuery,
    func: fn(BlockInfoResponse) -> T,
    num: Option<usize>,
) -> Result<(), Error> {
    match deploy_service_client::DeployServiceClient::connect(host).await {
        Ok(mut client) => {
            let request = Request::new(payload);
            match client.show_main_chain(request).await {
                Ok(stream) => {
                    match num {
                        Some(num) => {
                            let mut stream = stream.into_inner().take(num);
                            while let Some(item) = stream.next().await {
                                func(item.unwrap());
                            }
                        }
                        None => {
                            let mut stream = stream.into_inner();
                            while let Some(item) = stream.next().await {
                                func(item.unwrap());
                            }
                        }
                    }

                    Ok(())
                }
                Err(err) => {
                    let err = format!("Error getting stream {}", err);
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

pub async fn show_blocks_util_stream<T>(
    host: String,
    payload: BlocksQuery,
    func: fn(BlockInfoResponse) -> T,
    num: Option<usize>,
) -> Result<(), Error> {
    match deploy_service_client::DeployServiceClient::connect(host).await {
        Ok(mut client) => {
            let request = Request::new(payload);
            match client.get_blocks(request).await {
                Ok(stream) => {
                    match num {
                        Some(num) => {
                            let mut stream = stream.into_inner().take(num);
                            while let Some(item) = stream.next().await {
                                func(item.unwrap());
                            }
                        }
                        None => {
                            let mut stream = stream.into_inner();
                            while let Some(item) = stream.next().await {
                                func(item.unwrap());
                            }
                        }
                    }

                    Ok(())
                }
                Err(err) => {
                    let err = format!("Error getting stream {}", err);
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

pub async fn get_blocks_by_height_util_stream<T>(
    host: String,
    payload: BlocksQueryByHeight,
    func: fn(BlockInfoResponse) -> T,
    num: Option<usize>,
) -> Result<(), Error> {
    match deploy_service_client::DeployServiceClient::connect(host).await {
        Ok(mut client) => {
            let request = Request::new(payload);
            match client.get_blocks_by_heights(request).await {
                Ok(stream) => {
                    match num {
                        Some(num) => {
                            let mut stream = stream.into_inner().take(num);
                            while let Some(item) = stream.next().await {
                                func(item.unwrap());
                            }
                        }
                        None => {
                            let mut stream = stream.into_inner();
                            while let Some(item) = stream.next().await {
                                func(item.unwrap());
                            }
                        }
                    }

                    Ok(())
                }
                Err(err) => {
                    let err = format!("Error getting stream {}", err);
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
