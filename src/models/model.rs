#[derive(Debug)]
pub struct DeployData {
    pub timestamp: u32,
    pub term: String,
    pub shard_id: String,
    pub phlo_price: u64,
    pub phlo_limit: u32,
    pub valid_after_block_number: i32,
}

#[derive(Debug)]
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

#[derive(Debug)]
pub struct DeployDataReturn {
    pub data: DeployData,
    pub deployer: String,
    pub signture: String,
    pub sig_algorithm: String,
}
