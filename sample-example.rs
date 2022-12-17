use std::hash::{Hash, Hasher};

// Define a struct to represent a node in the tree
struct Node<T> {
    data: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
    hash: Vec<u8>,
}

impl<T> Node<T>
where
    T: Hash,
{
    // Create a new node with the given data
    fn new(data: T) -> Self {
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

    // Recursively create a merkle tree from the given data
    fn from_vec(data: Vec<T>) -> Option<Self> {
        if data.is_empty() {
            return None;
        }
        if data.len() == 1 {
            return Some(Self::new(data[0]));
        }

        let mid = data.len() / 2;
        let left_data = data[..mid].to_vec();
        let right_data = data[mid..].to_vec();

        let mut left = Self::from_vec(left_data);
        let mut right = Self::from_vec(right_data);

        if left.is_none() {
            return right;
        }
        if right.is_none() {
            return left;
        }

        let mut root = Self::new((left.as_ref().unwrap().data, right.as_ref().unwrap().data));
        root.left = left.take();
        root.right = right.take();
        Some(root)
    }

    // Return the hash of the root node
    fn root_hash(&self) -> Vec<u8> {
        self.hash.clone()
    }
}

fn main() {
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let tree = Node::from_vec(data);
    let root_hash = tree.as_ref().unwrap().root_hash();
    println!("Root hash: {:?}", root_hash);
}
