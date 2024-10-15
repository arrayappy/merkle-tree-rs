use std::fmt::{Debug, Display};

use sha2::{Digest, Sha256};

#[derive(Clone, Debug)]
struct MerkelNode {
    data: String,
    left: Option<Box<MerkelNode>>,
    right: Option<Box<MerkelNode>>,
}

impl MerkelNode {
    fn new(data: String) -> Self {
        return MerkelNode {
            data,
            left: None,
            right: None,
        };
    }
}

type MerkelNodePair = (MerkelNode, MerkelNode);

#[derive(Debug)]
struct MerkleTree {
    root: Box<MerkelNode>,
}

fn sha256_hash<T: AsRef<[u8]>>(input: T) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input);
    let result = hasher.finalize();
    format!("{:x}", result)
}

impl MerkleTree {
    fn new() -> Self {
        return MerkleTree {
            root: Box::new(MerkelNode::new("dummy".to_owned())),
        };
    }
    fn create<T: AsRef<[u8]> + Clone + Display + Debug>(&mut self, data: &mut Vec<T>) {
        let mut leaves: Vec<String> = vec![];
        if data.len() % 2 != 0 {
            data.push(data[data.len() - 1].clone());
        }

        for i in data.iter() {
            let hash = sha256_hash(i);
            leaves.push(hash);
        }

        let mut merkle_node_pairs: Vec<MerkelNodePair> = vec![];

        let mut i = 0;
        while i < leaves.len() - 1 {
            let pair = (
                MerkelNode::new(leaves[i].clone()),
                MerkelNode::new(leaves[i + 1].clone()),
            );
            merkle_node_pairs.push(pair);
            i += 2;
        }

        self.build_merkle_tree(merkle_node_pairs);
    }

    fn build_merkle_tree(&mut self, merkle_pairs: Vec<MerkelNodePair>) {
        let mut output_nodes: Vec<MerkelNode> = vec![];

        for i in merkle_pairs {
            let mut concat_string = String::from(i.0.data.clone());
            concat_string.push_str(&i.1.data);

            let hash = sha256_hash(concat_string);
            let mut new_node = MerkelNode::new(hash);

            new_node.left = Some(Box::new(i.0));
            new_node.right = Some(Box::new(i.1));

            output_nodes.push(new_node);
        }

        if output_nodes.len() == 1 {
            self.root = Box::new(output_nodes.last().unwrap().clone());
            return;
        }

        if output_nodes.len() % 2 != 0 {
            output_nodes.push(output_nodes.last().unwrap().to_owned());
        }
        let mut output_pairs: Vec<MerkelNodePair> = vec![];

        let mut i = 0;
        while i < output_nodes.len() - 1 {
            let pair = (output_nodes[i].clone(), output_nodes[i + 1].clone());
            output_pairs.push(pair);
            i += 2;
        }

        return self.build_merkle_tree(output_pairs);
    }
}

fn print_tree(root: Option<Box<MerkelNode>>) {
    fn print_tree_helper(node: &Option<Box<MerkelNode>>, prefix: String, is_left: bool) {
        if let Some(ref node) = node {
            println!(
                "{}{}{}",
                prefix,
                if is_left { "├──" } else { "└──" },
                reduce_string(node.data.clone())
            );

            let new_prefix = format!("{}{}", prefix, if is_left { "│   " } else { "    " });
            print_tree_helper(&node.left, new_prefix.clone(), true);
            print_tree_helper(&node.right, new_prefix, false);
        }
    }

    print_tree_helper(&root, String::new(), false);
}

fn reduce_string(input: String) -> String {
    let mut output = String::new();
    output.push_str(input[0..3].as_ref());
    output.push_str("...");
    output.push_str(input[input.len() - 4..input.len()].as_ref());

    return output;
}
fn main() {
    let mut merkle_tree = MerkleTree::new();

    let mut data = vec![
        "a".to_owned(),
        "b".to_owned(),
        "c".to_owned(),
        "d".to_owned(),
        "e".to_owned(),
        "f".to_owned(),
        "g".to_owned(),
        "h".to_owned(),
    ];

    merkle_tree.create(&mut data);

    print_tree(Some(merkle_tree.root));
}