use crate::proto::{casper::Par, deploy::DataWithBlockInfo};

pub fn get_first_block(block_info: &Vec<DataWithBlockInfo>) -> &DataWithBlockInfo {
    &block_info[0]
}

pub fn get_value_from_block(block_info: &Vec<DataWithBlockInfo>) -> Option<&Par> {
    for i in 0..block_info.len() {
        let block = &block_info[i];
        if block.post_block_data.len() > 0 {
            for j in 0..block.post_block_data.len() {
                let data = &block.post_block_data[j];
                return Some(data);
            }
        }
    }
    None
}
