# MerkleTree

A merkle tree, also known as a binary hash tree, is a data structure that allows for efficient and secure verification of the contents of large data sets. It works by dividing the data into blocks, creating a cryptographic hash of each block, and then pairing each hash with its corresponding block of data. The paired hashes are then hashed together, creating a new set of paired hashes. This process is repeated until there is only one hash left, which is called the "root hash" or "merkle root."

The resulting tree-like structure allows for efficient verification of the data by only requiring the root hash and the hashes of the individual blocks that have been modified, rather than the entire dataset. This makes it useful for verifying large amounts of data, such as in a blockchain.

One example of how a merkle tree can be used is in a blockchain where each block contains a list of transactions. The transactions can be hashed and organized into a merkle tree, with the root hash included in the block header. This allows someone who only has the block header to verify that a specific transaction is included in the block without having to download the entire block.

Merkle trees have a number of other applications beyond blockchain, such as in version control systems and file integrity checks. They are an efficient and secure way to verify large amounts of data.


![image](https://user-images.githubusercontent.com/117555665/208235784-3e2ddc8f-04ef-4c9b-b8d9-66f80552e22a.png)
![image](https://user-images.githubusercontent.com/117555665/208235792-14fbffb8-eb28-4e34-80ed-1094a690af58.png)

Each node in the tree represents a hash of the data stored at that node. For example, h0 is the hash of the concatenation of h1 and h2. The hash of the root node, h0, is the root hash of the merkle tree.
