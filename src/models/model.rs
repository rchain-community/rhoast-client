use core::time::Duration;
use serde::{Deserialize, Serialize};
pub trait HttpModel {}

impl HttpModel for StatusRespoonse {}
impl HttpModel for DeployData {}
impl HttpModel for DeployDataPayload {}
impl HttpModel for DeployDataReturn {}
impl HttpModel for EasyDeploy {}
impl HttpModel for DeployResponse {}
impl HttpModel for ExploreDataOptions {}
impl HttpModel for ExploreDeployResponse {}
impl HttpModel for Version {}
impl HttpModel for BlockOptions {}
impl HttpModel for LightBlockInfo {}
impl HttpModel for BlockResponse {}
impl HttpModel for PrepareDeployOptions {}
impl HttpModel for PrepareDeployResponse {}
impl HttpModel for DataPayload {}
impl HttpModel for DataAtNameUnforgDeployerOptions {}
impl HttpModel for DataAtNameUnforgDeployOptions {}
impl HttpModel for DataAtNameUnforgPrivateOptions {}
impl HttpModel for DataAtNameByBlockHashUnforgPrivateOptions {}
impl HttpModel for DataAtNameByBlockHashUnforgDeployOptions {}
impl HttpModel for DataAtNameByBlockHashUnforgDeployerOptions {}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeployData {
    pub timestamp: i64,
    pub term: String,
    pub shard_id: String,
    pub phlo_price: u64,
    pub phlo_limit: u32,
    pub valid_after_block_number: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeployDataPayload {
    pub sig_algorithm: String,
    pub timestamp: i64,
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
    pub phlo_price: Option<u64>,
    pub phlo_price_auto: Option<String>,
    pub phlo_limit: u32,
    pub timeout: Option<Duration>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeployResponse {
    pub names: Vec<String>,
    pub block_number: u64,
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
    pub parents_hash_list: Vec<String>, //not sure of this type as the TS has any here
    pub block_hash: String,
    pub block_size: String,
    pub seq_num: i32,
    pub block_number: i32,
    pub version: i64,
    pub deploy_count: i32,
    pub tuple_space_hash: String,
    pub timestamp: i64,
    pub fault_tolerance: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BlockResponse {
    pub blocks: Vec<LightBlockInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PrepareDeployOptions {
    pub deployer: String,
    pub timestamp: i32,
    pub name_qty: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PrepareDeployResponse {
    pub names: Vec<String>,
    pub block_number: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataPayload {
    pub data: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataAtNameUnforgPrivate {
    pub unforg_private: DataPayload,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataAtNameUnforgDeploy {
    pub unforg_deploy: DataPayload,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataAtNameUnforgDeployer {
    pub unforg_deployer: DataPayload,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataAtNameUnforgPrivateOptions {
    pub name: DataAtNameUnforgPrivate,
    pub depth: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataAtNameUnforgDeployOptions {
    pub name: DataAtNameUnforgDeploy,
    pub depth: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataAtNameUnforgDeployerOptions {
    pub name: DataAtNameUnforgDeployer,
    pub depth: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataAtNameByBlockHashUnforgPrivateOptions {
    pub name: DataAtNameUnforgPrivate,
    pub depth: u64,
    pub block_hash: String,
    pub use_pre_state_hash: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataAtNameByBlockHashUnforgDeployOptions {
    pub name: DataAtNameUnforgDeploy,
    pub depth: u64,
    pub block_hash: String,
    pub use_pre_state_hash: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataAtNameByBlockHashUnforgDeployerOptions {
    pub name: DataAtNameUnforgDeployer,
    pub depth: u64,
    pub block_hash: String,
    pub use_pre_state_hash: bool,
}
