use rhoast_client::{http::Http, models::model::EasyDeploy};
#[tokio::main]
async fn main() {

let client=Http::new("http://localhost:40403");
let k=EasyDeploy{
    term: String::from("new a in {}"),
    shard_id: Some(String::from("root")),
    private_key: String::from("28a5c9ac133b4449ca38e9bdf7cacdce31079ef6b3ac2f0a080af83ecff98b36"),
    phlo_price: Some(100000),
    phlo_price_auto: Some(String::from("auto")),
    phlo_limit: 1,
    timeout: None
};
println!("{:?}", client.easy_deploy(k).await)
    
}
