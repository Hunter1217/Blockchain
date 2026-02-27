// Source: Dr. Pramod Viswanath, Principles of Blockchains (Princeton).

use serde::{Serialize, Deserialize};
use rand::Rng;
use std::time::{SystemTime, UNIX_EPOCH};
use ring::digest;

use crate::types::hash::{H256, Hashable};
use crate::types::transaction::SignedTransaction;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Header {
    pub parent: H256,
    pub nonce: u32,
    pub difficulty: H256,
    pub timestamp: u128, 
    pub merkle_root: H256,
}

// Content is the block body (transactions).
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Content {
    pub transactions: Vec<SignedTransaction>,
}

// A Block = Header + Content
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub header: Header,
    pub content: Content,
}

// needed so we can build a Merkle tree over transactions.
impl Hashable for SignedTransaction {
    fn hash(&self) -> H256 {
        let bytes = bincode::serialize(self).expect("SignedTransaction serialization should not fail");
        let d = digest::digest(&digest::SHA256, &bytes);
        H256::from(d)
    }
}

impl Hashable for Header {
    fn hash(&self) -> H256 {
        let bytes = bincode::serialize(self).expect("Header serialization should not fail");
        let d = digest::digest(&digest::SHA256, &bytes);
        H256::from(d)
    }
}

impl Hashable for Block {
    fn hash(&self) -> H256 {
        // hash header only, not content
        self.header.hash()
    }
}

impl Block {
    pub fn get_parent(&self) -> H256 {
        self.header.parent
    }

    pub fn get_difficulty(&self) -> H256 {
        self.header.difficulty
    }
}

#[cfg(any(test, test_utilities))]
pub fn generate_random_block(parent: &H256) -> Block {
    let mut rng = rand::thread_rng();

    let content = Content { transactions: vec![] };

    // Merkle root of empty input
    let merkle_root = H256::default();

    let header = Header {
        parent: *parent,
        nonce: rng.gen::<u32>(),
        difficulty: H256::default(),
        timestamp: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time went backwards")
            .as_millis(),
        merkle_root,
    };

    Block { header, content }
}