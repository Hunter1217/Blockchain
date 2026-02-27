// Source: Dr. Pramod Viswanath, Principles of Blockchains (Princeton).
use serde::{Serialize,Deserialize};
use ring::signature::{Ed25519KeyPair, Signature};
use rand::{Rng, RngCore};
use crate::types::address::Address;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Transaction {
    sender: Address,
    receiver: Address,
    value: u64
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct SignedTransaction {
    pub transaction: Transaction,
    pub signature: Vec<u8>,
    pub public_key: Vec<u8>,
}

/// Create digital signature of a transaction
pub fn sign(t: &Transaction, key: &Ed25519KeyPair) -> Signature {
    // serialize the transaction into deterministic bytes
    let msg = bincode::serialize(t).expect("Transaction serialization should not fail");

    // sign the serialized bytes with Ed25519
    key.sign(&msg)
}

/// Verify digital signature of a transaction, using public key instead of secret key
pub fn verify(t: &Transaction, public_key: &[u8], signature: &[u8]) -> bool {
    use ring::signature::{UnparsedPublicKey, ED25519}; // ring helper for verifying from raw bytes

    // serialize transaction the SAME WAY as sign() did
    // Verification only works if message bytes match exactly.
    let msg = match bincode::serialize(t) {
        Ok(m) => m,
        Err(_) => return false, // if serialization fails, treat as invalid
    };

    // build a verifier from the algorithm + raw public key bytes, then verify.
    let pk = UnparsedPublicKey::new(&ED25519, public_key);
    pk.verify(&msg, signature).is_ok()
}

#[cfg(any(test, test_utilities))]
pub fn generate_random_transaction() -> Transaction {
    // generate random sender/receiver (20 bytes each) + random u64 value.
    let mut rng = rand::thread_rng();

    let mut sender_bytes = [0u8; 20];
    let mut receiver_bytes = [0u8; 20];
    rng.fill_bytes(&mut sender_bytes);
    rng.fill_bytes(&mut receiver_bytes);

    Transaction {
        sender: Address::from(sender_bytes),
        receiver: Address::from(receiver_bytes),
        value: rng.gen::<u64>(),
    }
}

// DO NOT CHANGE THIS COMMENT, IT IS FOR AUTOGRADER. BEFORE TEST

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::key_pair;
    use ring::signature::KeyPair;


    #[test]
    fn sign_verify() {
        let t = generate_random_transaction();
        let key = key_pair::random();
        let signature = sign(&t, &key);
        assert!(verify(&t, key.public_key().as_ref(), signature.as_ref()));
    }
    #[test]
    fn sign_verify_two() {
        let t = generate_random_transaction();
        let key = key_pair::random();
        let signature = sign(&t, &key);
        let key_2 = key_pair::random();
        let t_2 = generate_random_transaction();
        assert!(!verify(&t_2, key.public_key().as_ref(), signature.as_ref()));
        assert!(!verify(&t, key_2.public_key().as_ref(), signature.as_ref()));
    }
}
