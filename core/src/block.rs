use utils::coder;
use chrono::prelude::*;
use serde::{Serialize};

// BlockHeader
// 区块头
#[derive(Debug, Serialize)]
pub struct BlockHeader {
    pub time: i64,
    pub tx_hash: String,
    pub pre_hash: String,
}

// Block
// 区块
pub struct Block {
    pub header: BlockHeader,
    pub hash: String,
    pub data: String,
}

impl Block {
    pub fn new_block(data: String, pre_hash: String) -> Block {
        let transaction_bytes = coder::my_serialize(&data);
        let tx_hash = coder::get_hash(&transaction_bytes[..]);

        let time = Utc::now().timestamp();

        let mut block = Block {
            header: BlockHeader {
                time: time,
                tx_hash: tx_hash,  //transactions data merkle root hash
                pre_hash: pre_hash,
            },
            hash: "".to_string(),
            data: data,
        };

        block.set_hash();
        block
    }

    fn set_hash(&mut self) {
        let header = coder::my_serialize(&(self.header));
        self.hash = coder::get_hash(&header[..]);
    }
}