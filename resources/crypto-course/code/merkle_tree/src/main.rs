use sha2::{Digest, Sha256};

/// Simple illustration of how a Merkle tree can be (partially) implemented.
fn main() {
    // Set up leaves
    let leaves = vec![
        b"Leaf1".as_slice(),
        b"Leaf2".as_slice(),
        b"Leaf3".as_slice(),
        b"Leaf4".as_slice(), 
        b"Leaf5".as_slice(), 
    ];
    
    // Create merkle tree
    let tree = MerkleTree::new(leaves);

    println!("Merkle root is: {}", hex::encode(tree.root.hash.clone()));

    // Verify existence of leaf
    let leaf_exists = tree.verify(b"Leaf3".as_slice());

    println!("The outcome of the verification was: {}", leaf_exists);
}

#[allow(dead_code)] 
#[derive(Clone,Debug)]
struct MerkleNode {
    hash: Vec<u8>,
    left: Option<Box<MerkleNode>>,
    right: Option<Box<MerkleNode>>
}

impl MerkleNode {
    fn new(data: &[u8]) -> Self {
        let hash = Sha256::digest(data).to_vec();

        Self {
            hash,
            left: None,
            right: None
        }
    }

    fn combine(left: MerkleNode, right: MerkleNode) -> Self {
        // Hash left and right together
        let mut hasher = Sha256::new();
        hasher.update(&left.hash);
        hasher.update(&right.hash);
        let hash = hasher.finalize().to_vec();

        Self { 
            hash, 
            left: Some(Box::new(left)), 
            right: Some(Box::new(right)), 
        }
    }
    
    fn verify(&self, hash_to_verify: &[u8]) -> bool {
        if self.left.is_none() && self.right.is_none() {
            // Leaf node, check if it is the correct hash
            return self.hash == hash_to_verify;
        }

        // Recursively check child nodes for the correct hash
        let mut left_result = false; 
        let mut right_result = false; 

        if let Some(left) = &self.left {
            left_result = left.verify(hash_to_verify);
        }

        if let Some(right) = &self.right {
            right_result = right.verify(hash_to_verify);
        }

        left_result || right_result
    }
}

#[derive(Clone,Debug)]
struct MerkleTree {
    root: MerkleNode
}

impl MerkleTree {
    fn new(leaves: Vec<&[u8]>) -> Self {
        if leaves.is_empty() {
            panic!("No leaves provided for MerkleTree");
        }

        // Create mutable vec for reuse at each level of the tree
        let mut nodes: Vec<MerkleNode> = leaves.into_iter()
            .map(|leaf| MerkleNode::new(leaf))
            .collect();

        while nodes.len() > 1 {
            let mut next = vec![];

            // Iterate over array 2 by 2
            for chunk in nodes.chunks(2) {
                if chunk.len() == 0 {
                    panic!("Found chunk of length 0");
                } else if chunk.len() == 1 {
                    // Single node, combine with self
                    next.push(MerkleNode::combine(chunk[0].clone(),chunk[0].clone()));
                } else {
                    // Two nodes, hash together
                    next.push(MerkleNode::combine(chunk[0].clone(), chunk[1].clone()));
                }
            }

            // Update nodes for next level
            nodes = next;
        }

        MerkleTree {
            root: nodes.remove(0)
        }
    }

    fn verify(&self, leaf: &[u8]) -> bool {
        let hash = Sha256::digest(leaf).to_vec();
        
        self.root.verify(&hash)
    }
}
