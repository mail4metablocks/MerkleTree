use std::collections::HashMap;
use std::hash::{Hash, Hasher};

struct Node<T> {
    // The data stored at this node
    data: T,
    // The left child of this node
    left: Option<Box<Node<T>>>,
    // The right child of this node
    right: Option<Box<Node<T>>>,
    // The hash of this node
    hash: Vec<u8>,
}

impl<T> Node<T>
where
    T: Hash,
{
    // Creates a new node with the given data
    fn new(data: T) -> Self {
        // Hash the data
        let mut s = DefaultHasher::new();
        data.hash(&mut s);
        let hash = s.finish().to_le_bytes().to_vec();

        Self {
            data,
            left: None,
            right: None,
            hash,
        }
    }

    // Recursively creates a merkle tree from the given data
    fn from_vec(data: Vec<T>) -> Option<Self> {
        // Return None if the data vector is empty
        if data.is_empty() {
            return None;
        }
        // If there is only one piece of data, return a single node
        if data.len() == 1 {
            return Some(Self::new(data[0]));
        }

        // Split the data vector into left and right halves
        let mid = data.len() / 2;
        let left_data = data[..mid].to_vec();
        let right_data = data[mid..].to_vec();

        // Recursively create the left and right subtrees
        let mut left = Self::from_vec(left_data);
        let mut right = Self::from_vec(right_data);

        // If either of the subtrees is None, return the other one
        if left.is_none() {
            return right;
        }
        if right.is_none() {
            return left;
        }

        // Create a new node with the left and right subtrees as children
        let mut root = Self::new((left.as_ref().unwrap().data, right.as_ref().unwrap().data));
        root.left = left.take();
        root.right = right.take();
        Some(root)
    }

    // Returns the hash of the root node
    fn root_hash(&self) -> Vec<u8> {
        self.hash.clone()
    }
}

// Define a type alias for a merkle tree
type MerkleTree<T> = Option<Box<Node<T>>>;

fn main() {
    // Create a data vector
    let data = vec![1, 2,
