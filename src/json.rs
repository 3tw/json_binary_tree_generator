use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_json;
use std::cmp::Ordering;

#[derive(Deserialize, Serialize)]
pub struct Node {
    pub id: String,
    pub page: u16,
}

#[derive(Deserialize, Serialize)]
pub struct Edge {
    pub id: String,
    pub source: String,
    pub target: String,
}

impl Node {
    fn new(id: u32) -> Node {
        Node {
            id: id.to_string(),
            page: rand::thread_rng().gen_range(0..500),
        }
    }
}

impl Edge {
    // fn new(source: u32, target: u32) -> serde_json::Result<String> {
    fn new(source: u32, target: u32) -> Edge {
        let source_string = source.to_string();
        let target_string = target.to_string();
        let id = source_string.clone() + "-" + &target_string;
        Edge {
            id: id,
            source: source_string,
            target: target_string,
        }
    }
}

pub fn create_json_content(iterations: u32) -> String {
    let mut last_id: u32 = 0;
    let mut data = Vec::new();
    data.push(String::from("{\"elements\":["));

    for i in 1..iterations {
        let node = Node::new(i);
        let node_json = serde_json::to_string(&node).expect("Couldn't serialize node to json");
        data.push(node_json);
        data.push(String::from(","));
        if last_id > 0 {
            let edge = Edge::new(last_id, i);
            let edge_json = serde_json::to_string(&edge).expect("Couldn't serialize edge to json");
            data.push(edge_json);
            match i.cmp(&iterations) {
                Ordering::Less => data.push(String::from(",")),
                _ => ()
            };
        }
        last_id = i;
    }
    data.push(String::from("]}"));
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
    fn create_new_edge() {
        let edge = Edge::new(10, 20);
        assert_eq!(edge.id, "10-20");
        assert_eq!(edge.source, "10");
        assert_eq!(edge.target, "20");
    }
}
