// Source: Dr. Pramod Viswanath, Principles of Blockchains (Princeton).
use crate::types::block::Block;
use crate::types::hash::{H256, Hashable};
use std::collections::HashMap;
use ring::digest;

pub struct Blockchain {
    blocks: HashMap<H256, Block>,
    heights: HashMap<H256, u64>,
    tip: H256,
    genesis: H256,
}

impl Blockchain {
    /// Create a new blockchain, only containing the genesis block
    pub fn new() -> Self {
        let genesis_parent = H256::default();

        let genesis_block = crate::types::block::generate_random_block(&genesis_parent);

        let genesis_hash = genesis_block.hash();

        let mut blocks = HashMap::new();
        blocks.insert(genesis_hash, genesis_block);

        let mut heights = HashMap::new();
        heights.insert(genesis_hash, 0);

        Blockchain {
            blocks,
            heights,
            tip: genesis_hash,
            genesis: genesis_hash,
        }
    }

    // Insert a block into blockchain
    pub fn insert(&mut self, block: &Block) {
        let h = block.hash();
        let parent = block.get_parent();

        // compute this block's height as parent's height + 1
        let parent_height = *self
            .heights
            .get(&parent)
            .expect("parent block must exist in this part");
        let height = parent_height + 1;

        // store the block and its height
        self.blocks.insert(h, block.clone());
        self.heights.insert(h, height);

        // update tip if strictly longer chain
        let tip_height = *self.heights.get(&self.tip).expect("tip must exist");
        if height > tip_height {
            self.tip = h;
        }
    }

    // Get the last blocks hash of the longest chain
    pub fn tip(&self) -> H256 {
        self.tip
    }

    // Get all blocks hashes of the longest chain, ordered from genesis to the tip
    pub fn all_blocks_in_longest_chain(&self) -> Vec<H256> {
        // Walk backwards from tip to genesis using parent pointers, then reverse.
        let mut chain: Vec<H256> = Vec::new();
        let mut cur = self.tip;

        loop {
            chain.push(cur);
            if cur == self.genesis {
                break;
            }
            let block = self.blocks.get(&cur).expect("block must exist");
            cur = block.get_parent();
        }

        chain.reverse();
        chain
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::block::generate_random_block;

    #[test]
    fn insert_one() {
        let mut blockchain = Blockchain::new();
        let genesis_hash = blockchain.tip();
        let block = generate_random_block(&genesis_hash);
        blockchain.insert(&block);
        assert_eq!(blockchain.tip(), block.hash());
    }
}