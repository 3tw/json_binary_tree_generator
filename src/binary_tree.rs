// Binary Tree inspired by Daw-Chih Liou implementation
// Source: https://hackernoon.com/how-to-insert-binary-tree-in-rust

use crate::node::Node;
use std::collections::VecDeque;

#[derive(Debug, PartialEq)]
pub struct BinaryTree {
    pub value: String,
    pub id: u64,
    pub parent: Option<u64>,
    pub left: Option<Box<BinaryTree>>,
    pub right: Option<Box<BinaryTree>>,
}

impl BinaryTree {
    pub fn init(value: String) -> Self {
        BinaryTree {
            value,
            id: 0,
            parent: None,
            left: None,
            right: None,
        }
    }
    pub fn new(value: String, id: u64, parent: u64) -> Self {
        BinaryTree {
            value,
            id: id,
            parent: Some(parent),
            left: None,
            right: None,
        }
    }

    pub fn insert(&mut self, new_value: String) -> &Option<Box<BinaryTree>> {
        let mut queue: VecDeque<&mut BinaryTree> = VecDeque::new();
        queue.push_front(self);

        loop {
            let BinaryTree {
                ref mut id,
                ref mut left,
                ref mut right,
                ..
            } = queue.pop_back().unwrap();

            match left {
                Some(node) => {
                    queue.push_front(node);
                }
                None => {
                    let tree_id = new_value.parse::<u64>().unwrap();
                    *left = Some(Box::new(BinaryTree::new(new_value, tree_id, *id)));
                    return &*left;
                }
            }

            match right {
                Some(node) => {
                    queue.push_front(node);
                }
                None => {
                    let tree_id = new_value.parse::<u64>().unwrap();
                    *right = Some(Box::new(BinaryTree::new(new_value, tree_id, *id)));
                    return &*right;
                }
            }
        }
    }

    pub fn get_node_object(&self) -> Node {
        let parent = match self.parent {
            Some(val) => val,
            _ => 0,
        };
        Node {
            id: self.id,
            parent: parent,
            text: (*self.value).to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_new_tree() {
        let tree = BinaryTree::init("1".to_string());
        assert_eq!(tree.value, "1".to_string());
    }
}
