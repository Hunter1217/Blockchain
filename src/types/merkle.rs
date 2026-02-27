// Source: Dr. Pramod Viswanath, Principles of Blockchains (Princeton).
use super::hash::{Hashable, H256};
use ring::digest; 

/// A Merkle tree.
#[derive(Debug, Default)]
pub struct MerkleTree {
    levels: Vec<Vec<H256>>,
}

impl MerkleTree {
    pub fn new<T>(data: &[T]) -> Self where T: Hashable, {
        // Build the leaf level by hashing each datum.
        let mut current_level: Vec<H256> = data.iter().map(|x| x.hash()).collect();

        // store all levels (including leaf level).
        let mut levels: Vec<Vec<H256>> = Vec::new();

        // Push the leaf level
        levels.push(current_level.clone());

        // iteratively build parent levels until we reach a single root hash
        while current_level.len() > 1 {
            // If odd number of nodes, duplicate the last node (spec requirement).
            if current_level.len() % 2 == 1 {
                let last = *current_level.last().expect("non-empty level");
                current_level.push(last);
            }

            // Build the next parent level
            let mut next_level: Vec<H256> = Vec::with_capacity(current_level.len() / 2);

            // Take pairs and hash their concatenation
            for pair in current_level.chunks(2) {
                let left = pair[0];
                let right = pair[1];

                // Concatenate left||right bytes
                let mut combined = Vec::with_capacity(64);
                combined.extend_from_slice(left.as_ref());
                combined.extend_from_slice(right.as_ref());

                //hash combined bytes using ring SHA256, then convert to H256 
                let d = digest::digest(&digest::SHA256, &combined);
                let parent: H256 = d.into(); 
                next_level.push(parent);
            }

            // Move up one level
            current_level = next_level.clone();
            levels.push(next_level);
        }

        MerkleTree { levels }
    }

   pub fn root(&self) -> H256 {
        // Root was computed in new() and stored as the last level’s only element.
        // levels.last() should be vec![root]
        *self
            .levels
            .last()
            .expect("tree must have at least one level")
            .first()
            .expect("root level must have one hash")
    }

    // Returns the Merkle Proof of data at index i
    // the list of sibling hashes along the path from the leaf to the root
    pub fn proof(&self, mut index: usize) -> Vec<H256> {
        let mut proof: Vec<H256> = Vec::new();

        // We iterate over each level except the root level.
        // levels[0] = leaves, levels[last] = root
        for level in &self.levels[..self.levels.len() - 1] {

            let is_right = index % 2 == 1;
            let sibling_index = if is_right {
                index - 1
            } else {
                // left node’s sibling is index+1 if it exists; otherwise it's itself (duplicate)
                if index + 1 < level.len() {
                    index + 1
                } else {
                    index
                }
            };

            proof.push(level[sibling_index]);

            // Move to the parent index for the next level
            index /= 2;
        }

        proof
    }
}

/// Verify that the datum hash with a vector of proofs will produce the Merkle root. Also need the
/// index of datum and `leaf_size`, the total number of leaves.
pub fn verify(root: &H256, datum: &H256, proof: &[H256], mut index: usize, mut leaf_size: usize) -> bool {
    // Start with the leaf hash
    let mut current = *datum;

    // At each level, combine current hash with its sibling from the proof.
    for sibling in proof {
        // if odd, last leaf was duplicated to make even
        let padded_size = if leaf_size % 2 == 1 { leaf_size + 1 } else { leaf_size };

        // Decide concatenation order based on whether current node is left or right
        let (left, right) = if index % 2 == 0 {
            (current, *sibling)
        } else {
            (*sibling, current)
        };

        // Hash left to right
        let mut combined = Vec::with_capacity(64);
        combined.extend_from_slice(left.as_ref());
        combined.extend_from_slice(right.as_ref());
        
        let d = digest::digest(&digest::SHA256, &combined);
        current = d.into(); 

        // Move up to parent
        index /= 2;

        // Next level size = number of parents from this level
        leaf_size = padded_size / 2;
    }

    //we have the root.
    &current == root
}
// DO NOT CHANGE THIS COMMENT, IT IS FOR AUTOGRADER. BEFORE TEST

#[cfg(test)]
mod tests {
    use crate::types::hash::H256;
    use super::*;

    macro_rules! gen_merkle_tree_data {
        () => {{
            vec![
                (hex!("0a0b0c0d0e0f0e0d0a0b0c0d0e0f0e0d0a0b0c0d0e0f0e0d0a0b0c0d0e0f0e0d")).into(),
                (hex!("0101010101010101010101010101010101010101010101010101010101010202")).into(),
            ]
        }};
    }

    #[test]
    fn merkle_root() {
        let input_data: Vec<H256> = gen_merkle_tree_data!();
        let merkle_tree = MerkleTree::new(&input_data);
        let root = merkle_tree.root();
        assert_eq!(
            root,
            (hex!("6b787718210e0b3b608814e04e61fde06d0df794319a12162f287412df3ec920")).into()
        );
        // "b69566be6e1720872f73651d1851a0eae0060a132cf0f64a0ffaea248de6cba0" is the hash of
        // "0a0b0c0d0e0f0e0d0a0b0c0d0e0f0e0d0a0b0c0d0e0f0e0d0a0b0c0d0e0f0e0d"
        // "965b093a75a75895a351786dd7a188515173f6928a8af8c9baa4dcff268a4f0f" is the hash of
        // "0101010101010101010101010101010101010101010101010101010101010202"
        // "6b787718210e0b3b608814e04e61fde06d0df794319a12162f287412df3ec920" is the hash of
        // the concatenation of these two hashes "b69..." and "965..."
        // notice that the order of these two matters
    }

    #[test]
    fn merkle_proof() {
        let input_data: Vec<H256> = gen_merkle_tree_data!();
        let merkle_tree = MerkleTree::new(&input_data);
        let proof = merkle_tree.proof(0);
        assert_eq!(proof,
                   vec![hex!("965b093a75a75895a351786dd7a188515173f6928a8af8c9baa4dcff268a4f0f").into()]
        );
        // "965b093a75a75895a351786dd7a188515173f6928a8af8c9baa4dcff268a4f0f" is the hash of
        // "0101010101010101010101010101010101010101010101010101010101010202"
    }

    #[test]
    fn merkle_verifying() {
        let input_data: Vec<H256> = gen_merkle_tree_data!();
        let merkle_tree = MerkleTree::new(&input_data);
        let proof = merkle_tree.proof(0);
        assert!(verify(&merkle_tree.root(), &input_data[0].hash(), &proof, 0, input_data.len()));
    }
}
