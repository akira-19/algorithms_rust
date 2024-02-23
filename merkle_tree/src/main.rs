use sha2::{Digest, Sha256};
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone)]
struct Node {
    hash: Vec<u8>,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(left: Option<Box<Node>>, right: Option<Box<Node>>, data: Option<&[u8]>) -> Self {
        let mut hasher = Sha256::new();
        match (left.clone(), right.clone()) {
            (Some(l), Some(r)) => {
                hasher.update(&l.hash);
                hasher.update(&r.hash);
            }
            (None, None) => {
                hasher.update(data.expect("Leaf nodes must contain data"));
            }
            _ => panic!("Nodes should either have two children or be leaf nodes with data."),
        }
        let hash = hasher.finalize().to_vec();

        Node { hash, left, right }
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.hash)
    }
}

fn build_merkle_tree(data_blocks: Vec<&[u8]>) -> Node {
    let mut nodes = data_blocks
        .into_iter()
        .map(|data| Node::new(None, None, Some(data)))
        .map(|node| Box::new(node))
        .collect::<Vec<_>>();

    while nodes.len() > 1 {
        let mut new_level = Vec::new();
        for chunk in nodes.chunks(2) {
            if chunk.len() == 2 {
                let node = Node::new(Some(chunk[0].clone()), Some(chunk[1].clone()), None);
                new_level.push(Box::new(node));
            } else {
                // For an odd number of nodes, duplicate the last one
                let node = Node::new(Some(chunk[0].clone()), Some(chunk[0].clone()), None);
                new_level.push(Box::new(node));
            }
        }
        nodes = new_level;
    }

    *nodes[0].clone()
}

fn main() {
    let mut hasher1 = Sha256::new();
    let mut hasher2 = Sha256::new();
    let mut hasher3 = Sha256::new();

    hasher1.update(b"block1");
    hasher2.update(b"block2");
    hasher3.update(b"block3");

    let result1 = hasher1.finalize();
    let result2 = hasher2.finalize();
    let result3 = hasher3.finalize();

    let merkle_root =
        build_merkle_tree([result1.as_slice(), result2.as_slice(), result3.as_slice()].to_vec());
    println!("Merkle Root Hash: {:?}", merkle_root);
}
