use futures::Future;
use rhoast_client::grpc::deploy::*;
use rhoast_client::grpc::propose::propose_util;
use rhoast_client::proto::casper::*;

const OTHER_PORT: &str = "40401";
const PROPOSE_PORT: &str = "40402";
const KEY: &str = "URL";

async fn check_env<F, Fut>(input: F)
where
    F: FnOnce() -> Fut,
    Fut: Future<Output = ()>,
{
    let value = std::env::var(KEY);
    match value {
        Ok(_url) => input().await,
        Err(_e) => {
            println!("no url provided, skipping test")
        }
    }
}

#[tokio::test]
async fn exec_all_test() {
    check_env(test_do_deploy_util).await;
    check_env(test_get_block_util).await;
    check_env(test_machine_verification_dag_util).await;
    check_env(test_find_deploy_util).await;
    check_env(test_prview_private_names_util).await;
    check_env(test_last_finalized_block_util).await;
    check_env(test_is_finalized_util).await;
    check_env(test_bond_status_util).await;
    check_env(test_exploratory_deploy_util).await;
    check_env(test_get_event_by_hash_util).await;
    check_env(test_prpopose_util).await;
}

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
    let url = std::env::var(KEY).unwrap();
    let url_port = format!("{url}:{OTHER_PORT}");
    let deploy = do_deploy_util(url_port, payload).await.unwrap();
    assert!(deploy.message.is_some());
}

async fn test_get_block_util() {
    let payload = BlockQuery {
        hash: "hash_value".to_string(),
    };
    let url = std::env::var(KEY).unwrap();
    let url_port = format!("{url}:{OTHER_PORT}");
    let block = get_block_util(url_port, payload).await.unwrap();
    assert!(block.message.is_some());
}

async fn test_machine_verification_dag_util() {
    let payload = MachineVerifyQuery {};
    let url = std::env::var(KEY).unwrap();
    let url_port = format!("{url}:{OTHER_PORT}");
    let dag = machine_verification_dag_util(url_port, payload)
        .await
        .unwrap();
    assert!(dag.message.is_some());
}

async fn test_find_deploy_util() {
    let payload = FindDeployQuery {
        deploy_id: "Q2FsaWZvcm5pYSBQbGFzdGlj".as_bytes().to_vec(),
    };
    let url = std::env::var(KEY).unwrap();
    let url_port = format!("{url}:{OTHER_PORT}");

    let deploy = find_deploy_util(url_port, payload).await.unwrap();
    assert!(deploy.message.is_some());
}

async fn test_prview_private_names_util() {
    let payload = PrivateNamePreviewQuery {
        user: "Q2FsaWZvcm5pYSBQbGFzdGlj".as_bytes().to_vec(),
        timestamp: 148,
        name_qty: 164,
    };
    let url = std::env::var(KEY).unwrap();
    let url_port = format!("{url}:{OTHER_PORT}");
    let ids = prview_private_names_util(url_port, payload).await.unwrap();
    assert!(ids.message.is_some());
}

async fn test_last_finalized_block_util() {
    let payload = LastFinalizedBlockQuery {};
    let url = std::env::var(KEY).unwrap();
    let url_port = format!("{url}:{OTHER_PORT}");

    let block = last_finalized_block_util(url_port, payload).await.unwrap();
    assert!(block.message.is_some());
}

async fn test_is_finalized_util() {
    let payload = IsFinalizedQuery {
        hash: "hash".to_string(),
    };
    let url = std::env::var(KEY).unwrap();
    let url_port = format!("{url}:{OTHER_PORT}");

    let finalized = is_finalized_util(url_port, payload).await.unwrap();
    assert!(finalized.message.is_some());
}

async fn test_bond_status_util() {
    let payload = BondStatusQuery {
        public_key: "key".as_bytes().to_vec(),
    };
    let url = std::env::var(KEY).unwrap();
    let url_port = format!("{url}:{OTHER_PORT}");

    let bond = bond_status_util(url_port, payload).await.unwrap();
    assert!(bond.message.is_some());
}

async fn test_exploratory_deploy_util() {
    let payload = ExploratoryDeployQuery {
        term: "term".to_string(),
        block_hash: "hash".to_string(),
        use_pre_state_hash: false,
    };
    let url = std::env::var(KEY).unwrap();
    let url_port = format!("{url}:{OTHER_PORT}");

    let explore = exploratory_deploy_util(url_port, payload).await.unwrap();
    assert!(explore.message.is_some());
}

async fn test_get_event_by_hash_util() {
    let payload = BlockQuery {
        hash: "hash".to_string(),
    };
    let url = std::env::var(KEY).unwrap();
    let url_port = format!("{url}:{OTHER_PORT}");

    let explore = get_event_by_hash_util(url_port, payload).await.unwrap();
    assert!(explore.message.is_some());
}

async fn test_prpopose_util() {
    let url = std::env::var(KEY).unwrap();
    let url_port = format!("{url}:{PROPOSE_PORT}");

    let propose = propose_util(url_port, true).await.unwrap();

    assert!(propose.message.is_some())
}
