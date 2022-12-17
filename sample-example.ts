class Node<T> {
  // The data stored at this node
  data: T;
  // The left child of this node
  left: Node<T> | null;
  // The right child of this node
  right: Node<T> | null;
  // The hash of this node
  hash: number[];

  constructor(data: T) {
    this.data = data;
    this.left = null;
    this.right = null;
    this.hash = this.calculateHash(data);
  }

  // Creates a new node with the given data
  static new<T>(data: T): Node<T> {
    return new Node<T>(data);
  }

  // Recursively creates a merkle tree from the given data
  static fromArray<T>(data: T[]): Node<T> | null {
    if (data.length === 0) {
      return null;
    }
    if (data.length === 1) {
      return Node.new(data[0]);
    }

    const mid = Math.floor(data.length / 2);
    const leftData = data.slice(0, mid);
    const rightData = data.slice(mid);

    const left = Node.fromArray(leftData);
    const right = Node.fromArray(rightData);

    if (!left || !right) {
      return left || right;
    }

    const root = Node.new((left.data, right.data));
    root.left = left;
    root.right = right;
    return root;
  }

  // Returns the hash of the root node
  rootHash(): number[] {
    return this.hash;
  }

  // Calculates the hash of the given data
  private calculateHash(data: T): number[] {
    // You would need to implement a hashing function here
    // For the sake of simplicity, we will just return a random number
    return [Math.floor(Math.random() * 1000)];
  }
}

const data = [1, 2, 3, 4, 5, 6, 7, 8];
const tree = Node.fromArray(data);
const rootHash = tree!.rootHash();
console.log(`Root hash: ${rootHash}`);
