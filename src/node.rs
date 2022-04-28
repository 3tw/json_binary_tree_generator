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
    pub id: u64,
    pub parent: u64,
    pub text: String,
}
impl Stringify for Node {}
