use core::time::Duration;
use serde::{Deserialize, Serialize};

pub trait HttpModel {}

impl HttpModel for StatusRespoonse {}
impl HttpModel for DeployDataRequest {}
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
impl HttpModel for Bond {}
impl HttpModel for String {}
impl HttpModel for bool {}
impl HttpModel for Vec<LightBlockInfo> {}
impl HttpModel for BlockInfo {}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeployDataRequest {
    pub data: DeployData,
    pub sig_algorithm: String,
    pub signature: String,
    pub deployer: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeployData {
    pub timestamp: i64,
    pub term: String,
    pub shard_id: String,
    pub phlo_price: u64,
    pub phlo_limit: u32,
    pub valid_after_block_number: i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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
#[serde(rename_all = "camelCase")]
pub struct DeployDataReturn {
    pub data: DeployData,
    pub deployer: String,
    pub signture: String,
    pub sig_algorithm: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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
#[serde(rename_all = "camelCase")]
pub struct DeployResponse {
    pub names: Vec<String>,
    pub block_number: u64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExploreDataOptions {
    pub term: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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
#[serde(rename_all = "camelCase")]
pub struct StatusRespoonse {
    pub address: String,
    pub version: Version,
    pub peers: u32,
    pub nodes: u32,
    pub min_phlo_price: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BlockOptions {
    pub position: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BlockPostion {
    pub start: i32,
    pub end: i32,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LightBlockInfo {
    pub parents_hash_list: Vec<String>,
    pub block_hash: String,
    pub sig: String,
    pub sig_algorithm: String,
    pub shard_id: String,
    pub extra_bytes: String,
    pub sender: String,
    pub block_size: String,
    pub seq_num: i32,
    pub block_number: i32,
    pub version: i64,
    pub deploy_count: i32,
    pub tuple_space_hash: Option<String>,
    pub timestamp: i64,
    pub fault_tolerance: f32,
    pub pre_state_hash: String,
    pub post_state_hash: String,
    pub bonds: Vec<Bond>,
    pub justifications: Vec<ValidatorHash>,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidatorHash {
    pub validator: String,
    pub latest_block_hash: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Bond {
    pub validator: String,
    pub stake: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockInfo {
    pub block_info: LightBlockInfo,
    pub deploys: Vec<BlockDeploys>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockDeploys {
    pub deployer: String,
    pub term: String,
    pub timestamp: i64,
    pub sig: String,
    pub sig_algorithm: String,
    pub phlo_price: u64,
    pub phlo_limit: i64,
    pub valid_after_block_number: u32,
    pub cost: u64,
    pub errored: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BlockResponse {
    pub blocks: Vec<LightBlockInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrepareDeployOptions {
    pub deployer: String,
    pub timestamp: i32,
    pub name_qty: i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrepareDeployResponse {
    pub names: Vec<String>,
    pub seq_number: i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataPayload {
    pub data: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataAtNameUnforgPrivate {
    pub unforg_private: DataPayload,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataAtNameUnforgDeploy {
    pub unforg_deploy: DataPayload,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataAtNameUnforgDeployer {
    pub unforg_deployer: DataPayload,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataAtNameUnforgPrivateOptions {
    pub name: DataAtNameUnforgPrivate,
    pub depth: u64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataAtNameUnforgDeployOptions {
    pub name: DataAtNameUnforgDeploy,
    pub depth: u64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataAtNameUnforgDeployerOptions {
    pub name: DataAtNameUnforgDeployer,
    pub depth: u64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataAtNameByBlockHashUnforgPrivateOptions {
    pub name: DataAtNameUnforgPrivate,
    pub depth: u64,
    pub block_hash: String,
    pub use_pre_state_hash: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataAtNameByBlockHashUnforgDeployOptions {
    pub name: DataAtNameUnforgDeploy,
    pub depth: u64,
    pub block_hash: String,
    pub use_pre_state_hash: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataAtNameByBlockHashUnforgDeployerOptions {
    pub name: DataAtNameUnforgDeployer,
    pub depth: u64,
    pub block_hash: String,
    pub use_pre_state_hash: bool,
}
