pub mod rho;

#[derive(Debug)]
pub struct Node {
    pub http_url: String,
    pub grpc_url: String,
    pub shard_id: String,
}

#[derive(Debug)]
pub struct Account {
    pub from_acc_name: String,
    pub from_acc_addr: String,
    pub to_acc_name: String,
    pub to_acc_addr: String,
    pub pri_key: String,
    pub amount: u64,
}
