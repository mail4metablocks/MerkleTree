package main

import (
	"crypto/sha256"
	"encoding/hex"
	"fmt"
)

// Node represents a node in the tree
type Node struct {
	// The data stored at this node
	Data []byte
	// The left child of this node
	Left *Node
	// The right child of this node
	Right *Node
	// The hash of this node
	Hash []byte
}

// NewNode creates a new node with the given data
func NewNode(data []byte) *Node {
	node := &Node{
		Data: data,
		Left: nil,
		Right: nil,
	}
	node.Hash = node.calculateHash()
	return node
}

// FromSlice creates a new merkle tree from the given slice
func FromSlice(data [][]byte) *Node {
	if len(data) == 0 {
		return nil
	}
	if len(data) == 1 {
		return NewNode(data[0])
	}

	mid := len(data) / 2
	leftData := data[:mid]
	rightData := data[mid:]

	left := FromSlice(leftData)
	right := FromSlice(rightData)

	if left == nil {
		return right
	}
	if right == nil {
		return left
	}

	root := NewNode(append(left.Data, right.Data...))
	root.Left = left
	root.Right = right
	return root
}

// RootHash returns the hash of the root node
func (n *Node) RootHash() []byte {
	return n.Hash
}

// calculateHash calculates the hash of the node
func (n *Node) calculateHash() []byte {
	h := sha256.New()
	h.Write(n.Data)
	return h.Sum(nil)
}

func main() {
	data := [][]byte{
		[]byte("hello"),
		[]byte("world"),
		[]byte("this"),
		[]byte("is"),
		[]byte("a"),
		[]byte("merkle"),
		[]byte("tree"),
	}

	tree := FromSlice(data)
	rootHash := tree.RootHash()
	fmt.Printf("Root hash: %s\n", hex.EncodeToString(rootHash))
}
