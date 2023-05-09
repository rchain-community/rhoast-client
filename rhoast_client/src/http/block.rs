use super::Http;
use crate::error::Error;
use crate::http::get_method;
use crate::models::model::{BlockInfo, BlockOptions, BlockPostion, LightBlockInfo};

impl Http {
    pub async fn block_call(&self, options: BlockOptions) -> Result<Vec<LightBlockInfo>, Error> {
        let url = format!("{}/api/blocks/{}", &self.host, options.position);
        let req = reqwest::get(url).await;
        get_method::<Vec<LightBlockInfo>>(req, &String::from("Error getting block")).await
    }

    pub async fn latest_block_call(&self) -> Result<Vec<LightBlockInfo>, Error> {
        let url = format!("{}/api/blocks", &self.host);
        let req = reqwest::get(url).await;
        get_method::<Vec<LightBlockInfo>>(req, &String::from("Error getting block")).await
    }

    pub async fn limit_block_call(
        &self,
        options: BlockPostion,
    ) -> Result<Vec<LightBlockInfo>, Error> {
        let url = format!(
            "{}/api/blocks/{}/{}",
            &self.host, options.start, options.end
        );
        let req = reqwest::get(url).await;
        get_method::<Vec<LightBlockInfo>>(req, &String::from("Error getting block")).await
    }

    pub async fn hash_block_call(&self, hash: &String) -> Result<BlockInfo, Error> {
        let url = format!("{}/api/block/{}", &self.host, hash);
        let req = reqwest::get(url).await;
        get_method::<BlockInfo>(req, &String::from("Error getting block")).await
    }

    pub async fn last_finalized_block(&self) -> Result<BlockInfo, Error> {
        let url = format!("{}/api/last-finalized-block", &self.host);
        let req = reqwest::get(url).await;
        get_method::<BlockInfo>(req, &String::from("Error getting block")).await
    }

    pub async fn is_finalized_block(&self, hash: &String) -> Result<bool, Error> {
        let url = format!("{}/api/is-finalized/{}", &self.host, hash);
        let req = reqwest::get(url).await;
        get_method::<bool>(req, &String::from("Error getting block")).await
    }

    pub async fn valid_after_block_number(&self) -> Result<i32, Error> {
        let block = BlockOptions { position: 1 };
        let res = self.block_call(block).await?;
        Ok(res[0].block_number)
    }
}
