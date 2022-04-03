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
    pub id: String,
    pub random_number: u16,
}

impl Node {
    fn new(id: u32) -> Node {
        Node {
            id: id.to_string(),
            random_number: rand::thread_rng().gen_range(0..500),
        }
    }
}

impl Stringify for Node {}

pub fn create_json_content(iterations: u32) -> String {
    let mut data = Vec::new();
    data.push(String::from("["));

    for i in 1..iterations {
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
        assert_eq!(node.id, "10")
    }
    #[test]
    fn create_new_node_inequality() {
        let node = Node::new(1000);
        assert_ne!(node.id, "999");
    }
}
