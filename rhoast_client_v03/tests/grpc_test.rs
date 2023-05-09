use futures::Future;
use rhoast_client_v03::grpc::Grpc;
use rhoast_client_v03::proto::casper::*;

const KEY: &str = "URL";
const PORT: &str = "PORT";

async fn check_env<F, Fut>(input: F)
where
    F: FnOnce(Grpc) -> Fut,
    Fut: Future<Output = ()>,
{
    let value = std::env::var(KEY);
    match value {
        Ok(url) => {
            let port = std::env::var(PORT).unwrap();
            let url_port = format!("{url}:{port}");

            let grpc = Grpc::new(&url_port);
            input(grpc).await
        }
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
    check_env(test_prpopose_result_util).await;
}

async fn test_do_deploy_util(grpc: Grpc) {
    let payload = DeployDataProto {
        deployer: "Q2FsaWZvcm5pYSBQbGFzdGlj".as_bytes().to_vec(),
        term: "Multi-tiered".to_string(),
        timestamp: 9709,
        sig: "secp256k1".as_bytes().to_vec(),
        sig_algorithm: "secp256k1".to_string(),
        shard_id: "shard".to_string(),
        phlo_price: 510,
        phlo_limit: 968,
        valid_after_block_number: 446,
    };

    let deploy = grpc.do_deploy_util(payload).await.unwrap();
    assert!(deploy.message.is_some());
}

async fn test_get_block_util(grpc: Grpc) {
    let payload = BlockQuery {
        hash: "hash_value".to_string(),
    };
    let block = grpc.get_block_util(payload).await.unwrap();
    assert!(block.message.is_some());
}

async fn test_machine_verification_dag_util(grpc: Grpc) {
    let payload = MachineVerifyQuery {};
    let dag = grpc.machine_verification_dag_util(payload).await.unwrap();
    assert!(dag.message.is_some());
}

async fn test_find_deploy_util(grpc: Grpc) {
    let payload = FindDeployQuery {
        deploy_id: "Q2FsaWZvcm5pYSBQbGFzdGlj".as_bytes().to_vec(),
    };
    let deploy = grpc.find_deploy_util(payload).await.unwrap();
    assert!(deploy.message.is_some());
}

async fn test_prview_private_names_util(grpc: Grpc) {
    let payload = PrivateNamePreviewQuery {
        user: "Q2FsaWZvcm5pYSBQbGFzdGlj".as_bytes().to_vec(),
        timestamp: 148,
        name_qty: 164,
    };
    let ids = grpc.prview_private_names_util(payload).await.unwrap();
    assert!(ids.message.is_some());
}

async fn test_last_finalized_block_util(grpc: Grpc) {
    let payload = LastFinalizedBlockQuery {};

    let block = grpc.last_finalized_block_util(payload).await.unwrap();
    assert!(block.message.is_some());
}

async fn test_is_finalized_util(grpc: Grpc) {
    let payload = IsFinalizedQuery {
        hash: "hash".to_string(),
    };
    let finalized = grpc.is_finalized_util(payload).await.unwrap();
    assert!(finalized.message.is_some());
}

async fn test_bond_status_util(grpc: Grpc) {
    let payload = BondStatusQuery {
        public_key: "key".as_bytes().to_vec(),
    };
    let bond = grpc.bond_status_util(payload).await.unwrap();
    assert!(bond.message.is_some());
}

async fn test_exploratory_deploy_util(grpc: Grpc) {
    let payload = ExploratoryDeployQuery {
        term: "term".to_string(),
        block_hash: "hash".to_string(),
        use_pre_state_hash: false,
    };

    let explore = grpc.exploratory_deploy_util(payload).await.unwrap();
    assert!(explore.message.is_some());
}

async fn test_get_event_by_hash_util(grpc: Grpc) {
    let payload = ReportQuery {
        hash: "hash".to_string(),
        force_replay: false,
    };

    let explore = grpc.get_event_by_hash_util(payload).await.unwrap();
    assert!(explore.message.is_some());
}

async fn test_prpopose_util(grpc: Grpc) {
    let propose = grpc.propose(true).await.unwrap();

    assert!(propose.message.is_some())
}

async fn test_prpopose_result_util(grpc: Grpc) {
    let payload = ProposeResultQuery {};
    let propose = grpc.propose_result(payload).await.unwrap();

    assert!(propose.message.is_some())
}
