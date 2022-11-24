use core::time::Duration;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DeployData {
    pub timestamp: u32,
    pub term: String,
    pub shard_id: String,
    pub phlo_price: u64,
    pub phlo_limit: u32,
    pub valid_after_block_number: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeployDataPayload {
    pub sig_algorithm: Option<String>,
    pub timestamp: u32,
    pub term: String,
    pub shard_id: String,
    pub phlo_price: u64,
    pub phlo_limit: u32,
    pub valid_after_block_number: i32,
    pub private_key: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeployDataReturn {
    pub data: DeployData,
    pub deployer: String,
    pub signture: String,
    pub sig_algorithm: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EasyDeploy {
    pub term: String,
    pub shard_id: Option<String>,
    pub private_key: String,
    pub phlo_price: u64,
    pub phlo_limit: u32,
    pub timeoute: Option<Duration>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExploreDataOptions {
    pub term: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExploreDeployResponse {
    pub names: Vec<String>,
    pub block_number: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Version {
    pub api: String,
    pub node: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatusRespoonse {
    pub address: String,
    pub version: Version,
    pub peers: u32,
    pub nodes: u32,
    pub min_phlo_price: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BlockOptions {
    pub position: i32,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LightBlockInfo {
    parents_hash_list: Vec<String>, //not sure of this type as the TS has any here
    block_hash: String,
    block_size: String,
    seq_num: i32,
    block_number: i32,
    version: i64,
    deploy_count: i32,
    tuple_space_hash: String,
    timestamp: i64,
    fault_tolerance: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BlockResponse {
    blocks: Vec<LightBlockInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PrepareDeployOptions {
    deployer: String,
    timestamp: i32,
    name_qty: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PrepareDeployResponse {
    names: Vec<String>,
    block_number: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataPayload {
    data: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataAtNameUnforgPrivate {
    unforg_private: DataPayload,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataAtNameUnforgDeploy {
    unforg_deploy: DataPayload,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataAtNameUnforgDeployer {
    unforg_deployer: DataPayload,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataAtNameUnforgPrivateOptions {
    name: DataAtNameUnforgPrivate,
    depth: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataAtNameUnforgDeployOptions {
    name: DataAtNameUnforgDeploy,
    depth: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataAtNameUnforgDeployerOptions {
    name: DataAtNameUnforgDeployer,
    depth: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataAtNameByBlockHashUnforgPrivateOptions {
    name: DataAtNameUnforgPrivate,
    depth: u64,
    block_hash: String,
    use_pre_state_hash: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataAtNameByBlockHashUnforgDeployOptions {
    name: DataAtNameUnforgDeploy,
    depth: u64,
    block_hash: String,
    use_pre_state_hash: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataAtNameByBlockHashUnforgDeployerOptions {
    name: DataAtNameUnforgDeployer,
    depth: u64,
    block_hash: String,
    use_pre_state_hash: bool,
}
