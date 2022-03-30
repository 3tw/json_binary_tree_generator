use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Deserialize, Serialize)]
pub struct Node {
    pub id: String,
    pub page: u8,
}

#[derive(Deserialize, Serialize)]
pub struct Edge {
    pub id: String,
    pub source: String,
    pub target: String,
}

impl Node {
    pub fn new() -> serde_json::Result<String> {
        let node = Node {
            id: String::from("krnkei"),
            page: 10,
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
