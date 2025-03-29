use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

#[derive(Debug)]
struct HuffmanCode {
    encoding_table: HashMap<char, Vec<bool>>,
}

impl HuffmanCode {
    fn new(encoding_table: HashMap<char, Vec<bool>>) -> Self {
        Self { encoding_table }
    }

    fn encode(&self, data: &str) -> Vec<bool> {
        Vec::new()
    }

    fn decode(&self, data: &[bool]) -> String {
        String::new()
    }

    fn serialize(&self) -> Vec<u8> {
        Vec::new()
    }

    fn deserialize(data: &[u8]) -> Self {
        Self { encoding_table: HashMap::new() }
    }
}

#[derive(Debug)]
enum Node {
    Leaf {
        value: char,
        count: i32,
    },
    Internal {
        left: Box<Node>,
        right: Box<Node>,
        weight: i32,
    },
}

impl Node {
    fn is_leaf(&self) -> bool {
        matches!(self, Node::Leaf { .. })
    }

    fn weight(&self) -> i32 {
        match self {
            Node::Leaf { count, .. } => *count,
            Node::Internal { weight, .. } => *weight,
        }
    }
}

#[derive(Debug)]
struct HuffmanTree {
    root: Box<Node>,
}

impl HuffmanTree {

    pub fn build_encoding_table(&self) -> HashMap<char, Vec<bool>> {
        let mut encoding_table = HashMap::new();
        self.walk_through_tree(&self.root, Vec::new(), &mut encoding_table);
        encoding_table
    }

    fn walk_through_tree(
        &self,
        node: &Box<Node>,
        current_path: Vec<bool>,
        table: &mut HashMap<char, Vec<bool>>,
    ) {
        match node.as_ref() {
            Node::Leaf { value, .. } => {
                table.insert(*value, current_path);
            }
            Node::Internal { left, right, .. } => {
                let mut left_path = current_path.clone();
                left_path.push(false);
                self.walk_through_tree(left, left_path, table);

                let mut right_path = current_path;
                right_path.push(true); 
                self.walk_through_tree(right, right_path, table);
            }
        }
    }

    fn new_leaf(value: char, count: i32) -> HuffmanTree {
        HuffmanTree {
            root: Box::new(Node::Leaf { value, count }),
        }
    }

    fn new_internal(left: Box<Node>, right: Box<Node>) -> HuffmanTree {
        let weight = left.weight() + right.weight();
        HuffmanTree {
            root: Box::new(Node::Internal {
                left,
                right,
                weight,
            }),
        }
    }

    fn weight(&self) -> i32 {
        self.root.weight()
    }

    fn build_tree(frequencies: &HashMap<char, i32>) -> HuffmanTree {
        let mut heap = BinaryHeap::new();

        for (&c, &count) in frequencies {
            heap.push(Reverse(HuffmanTree::new_leaf(c, count)));
        }

        while heap.len() > 1 {
            if let (Some(Reverse(left)), Some(Reverse(right))) = (heap.pop(), heap.pop()) {
                let combined = HuffmanTree::new_internal(left.root, right.root);
                heap.push(Reverse(combined));
            } else {
                panic!("Heap should contain at least two elements")
            }
        }

        if let Some(Reverse(tree)) = heap.pop() {
            tree
        } else {
            panic!("Heap should not be empty")
        }
    }
}

impl PartialEq for HuffmanTree {
    fn eq(&self, other: &Self) -> bool {
        self.weight() == other.weight()
    }
}

impl Eq for HuffmanTree {}

impl PartialOrd for HuffmanTree {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HuffmanTree {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.weight().cmp(&other.weight())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_leaf_node() {
        let leaf = HuffmanTree::new_leaf('a', 5);
        assert_eq!(leaf.weight(), 5);
    }

    #[test]
    fn test_internal_node() {
        let left = HuffmanTree::new_leaf('a', 3);
        let right = HuffmanTree::new_leaf('b', 2);
        let internal = HuffmanTree::new_internal(left.root, right.root);
        assert_eq!(internal.weight(), 5);
    }

    #[test]
    fn test_tree_comparison() {
        let tree1 = HuffmanTree::new_leaf('a', 3);
        let tree2 = HuffmanTree::new_leaf('b', 2);
        let tree3 = HuffmanTree::new_leaf('c', 4);

        assert!(tree1 > tree2);
        assert!(tree2 < tree3);
        assert_eq!(tree1, HuffmanTree::new_leaf('x', 3)); // Same weight, different char
    }

    #[test]
    fn test_deep_tree() {
        let leaf1 = HuffmanTree::new_leaf('a', 1);
        let leaf2 = HuffmanTree::new_leaf('b', 2);
        let leaf3 = HuffmanTree::new_leaf('c', 3);

        let internal1 = HuffmanTree::new_internal(leaf1.root, leaf2.root);
        let final_tree = HuffmanTree::new_internal(internal1.root, leaf3.root);

        assert_eq!(final_tree.weight(), 6);
    }

    #[test]
    fn test_build_tree() {
        let mut frequencies = HashMap::new();
        frequencies.insert('a', 4);
        frequencies.insert('b', 2);
        frequencies.insert('c', 1);
        frequencies.insert('d', 5);

        let tree = HuffmanTree::build_tree(&frequencies);
        assert_eq!(tree.weight(), 12);

        match tree.root.as_ref() {
            Node::Internal {
                weight,
                left,
                right,
            } => {
                assert_eq!(*weight, 12);
                assert!(left.is_leaf());
                assert!(!right.is_leaf());

                match left.as_ref() {
                    Node::Leaf { value, count } => {
                        assert_eq!(*value, 'd');
                        assert_eq!(*count, 5);
                    }
                    _ => panic!("Expected leaf node"),
                }
            }
            _ => panic!("Expected internal node"),
        }
    }

    #[test]
    fn test_create_encoding_table() {
        let mut frequencies = HashMap::new();
        frequencies.insert('a', 4);
        frequencies.insert('b', 2);
        frequencies.insert('c', 1);
        frequencies.insert('d', 5);

        let tree = HuffmanTree::build_tree(&frequencies);
        let encoding_table = tree.build_encoding_table();

        assert_eq!(encoding_table.len(), 4);
        assert!(encoding_table.contains_key(&'a'));
        assert!(encoding_table.contains_key(&'b')); 
        assert!(encoding_table.contains_key(&'c'));
        assert!(encoding_table.contains_key(&'d'));

        assert_eq!(encoding_table[&'d'], vec![false]);
        assert_eq!(encoding_table[&'a'], vec![true, true]);
        assert_eq!(encoding_table[&'b'], vec![true, false, true]);
        assert_eq!(encoding_table[&'c'], vec![true, false, false]);
    }
}
