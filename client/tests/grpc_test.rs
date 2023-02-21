use client::grpc::deploy::*;
use client::grpc::propose::propose_util;
use client::proto::casper::*;

const ENDPOINT: &str = "http://167.235.8.107:40401";
const PROPOSE_ENDPOINT: &str = "http://167.235.8.107:40402";

#[tokio::test]
async fn test_do_deploy_util() {
    let payload = DeployDataProto {
        deployer: "Q2FsaWZvcm5pYSBQbGFzdGlj".as_bytes().to_vec(),
        term: "Multi-tiered".to_string(),
        timestamp: 9709,
        sig: "secp256k1".as_bytes().to_vec(),
        sig_algorithm: "secp256k1".to_string(),
        phlo_price: 510,
        phlo_limit: 968,
        valid_after_block_number: 446,
    };
    let deploy = do_deploy_util(ENDPOINT.to_string(), payload).await.unwrap();
    assert!(deploy.message.is_some());
}

#[tokio::test]
async fn test_get_block_util() {
    let payload = BlockQuery {
        hash: "hash_value".to_string(),
    };
    let block = get_block_util(ENDPOINT.to_string(), payload).await.unwrap();
    assert!(block.message.is_some());
}

#[tokio::test]
async fn test_machine_verification_dag_util() {
    let payload = MachineVerifyQuery {};
    let dag = machine_verification_dag_util(ENDPOINT.to_string(), payload)
        .await
        .unwrap();
    assert!(dag.message.is_some());
}

#[tokio::test]
async fn test_find_deploy_util() {
    let payload = FindDeployQuery {
        deploy_id: "Q2FsaWZvcm5pYSBQbGFzdGlj".as_bytes().to_vec(),
    };
    let deploy = find_deploy_util(ENDPOINT.to_string(), payload)
        .await
        .unwrap();
    assert!(deploy.message.is_some());
}

#[tokio::test]
async fn test_prview_private_names_util() {
    let payload = PrivateNamePreviewQuery {
        user: "Q2FsaWZvcm5pYSBQbGFzdGlj".as_bytes().to_vec(),
        timestamp: 148,
        name_qty: 164,
    };
    let ids = prview_private_names_util(ENDPOINT.to_string(), payload)
        .await
        .unwrap();
    assert!(ids.message.is_some());
}

#[tokio::test]
async fn test_last_finalized_block_util() {
    let payload = LastFinalizedBlockQuery {};
    let block = last_finalized_block_util(ENDPOINT.to_string(), payload)
        .await
        .unwrap();
    assert!(block.message.is_some());
}

#[tokio::test]
async fn test_is_finalized_util() {
    let payload = IsFinalizedQuery {
        hash: "hash".to_string(),
    };
    let finalized = is_finalized_util(ENDPOINT.to_string(), payload)
        .await
        .unwrap();
    assert!(finalized.message.is_some());
}

#[tokio::test]
async fn test_bond_status_util() {
    let payload = BondStatusQuery {
        public_key: "key".as_bytes().to_vec(),
    };
    let bond = bond_status_util(ENDPOINT.to_string(), payload)
        .await
        .unwrap();
    assert!(bond.message.is_some());
}

#[tokio::test]
async fn test_exploratory_deploy_util() {
    let payload = ExploratoryDeployQuery {
        term: "term".to_string(),
        block_hash: "hash".to_string(),
        use_pre_state_hash: false,
    };
    let explore = exploratory_deploy_util(ENDPOINT.to_string(), payload)
        .await
        .unwrap();
    assert!(explore.message.is_some());
}

#[tokio::test]
async fn test_get_event_by_hash_util() {
    let payload = BlockQuery {
        hash: "hash".to_string(),
    };
    let explore = get_event_by_hash_util(ENDPOINT.to_string(), payload)
        .await
        .unwrap();
    assert!(explore.message.is_some());
}

#[tokio::test]
async fn test_prpopose_util() {
    let propose = propose_util(PROPOSE_ENDPOINT.to_string(), true)
        .await
        .unwrap();

    assert!(propose.message.is_some())
}
