use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_json;

pub trait Stringify: serde::Serialize {
    fn to_json(&self) -> String {
        match serde_json::to_string(self) {
            Ok(json_string) => json_string,
            Err(_e) => String::from("Couldn't serialize edge to json"),
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct Node {
    pub key: u32,
    pub parent: u64,
    pub text: String,
}

impl Node {
    fn new(id: u32) -> Node {
        let num: f64 = rand::thread_rng().gen_range(0.0..1.0);
        Node {
            key: id,
            parent: if id > 0 {
                (num * id as f64 / 2.0).floor() as u64
            } else {
                0
            },
            text: rand::thread_rng().gen_range(0..500).to_string(),
        }
    }
}

impl Stringify for Node {}

pub fn create_json_content(iterations: u32) -> String {
    let mut data = Vec::new();
    data.push(String::from("["));

    for i in 0..iterations {
        let node = Node::new(i);
        let node_json = node.to_json();
        data.push(node_json);
        data.push(String::from(","));
    }

    // Replave last char: ','
    let data_last_index = data.len() - 1;
    data[data_last_index] = String::from("]");
    data.into_iter().map(String::from).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_new_node() {
        let node = Node::new(10);
        assert_eq!(node.key, 10)
    }
    #[test]
    fn create_new_node_inequality() {
        let node = Node::new(1000);
        assert_ne!(node.key, 999);
    }
}
