use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_json;

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
    pub fn new(id: u16) -> serde_json::Result<String> {
        let node = Node {
            id: id.to_string(),
            page: rand::thread_rng().gen_range(0..500),
        };
        serde_json::to_string(&node)
    }
}
impl Edge {
    pub fn new() -> serde_json::Result<String> {
        let edge = Edge {
            id: String::from("krnkei"),
            source: String::from("a"),
            target: String::from("b"),
        };
        serde_json::to_string(&edge)
    }
}
// pub struct JsonData {
//     pub node: String,
//     pub edge: String,
// }

// impl JsonData {
//     pub fn new(node: String, edge: String) -> String {
//         JsonData { node, edge }
//     }
// }
