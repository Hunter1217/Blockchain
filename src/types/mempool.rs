// Source: Dr. Pramod Viswanath, Principles of Blockchains (Princeton).
use super::{
    hash::{Hashable, H256},
    transaction::SignedTransaction,
};

#[derive(Debug, Default, Clone)]
pub struct Mempool {
}

impl Mempool {
    pub fn new() -> Self {
        Self{}
    }

}
